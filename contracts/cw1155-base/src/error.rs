use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Expired")]
    Expired {},

    #[error("Zero Amount")]
    InvalidZeroAmount{},

    #[error("Token id : {token_id} undefined")]
    TokenUndefined{token_id : String},

    #[error("Invalid addresses transfer")]
    InvalidTransferAddress{},

    #[error("Exceed max supply")]
    ExceedMaxSupply{},
}
