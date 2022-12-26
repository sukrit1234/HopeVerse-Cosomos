use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Uint128,Binary};
use cw_utils::Expiration;

/// This is like Cw721ExecuteMsg but we add a Mint command for an owner
/// to make this stand-alone. You will likely want to remove mint and
/// use other control logic in any contract that inherits this.
#[cw_serde]
pub enum Cw721ExecuteMsg<T, E> {
    
    //Change contract owner only current owner to do this.
    ChangeOwner {new_owner : String},
    
    /// Transfer is a base message to move a token to another account without triggering actions
    Transfer {
        to : String, 
        token_id: String 
    },

    /// TransferFrom is a base message to move a token from account to another account without triggering actions
    /// that use for admin/owner or another cooperative contract to use this.
    TransferFrom { 
        from : String,
        to: String, 
        token_id: String 
    },

    /// Send is a base message to transfer a token to a contract and trigger an action
    /// on the receiving contract.
    Send {
        contract: String,
        token_id: String,
        msg: Binary,
    },

    /// SendFrom is a base message to transfer a token from wallet account to a contract and trigger an action
    /// on the receiving contract. for admin/owner or another cooperative contract to use this.
    SendFrom {
        from : String,
        contract: String,
        token_id: String,
        msg: Binary,
    },

    /// Allows spender to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        spender: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll { spender: String },

    // Update exist NFT token uri at specific id.
    UpdateTokenURI { token_id : String,token_uri: Option<String>},
    
    // Update exist NFT token extension data at specific id.
    UpdateTokenExtension { token_id : String , extension: T},

    /// Mint a new NFT, can only be called by the contract minter
    Mint{token_owner: String,token_uri: Option<String>,extension: T},

    /// Mint a new NFT, can only be called by the contract minter
    MintBatch{token_owner: String,token_uris: Vec<Option<String>>,extensions: Vec<T>},

    /// Burn an NFT the sender has access to
    Burn { token_id: String ,from_address : Option<String>},

    /// Burn an NFT the sender has access to
    BurnBatch { token_ids: Vec<String> ,from_address : Option<String>},

    UpdateMaxSupply {max_supply : Uint128},

    SetCooperative { cooperative : String , can_mint_for : bool , can_burn_from : bool},

    UnsetCooperative { cooperative : String },

    /// Extension msg
    Extension { msg: E },
}