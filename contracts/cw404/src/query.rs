use cosmwasm_std::{BlockInfo, Deps, Env, StdResult};
use cw20::BalanceResponse;
use cw721::{NumTokensResponse, OwnerOfResponse};

use cw404_package::{Cw721TransferExemptResponse, MaxNftSupplyRespone, TokenInfoResponse};

use crate::state::{
    Approval, NftInfo, BALANCES, CW721_TRANSFER_EXEMPT, MAX_NFT_SUPPLY, NFT_COUNT, NFT_TOKENS,
    TOKEN_INFO,
};

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

pub fn query_cw721_transfer_exempt(
    deps: Deps,
    address: String,
) -> StdResult<Cw721TransferExemptResponse> {
    let address = deps.api.addr_validate(&address)?;
    let state = CW721_TRANSFER_EXEMPT.load(deps.storage, &address)?;

    Ok(Cw721TransferExemptResponse { state })
}

pub fn query_owner_of(
    deps: Deps,
    env: Env,
    token_id: String,
    include_expired: bool,
) -> StdResult<OwnerOfResponse> {
    let nft_info = NFT_TOKENS.load(deps.storage, &token_id)?;

    Ok(OwnerOfResponse {
        owner: nft_info.owner.to_string(),
        approvals: humanize_approvals(&env.block, &nft_info, include_expired),
    })
}

fn humanize_approvals(
    block: &BlockInfo,
    nft_info: &NftInfo,
    include_expired: bool,
) -> Vec<cw721::Approval> {
    nft_info
        .approvals
        .iter()
        .filter(|apr| include_expired || !apr.is_expired(block))
        .map(humanize_approval)
        .collect()
}

fn humanize_approval(approval: &Approval) -> cw721::Approval {
    cw721::Approval {
        spender: approval.spender.to_string(),
        expires: approval.expires,
    }
}
