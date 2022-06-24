use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::route::SeiRoute;
use cosmwasm_std::{CustomQuery, Decimal, Uint64};

/// SeiQueryWrapper is an override of QueryRequest::Custom to access Sei-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SeiQueryWrapper {
    pub route: SeiRoute,
    pub query_data: SeiQuery,
}

// implement custom query
impl CustomQuery for SeiQueryWrapper {}

/// SeiQuery is defines available query datas
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SeiQuery {
    ExchangeRates {},
    ContractInfo {
        contract_address: String,
    },
}

/// ExchangeRateItem is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OracleExchangeRate {
    pub exchange_rate: Decimal,
    pub last_update: Uint64,
}

/// ExchangeRateItem is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DenomOracleExchangeRatePair {
    pub denom: String,
    pub oracle_exchange_rate: OracleExchangeRate,
}

/// ExchangeRatesResponse is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExchangeRatesResponse {
    pub denom_oracle_exchange_rate_pairs: Vec<DenomOracleExchangeRatePair>,
}

/// ContractInfoResponse is data format returned from WasmRequest::ContractInfo query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInfoResponse {
    pub address: String,
    pub creator: String,
    pub code_id: u64,
    pub admin: Option<String>,
}
