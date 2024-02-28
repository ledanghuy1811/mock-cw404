#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw20::Cw20Coin;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_balance, query_token_info, query_nft_num_token, query_max_nft_supply, query_cw721_transfer_exempt};
use crate::state::{TokenInfo, BALANCES, TOKEN_INFO, NFT_COUNT, MAX_NFT_SUPPLY, CW721_TRANSFER_EXEMPT};
use crate::execute::execute_transfer;

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// instantiate contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // check valid token info
    msg.validate()?;
    // create initial accounts
    let units = Uint128::from(10u128.pow(u32::from(msg.decimals)));
    let total_supply = create_accounts(&mut deps, &msg.initial_balances, units)?;
    let admin = deps.api.addr_validate(&msg.admin)?;

    let data = TokenInfo {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        total_supply,
        admin: admin.clone(),
        units,
        base_token_uri: msg.base_token_uri
    };
    TOKEN_INFO.save(deps.storage, &data)?;
    MAX_NFT_SUPPLY.save(deps.storage, &(total_supply / units))?;
    NFT_COUNT.save(deps.storage, &0)?;
    CW721_TRANSFER_EXEMPT.save(deps.storage, &admin, &true)?;

    Ok(Response::default())
}

pub fn create_accounts(
    deps: &mut DepsMut,
    accounts: &[Cw20Coin],
    units: Uint128
) -> Result<Uint128, ContractError> {
    validate_accounts(accounts)?;

    let mut total_supply = Uint128::zero();
    for account in accounts {
        let address = deps.api.addr_validate(&account.address)?;
        let ammout = account.amount.checked_mul(units)?;
        BALANCES.save(deps.storage, &address, &ammout)?;
        total_supply += ammout;
    }

    Ok(total_supply)
}

pub fn validate_accounts(accounts: &[Cw20Coin]) -> Result<(), ContractError> {
    let mut addresses = accounts.iter().map(|c| &c.address).collect::<Vec<_>>();
    addresses.sort();
    addresses.dedup();

    if addresses.len() != accounts.len() {
        Err(ContractError::DuplicateInitialBalanceses {})
    } else {
        Ok(())
    }
}

// execute contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer { recipient, amount } => execute_transfer(deps, env, info, recipient, amount)
    }
}

// query contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // cw20 query
        QueryMsg::Balance { address } => to_json_binary(&query_balance(deps, address)?),
        QueryMsg::TokenInfo {} => to_json_binary(&query_token_info(deps)?),

        // cw721 query
        QueryMsg::NftNumTokens {  } => to_json_binary(&query_nft_num_token(deps)?),
        QueryMsg::MaxNftSupply {  } => to_json_binary(&query_max_nft_supply(deps)?),
        QueryMsg::Cw721TransferExempt { address } => to_json_binary(&query_cw721_transfer_exempt(deps, address)?),
    }
}
