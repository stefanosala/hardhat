mod common;

use edr_eth::{
    remote::{
        eth::CallRequest,
        filter::{LogFilterOptions, LogOutput, OneOrMore},
        BlockSpec, BlockTag, PreEip1898BlockSpec,
    },
    transaction::EthTransactionRequest,
    Address, Bytes, B256, U256, U64,
};
use edr_evm::alloy_primitives::U160;
use edr_provider::{MethodInvocation, OneUsizeOrTwo, U64OrUsize};

use crate::common::{
    help_test_method_invocation_serde, help_test_method_invocation_serde_with_expected,
};

#[test]
fn test_serde_eth_accounts() {
    help_test_method_invocation_serde(MethodInvocation::Accounts(()));
}

#[test]
fn test_serde_eth_block_number() {
    help_test_method_invocation_serde(MethodInvocation::BlockNumber(()));
}

#[test]
fn test_serde_eth_call() {
    let tx = CallRequest {
        from: Some(Address::from(U160::from(1))),
        to: Some(Address::from(U160::from(2))),
        gas: Some(3),
        gas_price: Some(U256::from(4)),
        max_fee_per_gas: None,
        max_priority_fee_per_gas: None,
        value: Some(U256::from(123568919)),
        data: Some(Bytes::from(&b"whatever"[..])),
        access_list: None,
        transaction_type: None,
    };
    help_test_method_invocation_serde(MethodInvocation::Call(
        tx.clone(),
        Some(BlockSpec::latest()),
        None,
    ));
    help_test_method_invocation_serde_with_expected(
        MethodInvocation::Call(tx.clone(), None, None),
        MethodInvocation::Call(tx, Some(BlockSpec::latest()), None),
    );
}

#[test]
fn test_serde_eth_chain_id() {
    help_test_method_invocation_serde(MethodInvocation::ChainId(()));
}

#[test]
fn test_serde_eth_coinbase() {
    help_test_method_invocation_serde(MethodInvocation::Coinbase(()));
}

#[test]
fn test_serde_eth_estimate_gas() {
    let tx = CallRequest {
        from: Some(Address::from(U160::from(1))),
        to: Some(Address::from(U160::from(2))),
        gas: Some(3),
        gas_price: Some(U256::from(4)),
        max_fee_per_gas: None,
        max_priority_fee_per_gas: None,
        value: Some(U256::from(123568919)),
        data: Some(Bytes::from(&b"whatever"[..])),
        access_list: None,
        transaction_type: None,
    };
    help_test_method_invocation_serde(MethodInvocation::EstimateGas(
        tx.clone(),
        Some(BlockSpec::latest()),
    ));
    help_test_method_invocation_serde_with_expected(
        MethodInvocation::EstimateGas(tx.clone(), None),
        MethodInvocation::EstimateGas(tx, Some(BlockSpec::pending())),
    );
}

#[test]
fn test_serde_eth_fee_history() {
    help_test_method_invocation_serde(MethodInvocation::FeeHistory(
        U256::from(3),
        BlockSpec::Number(100),
        Some(vec![0.5_f64, 10_f64, 80_f64, 90_f64, 99.5_f64]),
    ));
}

#[test]
fn test_serde_eth_gas_price() {
    help_test_method_invocation_serde(MethodInvocation::GasPrice(()));
}

#[test]
fn test_serde_eth_get_balance() {
    help_test_method_invocation_serde(MethodInvocation::GetBalance(
        Address::from(U160::from(1)),
        Some(BlockSpec::latest()),
    ));
    help_test_method_invocation_serde_with_expected(
        MethodInvocation::GetBalance(Address::from(U160::from(1)), None),
        MethodInvocation::GetBalance(Address::from(U160::from(1)), Some(BlockSpec::latest())),
    );
}

#[test]
fn test_serde_eth_get_block_by_number() {
    help_test_method_invocation_serde(MethodInvocation::GetBlockByNumber(
        PreEip1898BlockSpec::Number(100),
        true,
    ));
}

#[test]
fn test_serde_eth_get_block_by_tag() {
    help_test_method_invocation_serde(MethodInvocation::GetBlockByNumber(
        PreEip1898BlockSpec::latest(),
        true,
    ));
}

