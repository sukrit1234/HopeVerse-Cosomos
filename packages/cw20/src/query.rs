use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Binary/*,StdError, StdResult */, Uint128};
use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::logo::LogoInfo;
use cw_utils::Expiration;


#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw20QueryMsg {
    
    /// Returns the current balance of the given address, 0 if unset.
    #[returns(BalanceResponse)]
    Balance { address: String },
    
    /// Returns metadata on the contract - name, decimals, supply, etc.
    #[returns(TokenInfoResponse)]
    TokenInfo {},
    
    #[returns(TokenSupplyResponse)]
    TokenSupply {},

    /// Only with "allowance" extension.
    /// Returns how much spender can use from owner account, 0 if unset.
    #[returns(AllowanceResponse)]
    Allowance { owner: String, spender: String },
    
    /// Only with "enumerable" extension (and "allowances")
    //Get all spender addresses and allowance infos , that account owner address approve.
    #[returns(AllAllowancesResponse)]
    AllAllowances {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// Only with "enumerable" extension (and "allowances")
    // Get all account owner addresses and allowance infos that spender approved or granted by these account owner addresses.
    #[returns(AllSpenderAllowancesResponse)]
    AllSpenderAllowances {
        spender: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// Only with "enumerable" extension
    /// Returns all accounts that have balances. Supports pagination.
    #[returns(AllAccountsResponse)]
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Only with "marketing" extension
    /// Returns more metadata on the contract to display in the client:
    /// - description, logo, project url, etc.
    #[returns(MarketingInfoResponse)]
    MarketingInfo {},

    /// Only with "marketing" extension
    /// Downloads the embedded logo data (if stored on chain). Errors if no logo data is stored for this
    /// contract.
    #[returns(DownloadLogoResponse)]
    DownloadLogo {},
}

#[cw_serde]
pub struct BalanceResponse {
    pub balance: Uint128,
}

#[cw_serde]
pub struct TokenInfoResponse {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[cw_serde]
pub struct TokenSupplyResponse {
    pub total_supply: Uint128,
    pub max_supply: Uint128,
}

#[cw_serde]
#[derive(Default)]
pub struct MarketingInfoResponse {
    /// A URL pointing to the project behind this token.
    pub project: Option<String>,
    /// A longer description of the token and it's utility. Designed for tooltips or such
    pub description: Option<String>,
    /// A link to the logo, or a comment there is an on-chain logo stored
    pub logo: Option<LogoInfo>,
    /// The address (if any) who can update this data structure
    pub marketing: Option<Addr>,
}

/// When we download an embedded logo, we get this response type.
/// We expect a SPA to be able to accept this info and display it.
#[cw_serde]
pub struct DownloadLogoResponse {
    pub mime_type: String,
    pub data: Binary,
}



#[cw_serde]
#[derive(Default)]
pub struct AllowanceResponse {
    pub allowance: Uint128,
    pub expires: Expiration,
}

#[cw_serde]
pub struct AllowanceInfo {
    pub spender: String,
    pub allowance: Uint128,
    pub expires: Expiration,
}

#[cw_serde]
#[derive(Default)]
pub struct AllAllowancesResponse {
    pub allowances: Vec<AllowanceInfo>,
}

#[cw_serde]
pub struct SpenderAllowanceInfo {
    pub owner: String,
    pub allowance: Uint128,
    pub expires: Expiration,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct AllSpenderAllowancesResponse {
    pub allowances: Vec<SpenderAllowanceInfo>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub struct AllAccountsResponse {
    pub accounts: Vec<String>,
}
