use cosmwasm_std::{Deps, StdResult};
use cw20::BalanceResponse;
use cw721::NumTokensResponse;

use cw404_package::{MaxNftSupplyRespone, TokenInfoResponse};

use crate::state::{BALANCES, MAX_NFT_SUPPLY, NFT_COUNT, TOKEN_INFO};

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

pub fn query_nft_num_token(deps: Deps) -> StdResult<NumTokensResponse> {
    let nft_count = NFT_COUNT.load(deps.storage)?;

    Ok(NumTokensResponse { count: nft_count })
}

pub fn query_max_nft_supply(deps: Deps) -> StdResult<MaxNftSupplyRespone> {
    let max_nft_supply = MAX_NFT_SUPPLY.load(deps.storage)?;

    Ok(MaxNftSupplyRespone {
        max: max_nft_supply,
    })
}
