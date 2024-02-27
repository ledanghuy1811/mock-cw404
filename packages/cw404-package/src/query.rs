use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct TokenInfoResponse {
    pub name: String, 
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
    pub admin: Addr,
    pub units: Uint128
}