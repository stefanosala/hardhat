mod config;

use std::sync::Arc;

use edr_eth::remote::jsonrpc;
use edr_provider::InvalidRequestReason;
use napi::{tokio::runtime, Env, JsFunction, JsObject, Status};
use napi_derive::napi;

use self::config::ProviderConfig;
use crate::{
    context::EdrContext,
    logger::{Logger, LoggerConfig, LoggerError},
    subscribe::SubscriberCallback,
    trace::RawTrace,
};

/// A JSON-RPC provider for Ethereum.
#[napi]
pub struct Provider {
    provider: Arc<edr_provider::Provider<LoggerError>>,
}

#[napi]
impl Provider {
    #[doc = "Constructs a new provider with the provided configuration."]
    #[napi(ts_return_type = "Promise<Provider>")]
    pub fn with_config(
        env: Env,
        // We take the context as argument to ensure that tracing is initialized properly.
        _context: &EdrContext,
        config: ProviderConfig,
        logger_config: LoggerConfig,
        #[napi(ts_arg_type = "(event: SubscriptionEvent) => void")] subscriber_callback: JsFunction,
    ) -> napi::Result<JsObject> {
        let config = edr_provider::ProviderConfig::try_from(config)?;
        let runtime = runtime::Handle::current();

        let logger = Box::new(Logger::new(&env, logger_config)?);
        let subscriber_callback = SubscriberCallback::new(&env, subscriber_callback)?;
        let subscriber_callback = Box::new(move |event| subscriber_callback.call(event));

        let (deferred, promise) = env.create_deferred()?;
        runtime.clone().spawn_blocking(move || {
            let result = edr_provider::Provider::new(runtime, logger, subscriber_callback, config)
                .map_or_else(
                    |error| Err(napi::Error::new(Status::GenericFailure, error.to_string())),
                    |provider| {
                        Ok(Provider {
                            provider: Arc::new(provider),
                        })
                    },
                );

            deferred.resolve(|_env| result);
        });

        Ok(promise)
    }

    #[doc = "Handles a JSON-RPC request and returns a JSON-RPC response."]
    #[napi]
    pub async fn handle_request(&self, json_request: String) -> napi::Result<Response> {
        let provider = self.provider.clone();
        let request = match serde_json::from_str(&json_request) {
            Ok(request) => request,
            Err(error) => {
                let message = error.to_string();
                let reason = InvalidRequestReason::new(&json_request, &message);

                // HACK: We need to log failed deserialization attempts when they concern input
                // validation.
                if let Some((method_name, provider_error)) = reason.provider_error() {
                    // Ignore potential failure of logging, as returning the original error is more
                    // important
                    let _result = runtime::Handle::current()
                        .spawn_blocking(move || {
                            provider.log_failed_deserialization(&method_name, &provider_error)
                        })
                        .await
                        .map_err(|error| {
                            napi::Error::new(Status::GenericFailure, error.to_string())
                        })?;
                }

                let data = serde_json::from_str(&json_request).ok();
                let response = jsonrpc::ResponseData::<()>::Error {
                    error: jsonrpc::Error {
                        code: reason.error_code(),
                        message: reason.error_message(),
                        data,
                    },
                };

                return serde_json::to_string(&response)
                    .map_err(|error| {
                        napi::Error::new(
                            Status::InvalidArg,
                            format!("Invalid JSON `{json_request}` due to: {error}"),
                        )
                    })
                    .map(|json_response| Response {
                        solidity_trace: None,
                        json: json_response,
                        traces: Vec::new(),
                    });
            }
        };

        let mut response = runtime::Handle::current()
            .spawn_blocking(move || provider.handle_request(request))
            .await
            .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))?;

        // We can take the solidity trace as it won't be used for anything else
        let solidity_trace = response.as_mut().err().and_then(|error| {
            if let edr_provider::ProviderError::TransactionFailed(failure) = error {
                if matches!(
                    failure.failure.reason,
                    edr_provider::TransactionFailureReason::OutOfGas(_)
                ) {
                    None
                } else {
                    Some(Arc::new(std::mem::take(
                        &mut failure.failure.solidity_trace,
                    )))
                }
            } else {
                None
            }
        });

        // We can take the traces as they won't be used for anything else
        let traces = match &mut response {
            Ok(response) => std::mem::take(&mut response.traces),
            Err(edr_provider::ProviderError::TransactionFailed(failure)) => {
                std::mem::take(&mut failure.traces)
            }
            Err(_) => Vec::new(),
        };

        let response = jsonrpc::ResponseData::from(response.map(|response| response.result));

        serde_json::to_string(&response)
            .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
            .map(|json_response| Response {
                solidity_trace,
                json: json_response,
                traces: traces.into_iter().map(Arc::new).collect(),
            })
    }
}

#[napi]
pub struct Response {
    json: String,
    /// When a transaction fails to execute, the provider returns a trace of the
    /// transaction.
    solidity_trace: Option<Arc<edr_evm::trace::Trace>>,
    /// This may contain zero or more traces, depending on the (batch) request
    traces: Vec<Arc<edr_evm::trace::Trace>>,
}

#[napi]
impl Response {
    #[napi(getter)]
    pub fn json(&self) -> String {
        self.json.clone()
    }

    #[napi(getter)]
    pub fn solidity_trace(&self) -> Option<RawTrace> {
        self.solidity_trace
            .as_ref()
            .map(|trace| RawTrace::new(trace.clone()))
    }

    #[napi(getter)]
    pub fn traces(&self) -> Vec<RawTrace> {
        self.traces
            .iter()
            .map(|trace| RawTrace::new(trace.clone()))
            .collect()
    }
}