#[test]
fn test_serde_eth_get_block_by_hash() {
    help_test_method_invocation_serde(MethodInvocation::GetBlockByHash(
        B256::from(U256::from(1)),
        true,
    ));
}

#[test]
fn test_serde_eth_get_transaction_count() {
    help_test_method_invocation_serde(MethodInvocation::GetTransactionCount(
        Address::from(U160::from(1)),
        Some(BlockSpec::latest()),
    ));
    help_test_method_invocation_serde_with_expected(
        MethodInvocation::GetTransactionCount(Address::from(U160::from(1)), None),
        MethodInvocation::GetTransactionCount(
            Address::from(U160::from(1)),
            Some(BlockSpec::latest()),
        ),
    );
}

#[test]
fn test_serde_eth_get_transaction() {
    help_test_method_invocation_serde(MethodInvocation::GetBlockTransactionCountByHash(
        B256::from(U256::from(1)),
    ));
}

#[test]
fn test_serde_eth_get_transaction_count_by_number() {
    help_test_method_invocation_serde(MethodInvocation::GetBlockTransactionCountByNumber(
        PreEip1898BlockSpec::Number(100),
    ));
}

#[test]
fn test_serde_eth_get_code() {
    help_test_method_invocation_serde(MethodInvocation::GetCode(
        Address::from(U160::from(1)),
        Some(BlockSpec::latest()),
    ));
    help_test_method_invocation_serde_with_expected(
        MethodInvocation::GetCode(Address::from(U160::from(1)), None),
        MethodInvocation::GetCode(Address::from(U160::from(1)), Some(BlockSpec::latest())),
    );
}

#[test]
fn test_serde_eth_get_filter_changes() {
    help_test_method_invocation_serde(MethodInvocation::GetFilterChanges(U256::from(100)));
}

#[test]
fn test_serde_eth_get_filter_logs() {
    help_test_method_invocation_serde(MethodInvocation::GetFilterLogs(U256::from(100)));
}

#[test]
fn test_serde_eth_get_logs_by_block_numbers() {
    help_test_method_invocation_serde(MethodInvocation::GetLogs(LogFilterOptions {
        from_block: Some(BlockSpec::Number(100)),
        to_block: Some(BlockSpec::Number(102)),
        block_hash: None,
        address: Some(OneOrMore::One(Address::from(U160::from(1)))),
        topics: None,
    }));
}

#[test]
fn test_serde_eth_get_logs_by_block_tags() {
    help_test_method_invocation_serde(MethodInvocation::GetLogs(LogFilterOptions {
        from_block: Some(BlockSpec::Tag(BlockTag::Safe)),
        to_block: Some(BlockSpec::latest()),
        block_hash: None,
        address: Some(OneOrMore::One(Address::from(U160::from(1)))),
        topics: Some(vec![Some(OneOrMore::One(B256::from(U256::from(1))))]),
    }));
}

#[test]
fn test_serde_eth_get_logs_by_block_hash() {
    help_test_method_invocation_serde(MethodInvocation::GetLogs(LogFilterOptions {
        from_block: None,
        to_block: None,
        block_hash: Some(B256::from(U256::from(1))),
        address: Some(OneOrMore::One(Address::from(U160::from(1)))),
        topics: Some(vec![Some(OneOrMore::One(B256::from(U256::from(1))))]),
    }));
}

#[test]
fn test_serde_eth_get_storage_at() {
    help_test_method_invocation_serde(MethodInvocation::GetStorageAt(
        Address::from(U160::from(1)),
        U256::ZERO,
        Some(BlockSpec::latest()),
    ));
    help_test_method_invocation_serde_with_expected(
        MethodInvocation::GetStorageAt(Address::from(U160::from(1)), U256::ZERO, None),
        MethodInvocation::GetStorageAt(
            Address::from(U160::from(1)),
            U256::ZERO,
            Some(BlockSpec::latest()),
        ),
    );
}

#[test]
fn test_serde_eth_get_tx_by_block_hash_and_index() {
    help_test_method_invocation_serde(MethodInvocation::GetTransactionByBlockHashAndIndex(
        B256::from(U256::from(1)),
        U256::from(1),
    ));
}

#[test]
fn test_serde_eth_get_tx_by_block_number_and_index() {
    help_test_method_invocation_serde(MethodInvocation::GetTransactionByBlockNumberAndIndex(
        PreEip1898BlockSpec::Number(100),
        U256::from(1),
    ));
}

