use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Cannot set approval that is already expired")]
    Expired {},

    #[error("Approval not found for: {spender}")]
    ApprovalNotFound { spender: String },

    #[error("Not found token id : {token_id}")]
    TokenNotFound { token_id : String },

    #[error("Address is not token owner")]
    NotTokenOwner { },

    #[error("Exceed max supply")]
    ExceedMaxSupply { },

    #[error("Total Supply Underflow")]
    TotalSupplyUnderflow { },

    #[error("Batch dimension mismatch")]
    BatchDimensionMismatch { },
}
