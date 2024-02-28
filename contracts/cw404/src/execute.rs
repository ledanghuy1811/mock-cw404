use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};

use crate::error::ContractError;
use crate::state::{BALANCES, TOKEN_INFO};

pub fn execute_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let recipient_addr = deps.api.addr_validate(&recipient)?;

    _tranfer_cw20_with_cw721(deps, env, info, recipient_addr, amount)
}

// Internal function for Cw-20 transfers. Also handles any Cw-721 transfers that may be required.
fn _tranfer_cw20_with_cw721(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let cw20_balance_of_sender_before = BALANCES
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();
    let cw20_balance_of_recipient_before = BALANCES
        .may_load(deps.storage, &recipient)?
        .unwrap_or_default();

    // Transfer cw20 token here
    let cw20_resp = _tranfer_cw20(&mut deps, env, &info, &recipient, amount);

    // 1. First deal with the whole tokens. These are easy and will just be transferred.
    // 2. Look at the fractional part of the value:
    //   a) If it causes the sender to lose a whole token that was represented by an NFT due to a
    //      fractional part being transferred, withdraw and store an additional NFT from the sender.
    //   b) If it causes the receiver to gain a whole new token that should be represented by an NFT
    //      due to receiving a fractional part that completes a whole token, retrieve or mint an NFT to the recevier.
    let token_info = TOKEN_INFO.load(deps.storage)?;
    let nft_to_transfer = amount / token_info.units;
    for i in 0..nft_to_transfer.u128() {
        // Transfer nft here
    }

    let cw20_balance_of_sender_after = BALANCES.load(deps.storage, &info.sender)?;
    let cw20_balance_of_recipient_after = BALANCES.load(deps.storage, &recipient)?;

    // First check if the send causes the sender to lose a whole token that was represented by an Cw721
    // due to a fractional part being transferred.
    //
    // Process:
    // Take the difference between the whole number of tokens before and after the transfer for the sender.
    // If that difference is greater than the number of Cw721 transferred (whole units), then there was
    // an additional Cw721 lost due to the fractional portion of the transfer.
    // If this is a self-send and the before and after balances are equal (not always the case but often),
    // then no Cw721s will be lost here.
    if (cw20_balance_of_sender_before / token_info.units
        - cw20_balance_of_sender_after / token_info.units)
        > nft_to_transfer
    {
        // withdraw and store cw721
    }

    // Then, check if the transfer causes the receiver to gain a whole new token which requires gaining
    // an additional Cw21.
    //
    // Process:
    // Take the difference between the whole number of tokens before and after the transfer for the recipient.
    // If that difference is greater than the number of Cw21s transferred (whole units), then there was
    // an additional Cw21 gained due to the fractional portion of the transfer.
    // Again, for self-sends where the before and after balances are equal, no Cw21s will be gained here.
    if (cw20_balance_of_recipient_after / token_info.units - cw20_balance_of_recipient_before / token_info.units) > nft_to_transfer {
        // retrieve or mint cw721
    }

    Ok(Response::default())
}

fn _tranfer_cw20(
    deps: &mut DepsMut,
    _env: Env,
    info: &MessageInfo,
    recipient: &Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    BALANCES.update(
        deps.storage,
        &info.sender,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_sub(amount)?)
        },
    )?;
    BALANCES.update(
        deps.storage,
        &recipient,
        |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + amount) },
    )?;

    let res = Response::new()
        .add_attribute("action", "transfer cw20")
        .add_attribute("from", &info.sender)
        .add_attribute("to", recipient)
        .add_attribute("amount", amount);

    Ok(res)
}
