use cosmwasm_schema::{cw_serde,QueryResponses};

use cosmwasm_std::Uint128;
use cw_utils::Expiration;

use crate::msg::TokenId;

#[cw_serde]
pub struct TokenSupply {
    pub total_supply: Uint128,
    pub max_supply: Uint128,
}

impl Default for TokenSupply {
    fn default() -> Self{
        Self{
            total_supply : Uint128::from(0u128),
            max_supply : Uint128::from(0u128),
        }
    }
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw1155QueryMsg {
    /// Returns the current balance of the given address, 0 if unset.
    /// Return type: BalanceResponse.
    #[returns(BalanceResponse)]
    Balance { owner: String, token_id: TokenId },
    
    /// Returns the current balance of the given address for a batch of tokens, 0 if unset.
    /// Return type: BatchBalanceResponse.
    #[returns(BatchBalanceResponse)]
    BatchBalance {
        owner: String,
        token_ids: Vec<TokenId>,
    },

    /// Returns the current balance of the given address for all of token, 0 if unset.
    /// Return type: AllBalanceResponse.
    #[returns(AllBalanceResponse)]
    AllBalance {owner: String},

    /// List all operators that can access all of the owner's tokens.
    /// Return type: OperatorsResponse.
    #[returns(OperatorsResponse)]
    AllOperators {
        owner: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    
    /*Need to revise */
    /// Query approved status `owner` granted to `operator`.
    /// Return type: IsApprovedForAllResponse
    #[returns(AllowanceResponse)]
    Allowance { owner: String, operator: String },

    /// With MetaData Extension.
    /// Query metadata of token
    /// Return type: TokenInfoResponse.
    #[returns(TokenInfoResponse)]
    TokenInfo { token_id: TokenId },

    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    /// Return type: TokensResponse.
    #[returns(TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    /// Return type: TokensResponse.

    #[returns(TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// With MetaData Extension.
    /// Returns top-level metadata about the contract
    /// Return type : ContractInfoResponse
    #[returns(ContractInfoResponse)]
    ContractInfo {},

    #[returns(TokenSupplyResponse)]
    TokenSupply {token_id: String},

    #[returns(TokenSuppliesResponse)]
    TokenSupplies {token_ids: Vec<TokenId>},

    #[returns(LastTokenIdResponse)]
    LastTokenID {},
}

#[cw_serde]
pub struct ContractInfoResponse {
    pub name: String,
    pub symbol: String,
    pub owner : String,
}

#[cw_serde]
pub struct BalanceResponse {
    pub balance: Uint128,
}

#[cw_serde]
pub struct BatchBalanceResponse {
    pub balances: Vec<Uint128>,
}

#[cw_serde]
pub struct AllBalanceResponse {
    pub tokenids: Vec<String>,
    pub amounts : Vec<Uint128>,
}

#[cw_serde]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: String,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
}

#[cw_serde]
pub struct OperatorsResponse {
    pub operators: Vec<Approval>,
}

#[cw_serde]
pub struct AllowanceResponse {
    pub approved: bool
}

#[cw_serde]
pub struct TokenInfoResponse {
    /// Should be a url point to a json file
    pub url: String,
}

#[cw_serde]
pub struct TokenSupplyResponse {
    /// Should be a url point to a json file
    pub supply : TokenSupply
}

#[cw_serde]
pub struct TokenSuppliesResponse {
    /// Should be a url point to a json file
    pub supplies: Vec<TokenSupply>,
}

#[cw_serde]
pub struct TokensResponse {
    /// Contains all token_ids in lexicographical ordering
    /// If there are more than `limit`, use `start_from` in future queries
    /// to achieve pagination.
    pub tokens: Vec<TokenId>,
}

#[cw_serde]
pub struct LastTokenIdResponse {
    pub token_id: Uint128,
}