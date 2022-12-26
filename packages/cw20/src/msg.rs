use crate::logo::Logo;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Uint128};
use cw_utils::Expiration;

#[cw_serde]
pub enum Cw20ExecuteMsg {

    //Change owner only current owner can do this.
    ChangeOwner {new_owner : String},

    /// Transfer is a base message to move tokens to another account without triggering actions
    Transfer { to: String, amount: Uint128 },
    /// Burn is a base message to destroy tokens forever
    Burn { amount: Uint128 },
    /// Send is a base message to transfer tokens to a contract and trigger an action
    /// on the receiving contract.
    Send {
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Only with "approval" extension. Allows spender to access an additional amount tokens
    /// from the owner's (env.sender) account. If expires is Some(), overwrites current allowance
    /// expiration with this one.
    IncreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    /// Only with "approval" extension. Lowers the spender's access of tokens
    /// from the owner's (env.sender) account by amount. If expires is Some(), overwrites current
    /// allowance expiration with this one.
    DecreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    
    /// Only with "approval" extension. Transfers amount tokens from from -> recipient
    /// if `env.sender` has sufficient pre-approval.
    TransferFrom {
        from: String,
        to: String,
        amount: Uint128,
    },

    /// Only with "approval" extension. Sends amount tokens from from -> contract
    /// if `env.sender` has sufficient pre-approval.
    SendFrom {
        from: String,
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Only with "approval" extension. Destroys tokens forever
    BurnFrom { from: String, amount: Uint128 },
    /// Only with the "mintable" extension. If authorized, creates amount new tokens
    /// and adds to the recipient balance.
    Mint { to: String, amount: Uint128 },


    // Set Cooperative address link contract address that allow to call Cw20 contract
    SetCooperative { cooperative: String,  can_mint_for : bool , can_burn_from : bool},

    //Unset Cooperative address
    UnSetCooperative { cooperative: String},

    /*Set Max Supply for token*/
    SetMaxSupply {max_supply : Uint128},

    /// Only with the "marketing" extension. If authorized, updates marketing metadata.
    /// Setting None/null for any of these will leave it unchanged.
    /// Setting Some("") will clear this field on the contract storage
    UpdateMarketing {
        /// A URL pointing to the project behind this token.
        project: Option<String>,
        /// A longer description of the token and it's utility. Designed for tooltips or such
        description: Option<String>,
        /// The address (if any) who can update this data structure
        marketing: Option<String>,
    },
    /// If set as the "marketing" role on the contract, upload a new URL, SVG, or PNG for the token
    UploadLogo(Logo),
}
