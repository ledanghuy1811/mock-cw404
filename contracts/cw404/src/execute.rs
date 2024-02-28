use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, Uint128};

use crate::error::ContractError;
use crate::state::{
    NftInfo, BALANCES, CW721_TRANSFER_EXEMPT, DEQUE_NFT, NFT_COUNT, NFT_TOKENS, TOKEN_INFO,
};

pub fn execute_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let recipient_addr = deps.api.addr_validate(&recipient)?;

    _tranfer_cw20_with_cw721(deps, env, info, recipient_addr.to_string(), amount)
}

// Internal function for Cw-20 transfers. Also handles any Cw-721 transfers that may be required.
fn _tranfer_cw20_with_cw721(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let recipient_address = deps.api.addr_validate(&recipient)?;

    let cw20_balance_of_sender_before = BALANCES
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();
    let cw20_balance_of_recipient_before = BALANCES
        .may_load(deps.storage, &recipient_address)?
        .unwrap_or_default();

    // Transfer cw20 token here
    let cw20_resp = _tranfer_cw20(&mut deps, &info, recipient.clone(), amount)?;

    // cw721 transfer exempt
    let is_sender_cw721_exempt = CW721_TRANSFER_EXEMPT
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();
    let is_recipient_cw721_exempt = CW721_TRANSFER_EXEMPT
        .may_load(deps.storage, &recipient_address)?
        .unwrap_or_default();

    // cw20 balance after
    let cw20_balance_of_sender_after = BALANCES.load(deps.storage, &info.sender)?;
    let cw20_balance_of_recipient_after = BALANCES.load(deps.storage, &recipient_address)?;
    let token_info = TOKEN_INFO.load(deps.storage)?;

    if is_sender_cw721_exempt && is_recipient_cw721_exempt {
        // Case 1) Both sender and recipient are Cw721 transfer exempt. No Cw721s need to be transferred.
        // NOOP.
    } else if is_sender_cw721_exempt {
        // Case 2) The sender is Cw721 transfer exempt, but the recipient is not. Contract should not attempt
        //         to transfer Cw721s from the sender, but the recipient should receive Cw721s
        //         from the bank/minted for any whole number increase in their balance.
        // Only cares about whole number increments.
        let nft_to_retrieve_or_mint = cw20_balance_of_recipient_after / token_info.units
            - cw20_balance_of_recipient_before / token_info.units;
        for i in 0..nft_to_retrieve_or_mint.u128() {
            let res = _retrieve_or_mint_cw721(&mut deps, env.clone(), &info, recipient.clone())?;
        }
    } else if is_recipient_cw721_exempt {
        // Case 3) The sender is not Cw721 transfer exempt, but the recipient is. Contract should attempt
        //         to withdraw and store Cw721s from the sender, but the recipient should not
        //         receive Cw721s from the bank/minted.
        // Only cares about whole number increments.
        // withdraw or store nft
    } else {
        // Case 4) Neither the sender nor the recipient are Cw721 transfer exempt.
        // Strategy:
        // 1. First deal with the whole tokens. These are easy and will just be transferred.
        // 2. Look at the fractional part of the value:
        //   a) If it causes the sender to lose a whole token that was represented by an NFT due to a
        //      fractional part being transferred, withdraw and store an additional NFT from the sender.
        //   b) If it causes the receiver to gain a whole new token that should be represented by an NFT
        //      due to receiving a fractional part that completes a whole token, retrieve or mint an NFT to the recevier.
        let nft_to_transfer = amount / token_info.units;
        for i in 0..nft_to_transfer.u128() {
            // Transfer nft here
        }

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
        if (cw20_balance_of_recipient_after / token_info.units
            - cw20_balance_of_recipient_before / token_info.units)
            > nft_to_transfer
        {
            // retrieve or mint cw721
        }
    }

    Ok(Response::default())
}

fn _tranfer_cw20(
    deps: &mut DepsMut,
    info: &MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let recipient = deps.api.addr_validate(&recipient)?;
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

fn _retrieve_or_mint_cw721(
    deps: &mut DepsMut,
    _env: Env,
    info: &MessageInfo,
    to: String,
) -> Result<Response, ContractError> {
    let token_id: Uint128;
    let is_deque_empty = DEQUE_NFT.is_empty(deps.storage)?;
    if !is_deque_empty {
        // If there are any tokens in the bank, use those first.
        // Pop off the end of the queue (FIFO).
        token_id = DEQUE_NFT.pop_back(deps.storage)?.unwrap();
    } else {
        // Otherwise, mint a new token, should not be able to go over the total fractional supply.
        let nft_count = NFT_COUNT.load(deps.storage)?;
        token_id = Uint128::from(nft_count).checked_add(Uint128::one())?;
        NFT_COUNT.save(deps.storage, &(nft_count + 1))?;
    }

    let token_info = TOKEN_INFO.load(deps.storage)?;
    let mut token_uri = token_info.base_token_uri.unwrap();
    token_uri.push_str(&token_id.to_string());

    _mint_cw721(deps, info, token_id, to, token_uri)
}

pub fn _mint_cw721(
    deps: &mut DepsMut,
    info: &MessageInfo,
    token_id: Uint128,
    owner: String,
    token_uri: String,
) -> Result<Response, ContractError> {
    let owner = deps.api.addr_validate(&owner)?;
    let nft_token = NftInfo {
        owner: owner.clone(),
        approvals: vec![],
        token_uri: Option::Some(token_uri),
    };
    NFT_TOKENS.update(deps.storage, &token_id.to_string(), |old| match old {
        Some(_) => Err(ContractError::Claimed {}),
        None => Ok(nft_token),
    })?;

    let resp = Response::new()
        .add_attribute("action", "mint nft")
        .add_attribute("minter", &info.sender)
        .add_attribute("owner", owner)
        .add_attribute("token_id", token_id);
    Ok(resp)
}