#[test]
fn test_serde_eth_get_tx_by_hash() {
    help_test_method_invocation_serde(MethodInvocation::GetTransactionByHash(B256::from(
        U256::from(1),
    )));
}

#[test]
fn test_serde_eth_get_tx_count_by_block_number() {
    help_test_method_invocation_serde(MethodInvocation::GetTransactionCount(
        Address::from(U160::from(1)),
        Some(BlockSpec::Number(100)),
    ));
}

#[test]
fn test_serde_eth_get_tx_count_by_block_tag() {
    help_test_method_invocation_serde(MethodInvocation::GetTransactionCount(
        Address::from(U160::from(1)),
        Some(BlockSpec::latest()),
    ));
}

#[test]
fn test_serde_eth_get_tx_receipt() {
    help_test_method_invocation_serde(MethodInvocation::GetTransactionReceipt(B256::from(
        U256::from(1),
    )));
}

#[test]
fn test_serde_eth_mining() {
    help_test_method_invocation_serde(MethodInvocation::Mining(()));
}

#[test]
fn test_serde_eth_new_block_filter() {
    help_test_method_invocation_serde(MethodInvocation::NewBlockFilter(()));
}

#[test]
fn test_serde_eth_new_filter() {
    help_test_method_invocation_serde(MethodInvocation::NewFilter(LogFilterOptions {
        from_block: Some(BlockSpec::Number(1000)),
        to_block: Some(BlockSpec::latest()),
        block_hash: None,
        address: Some(OneOrMore::One(Address::from(U160::from(1)))),
        topics: Some(vec![Some(OneOrMore::One(B256::from(U256::from(1))))]),
    }));
}

#[test]
fn test_serde_eth_new_pending_transaction_filter() {
    help_test_method_invocation_serde(MethodInvocation::NewPendingTransactionFilter(()));
}

#[test]
fn test_serde_eth_pending_transactions() {
    help_test_method_invocation_serde(MethodInvocation::PendingTransactions(()));
}

#[test]
fn test_serde_eth_send_raw_transaction() {
    help_test_method_invocation_serde(MethodInvocation::SendRawTransaction(Bytes::from(
        &b"whatever"[..],
    )));
}

#[test]
fn test_serde_eth_send_transaction() {
    help_test_method_invocation_serde(MethodInvocation::SendTransaction(EthTransactionRequest {
        from: Address::from(U160::from(1)),
        to: Some(Address::from(U160::from(2))),
        gas: Some(3_u64),
        gas_price: Some(U256::from(4)),
        max_fee_per_gas: None,
        value: Some(U256::from(123568919)),
        data: Some(Bytes::from(&b"whatever"[..])),
        nonce: None,
        chain_id: None,
        access_list: None,
        max_priority_fee_per_gas: None,
        transaction_type: None,
    }));
}

#[test]
fn test_serde_eth_sign() {
    help_test_method_invocation_serde(MethodInvocation::Sign(
        Bytes::from(&b"whatever"[..]),
        Address::from(U160::from(1)),
    ));
}

macro_rules! impl_serde_eth_subscribe_tests {
    ($(
        $name:ident => $variant:expr,
    )+) => {
        $(
            paste::item! {
                #[test]
                fn [<test_serde_eth_subscribe_ $name _without_filter>]() {
                    use edr_eth::remote::filter::SubscriptionType;

                    help_test_method_invocation_serde(MethodInvocation::Subscribe($variant, None));
                }

                #[test]
                fn [<test_serde_eth_subscribe_ $name _with_filter>]() {
                    use edr_eth::remote::filter::SubscriptionType;

                    help_test_method_invocation_serde(MethodInvocation::Subscribe($variant, Some(LogFilterOptions {
                        from_block: Some(BlockSpec::Number(1000)),
                        to_block: Some(BlockSpec::latest()),
                        block_hash: None,
                        address: Some(OneOrMore::One(Address::from(U160::from(1)))),
                        topics: Some(vec![Some(OneOrMore::One(B256::from(U256::from(1))))]),
                    })));
                }
            }
        )+
    };
}

impl_serde_eth_subscribe_tests! {
    logs => SubscriptionType::Logs,
    new_pending_transactions => SubscriptionType::NewPendingTransactions,
    new_heads => SubscriptionType::NewHeads,
}

