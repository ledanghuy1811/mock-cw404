use cosmwasm_std::{Deps, StdResult};
use cw20::BalanceResponse;

use cw404_package::TokenInfoResponse;

use crate::state::{BALANCES, TOKEN_INFO};

pub fn query_balance(deps: Deps, address: String) -> StdResult<BalanceResponse> {
    let address = deps.api.addr_validate(&address)?;
    let balance = BALANCES
        .may_load(deps.storage, &address)?
        .unwrap_or_default();

    Ok(BalanceResponse { balance })
}

pub fn query_token_info(deps: Deps) -> StdResult<TokenInfoResponse> {
    let info = TOKEN_INFO.load(deps.storage)?;
    let resp = TokenInfoResponse {
        name: info.name,
        symbol: info.symbol,
        decimals: info.decimals,
        total_supply: info.total_supply,
        admin: info.admin,
        units: info.units,
    };

    Ok(resp)
}
