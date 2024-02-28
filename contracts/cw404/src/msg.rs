use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{StdError, StdResult, Uint128};
use cw20::{BalanceResponse, Cw20Coin};
use cw721::NumTokensResponse;

use cw404_package::{MaxNftSupplyRespone, TokenInfoResponse, Cw721TransferExemptResponse};

// instantiate msg
#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
    pub admin: String,
    pub base_token_uri: Option<String>,
}

impl InstantiateMsg {
    pub fn validate(&self) -> StdResult<()> {
        // check name, symbol, decimals
        if !self.has_valid_name() {
            return Err(StdError::generic_err(
                "Name is not in the expected format (3-50 UTF-8 bytes)",
            ));
        }
        if !self.has_valid_symbol() {
            return Err(StdError::generic_err(
                "Ticker symbol is not in expected format [a-zA-Z\\-]{3,12}",
            ));
        }
        if self.decimals > 18 {
            return Err(StdError::generic_err("Decimals must not exceed 18"));
        }
        Ok(())
    }

    fn has_valid_name(&self) -> bool {
        let bytes = self.name.as_bytes();
        if bytes.len() < 3 || bytes.len() > 50 {
            return false;
        }
        true
    }

    fn has_valid_symbol(&self) -> bool {
        let bytes = self.symbol.as_bytes();
        if bytes.len() < 3 || bytes.len() > 12 {
            return false;
        }
        for byte in bytes.iter() {
            if (*byte != 45) && (*byte < 65 || *byte > 90) && (*byte < 97 || *byte > 122) {
                return false;
            }
        }
        true
    }
}

// execute msg
#[cw_serde]
pub enum ExecuteMsg {
    // Transfer is a base message to move tokens to another account without triggering actions
    // transfer token also nft
    Transfer { recipient: String, amount: Uint128 },
}

// query msg
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Cw20 query
    // Returns the current balance of the given address, 0 if unset.
    #[returns(BalanceResponse)]
    Balance { address: String },
    // Returns metadata on the contract - name, decimals, supply, etc.
    #[returns(TokenInfoResponse)]
    TokenInfo {},

    /// Cw721 query
    // Total number of tokens issued
    #[returns(NumTokensResponse)]
    NftNumTokens {},
    // Max NFT supply
    #[returns(MaxNftSupplyRespone)]
    MaxNftSupply {},
    // Cw721 transfer exempt
    #[returns(Cw721TransferExemptResponse)]
    Cw721TransferExempt { address: String },
}
