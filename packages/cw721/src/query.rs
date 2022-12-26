use cosmwasm_schema::{cw_serde,QueryResponses};
use schemars::JsonSchema;
use cosmwasm_std::Uint128;
use cw_utils::Expiration;

#[cw_serde]
pub struct TokenSupply {
    pub total_supply: Uint128,
    pub max_supply: Uint128,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw721QueryMsg<Q: JsonSchema> {
    
    /// Like Tokens but no any filter just token_id that we have.
    /// Return type: TokensResponse.
    #[returns(TokensResponse)]
    AllBalance {owner: String},

    /// Return the owner of the given token, error if token does not exist
    #[returns(OwnerOfResponse)]
    OwnerOf {token_id: String},

    /// Only with "allowance" extension.
    /// Returns how much spender can use from owner account, 0 if unset.
    #[returns(AllowanceResponse)]
    Allowance { owner: String, spender: String },

    /// List all operators that can access all of the owner's tokens
    #[returns(OperatorsResponse)]
    AllOperators {
        owner: String,
        /// unset or false will filter out expired items, you must set to true to see them
        include_expired: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Total number of tokens issued
    #[returns(TokenSupplyResponse)]
    TokenSupply {},

    /// With MetaData Extension.
    /// Returns top-level metadata about the contract
    #[returns(ContractInfoResponse)]
    ContractInfo {},

    /// With MetaData Extension.
    /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
    /// but directly from the contract
    #[returns(TokenInfoResponse<Q>)]
    TokenInfo { token_id: String },

    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients
    #[returns(AllTokenInfoResponse<Q>)]
    AllTokenInfo {token_id: String},

    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    #[returns(TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    #[returns(TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// Extension query
    #[returns(())]
    Extension { msg: Q },
}

#[cw_serde]
pub struct OwnerOfResponse {
    /// Owner of the token
    pub owner: String,

}

#[cw_serde]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: String,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
}

#[cw_serde]
pub struct ApprovalResponse {
    pub approval: Approval,
}

#[cw_serde]
pub struct ApprovalsResponse {
    pub approvals: Vec<Approval>,
}

#[cw_serde]
pub struct OperatorsResponse {
    pub operators: Vec<Approval>,
}

#[cw_serde]
pub struct TokenSupplyResponse {
    pub supply: TokenSupply,
}

#[cw_serde]
pub struct ContractInfoResponse {
    pub name: String,
    pub symbol: String,
    pub owner : String,
}

#[cw_serde]
pub struct TokenInfoResponse<T> {
    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,
    /// You can add any custom metadata here when you extend cw721-base
    pub extension: T,
}

#[cw_serde]
pub struct AllTokenInfoResponse<T> {
    /// Who is owner of token
    pub owner: String,
    /// Data on the token itself,
    pub info: TokenInfoResponse<T>,
}

#[cw_serde]
pub struct TokensResponse {
    /// Contains all token_ids in lexicographical ordering
    /// If there are more than `limit`, use `start_from` in future queries
    /// to achieve pagination.
    pub tokens: Vec<String>,
}

#[cw_serde]
#[derive(Default)]
pub struct AllowanceResponse {
    pub allowance: Uint128,
    pub expires: Expiration,
}