#[test]
fn test_serde_eth_syncing() {
    help_test_method_invocation_serde(MethodInvocation::Syncing(()));
}

#[test]
fn test_serde_eth_uninstall_filter() {
    help_test_method_invocation_serde(MethodInvocation::UninstallFilter(U256::from(100)));
}

#[test]
fn test_serde_eth_unsubscribe() {
    help_test_method_invocation_serde(MethodInvocation::Unsubscribe(U256::from(100)));
}

fn help_test_serde_value<T>(value: T)
where
    T: PartialEq + std::fmt::Debug + serde::de::DeserializeOwned + serde::Serialize,
{
    let serialized = serde_json::json!(value).to_string();

    let deserialized: T = serde_json::from_str(&serialized)
        .unwrap_or_else(|_| panic!("should have successfully deserialized json {serialized}"));

    assert_eq!(value, deserialized);
}

#[test]
fn test_serde_log_output() {
    help_test_serde_value(LogOutput {
        removed: false,
        log_index: Some(0),
        transaction_index: Some(99),
        transaction_hash: Some(B256::from(U256::from(1))),
        block_hash: Some(B256::from(U256::from(2))),
        block_number: Some(0),
        address: Address::from(U160::from(1)),
        data: Bytes::from_static(b"whatever"),
        topics: vec![B256::from(U256::from(3)), B256::from(U256::from(3))],
    });
}

#[test]
fn test_serde_one_or_more_addresses() {
    help_test_serde_value(OneOrMore::One(Address::from(U160::from(1))));
    help_test_serde_value(OneOrMore::Many(vec![
        Address::from(U160::from(1)),
        Address::from(U160::from(1)),
    ]));
}

#[test]
fn test_evm_increase_time() {
    help_test_method_invocation_serde(MethodInvocation::EvmIncreaseTime(U64OrUsize::U64(
        U64::from(12345),
    )));
}

#[test]
fn test_evm_mine() {
    help_test_method_invocation_serde(MethodInvocation::EvmMine(Some(U64OrUsize::U64(U64::from(
        12345,
    )))));
    help_test_method_invocation_serde(MethodInvocation::EvmMine(Some(U64OrUsize::Usize(12345))));
    help_test_method_invocation_serde(MethodInvocation::EvmMine(None));
}

#[test]
fn test_evm_set_next_block_timestamp() {
    help_test_method_invocation_serde(MethodInvocation::EvmSetNextBlockTimestamp(U64OrUsize::U64(
        U64::from(12345),
    )));
}

#[test]
fn test_serde_web3_client_version() {
    help_test_method_invocation_serde(MethodInvocation::Web3ClientVersion(()));
}

#[test]
fn test_serde_web3_sha3() {
    help_test_method_invocation_serde(MethodInvocation::Web3Sha3(Bytes::from(&b"whatever"[..])));
}

#[test]
fn test_evm_set_automine() {
    help_test_method_invocation_serde(MethodInvocation::EvmSetAutomine(false));
}

#[test]
fn test_evm_set_interval_mining() {
    help_test_method_invocation_serde(MethodInvocation::EvmSetIntervalMining(OneUsizeOrTwo::One(
        1000,
    )));
    help_test_method_invocation_serde(MethodInvocation::EvmSetIntervalMining(OneUsizeOrTwo::Two(
        [1000, 5000],
    )));
}

#[test]
fn test_evm_snapshot() {
    help_test_method_invocation_serde(MethodInvocation::EvmSnapshot(()));
}

#[test]
fn test_net_listening() {
    help_test_method_invocation_serde(MethodInvocation::NetListening(()));
}

#[test]
fn test_net_peer_count() {
    help_test_method_invocation_serde(MethodInvocation::NetPeerCount(()));
}

#[test]
fn test_personal_sign() {
    let call = MethodInvocation::Sign(Bytes::from(&b"whatever"[..]), Address::from(U160::from(1)));

    let serialized = serde_json::json!(call)
        .to_string()
        .replace("eth_sign", "personal_sign");

    let call_deserialized: MethodInvocation = serde_json::from_str(&serialized)
        .unwrap_or_else(|_| panic!("should have successfully deserialized json {serialized}"));

    assert_eq!(call, call_deserialized);
}
