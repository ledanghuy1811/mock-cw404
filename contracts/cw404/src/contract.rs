#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use cw2::set_contract_version;
use cw20::Cw20Coin;

use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use crate::state::{BALANCES, TokenInfo, TOKEN_INFO};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

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
    let total_supply = create_accounts(&mut deps, &msg.initial_balances)?;
    let admin = deps.api.addr_validate(&msg.admin)?;
    let units = Uint128::from(10u128.pow(u32::from(msg.decimals)));

    let data = TokenInfo {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        total_supply,
        admin, 
        units
    };
    TOKEN_INFO.save(deps.storage, &data)?;
    
    Ok(Response::default())
}

pub fn create_accounts(
    deps: &mut DepsMut,
    accounts: &[Cw20Coin],
) -> Result<Uint128, ContractError> {
    validate_accounts(accounts)?;

    let mut total_supply = Uint128::zero();
    for account in accounts {
        let address = deps.api.addr_validate(&account.address)?;
        BALANCES.save(deps.storage, &address, &account.amount)?;
        total_supply += account.amount;
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
