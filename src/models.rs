use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    address: Option<String>,
    name: Option<String>,
    symbol: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    buys: u64,
    sells: u64,
}

#[derive(Deserialize, Serialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    m5: OrderBook,
    h1: OrderBook,
    h6: OrderBook,
    h24: OrderBook,
}

#[derive(Deserialize, Serialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeChange {
    m5: Decimal,
    h1: Decimal,
    h6: Decimal,
    h24: Decimal,
}

#[derive(Deserialize, Serialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Liquidity {
    usd: Option<Decimal>,
    base: Decimal,
    #[serde(rename = "quote")]
    qoute: Decimal,
}

#[derive(Deserialize, Serialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    pub chain_id: String,
    pub dex_id: String,
    pub url: String,
    pub pair_address: String,
    pub base_token: Token,
    #[serde(rename = "quoteToken")]
    pub qoute_token: Token,
    pub price_native: Decimal,
    pub price_usd: Option<Decimal>,
    pub volume: TimeChange,
    pub price_change: TimeChange,
    pub liquidity: Liquidity,
}

#[derive(Deserialize, Serialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pairs {
    pub pairs: Vec<Pair>,
}
