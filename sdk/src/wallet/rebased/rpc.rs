// From https://github.com/iotaledger/iota/blob/develop/crates/iota-json-rpc-api/src/coin.rs
//

// use iota_json_rpc_types::{Balance, CoinPage, IotaCirculatingSupply, IotaCoinMetadata};
// use iota_open_rpc_macros::open_rpc;
// use iota_types::{
//     balance::Supply,
//     base_types::{IotaAddress, ObjectID},
// };
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use serde::Deserialize;
use serde_with::serde_as;

use super::ObjectID;
use super::bigint::BigInt;
use super::types::IotaAddress;

/// Provides access to coin-related data such as coins owned by an address,
/// balances, or metadata.
#[rpc(client, namespace = "iotax")]
pub trait CoinReadApi {
    // /// Return all Coin<`coin_type`> objects owned by an address.
    #[rustfmt::skip]
    #[method(name = "getCoins")]
    async fn get_coins(
        &self,
        // the owner's IOTA address
        owner: IotaAddress,
        // optional type name for the coin (e.g., 0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC), default to 0x2::iota::IOTA if not specified.
        coin_type: Option<String>,
        // optional paging cursor
        cursor: Option<ObjectID>,
        // maximum number of items per page
        limit: Option<usize>,
    ) -> RpcResult<CoinPage>;

    /// Return the total coin balance for one coin type, owned by the address owner.
    #[rustfmt::skip]
    #[method(name = "getBalance")]
    async fn get_balance(
        &self,
        // the owner's IOTA address
        owner: IotaAddress,
        // optional type names for the coin (e.g., 0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC), default to 0x2::iota::IOTA if not specified.
        coin_type: Option<String>,
    ) -> RpcResult<Balance>;
}

/// `next_cursor` points to the last item in the page;
/// Reading with `next_cursor` will start from the next item after `next_cursor`
/// if `next_cursor` is `Some`, otherwise it will start from the first item.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Page<T, C> {
    pub data: Vec<T>,
    pub next_cursor: Option<C>,
    pub has_next_page: bool,
}

pub type CoinPage = Page<Coin, ObjectID>;

#[serde_as]
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
pub struct SequenceNumber(#[serde_as(as = "BigInt<u64>")] u64);

#[serde_as]
#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Coin {
    pub coin_type: String,
    pub coin_object_id: ObjectID,
    // #[serde_as(as = "AsSequenceNumber")]
    pub version: SequenceNumber,
    // pub digest: ObjectDigest,
    #[serde_as(as = "BigInt<u64>")]
    pub balance: u64,
    // pub previous_transaction: TransactionDigest,
}

#[serde_as]
#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub coin_type: String,
    pub coin_object_count: usize,
    #[serde_as(as = "BigInt<u128>")]
    pub total_balance: u128,
}
