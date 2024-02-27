use cosmwasm_std::{StdError, OverflowError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Duplicate initial balance addresses")]
    DuplicateInitialBalanceses {},

    #[error("{0}")]
    OverflowError(#[from] OverflowError)
}
