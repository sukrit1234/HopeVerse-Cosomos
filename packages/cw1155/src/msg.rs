use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, Uint128};
use cw_utils::Expiration;

pub type TokenId = String;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Cw1155ExecuteMsg {
    
    //Change contract owner only current owner to do this.
    ChangeOwner{new_owner : String},

    Transfer {
        to: String,
        token_id: TokenId,
        amount: Uint128,
    },

    TransferFrom {
        from: String,
        to: String,
        token_id: TokenId,
        amount: Uint128,
    },

    BatchTransfer {
        to : String,
        batch: Vec<(TokenId, Uint128)>,
    },

    BatchTransferFrom {
        from: String,
        to : String,
        batch: Vec<(TokenId, Uint128)>,
    },


    Send {
        contract: String,
        token_id: TokenId,
        amount: Uint128,
        msg: Binary,
    },

    SendFrom {
        from: String,
        contract: String,
        token_id: TokenId,
        amount: Uint128,
        msg: Binary,
    },
    
    BatchSend {
        contract : String,
        batch: Vec<(TokenId, Uint128)>,
        msg: Binary,
    },

    BatchSendFrom {
        from: String,
        contract : String,
        batch: Vec<(TokenId, Uint128)>,
        msg: Binary,
    },


    /// Mint is a base message to mint tokens.
    Mint {
        /// If `to` is not contract, `msg` should be `None`
        to: String,
        token_id: TokenId,
        amount: Uint128,
    },
    /// BatchMint is a base message to mint multiple types of tokens in batch.
    BatchMint {
        /// If `to` is not contract, `msg` should be `None`
        to: String,
        batch: Vec<(TokenId, Uint128)>,
    },
    /// Burn is a base message to burn tokens.
    Burn {
        from: String,
        token_id: TokenId,
        amount: Uint128,
    },
    /// BatchBurn is a base message to burn multiple types of tokens in batch.
    BatchBurn {
        from: String,
        batch: Vec<(TokenId, Uint128)>,
    },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll { operator: String },

    // URI to define token
    DefineToken {token_uri : String, max_supply : Uint128},

    // Update token uri
    UpdateTokenUri {token_id : TokenId,token_uri : String},

    // Update token max supply
    UpdateMaxSupply {token_id : TokenId,max_supply : Uint128},

    SetCooperative { cooperative : String , can_mint_for : bool , can_burn_from : bool},

    UnsetCooperative { cooperative : String }
}
