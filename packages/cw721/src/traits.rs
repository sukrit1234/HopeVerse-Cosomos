use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::query::{AllowanceResponse};
use crate::{
    AllTokenInfoResponse, ContractInfoResponse, TokenInfoResponse,
    TokenSupplyResponse, OperatorsResponse, OwnerOfResponse, TokensResponse,
};
use cosmwasm_std::{Binary, CustomMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw_utils::Expiration;

pub trait Cw721<T, C>: Cw721Execute<T, C> + Cw721Query<T>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
}

pub trait Cw721Execute<T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
    type Err: ToString;

    fn transfer(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,
        token_id: String,
    ) -> Result<Response<C>, Self::Err>;

    fn send(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        contract: String,
        token_id: String,
        msg: Binary,
    ) -> Result<Response<C>, Self::Err>;

    fn transfer_from(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        from : String,
        recipient: String,
        token_id: String,
    ) -> Result<Response<C>, Self::Err>;

    fn send_from(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        from : String,
        contract: String,
        token_id: String,
        msg: Binary,
    ) -> Result<Response<C>, Self::Err>;

    fn approve_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        operator: String,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, Self::Err>;

    fn revoke_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        operator: String,
    ) -> Result<Response<C>, Self::Err>;

    fn burn(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        from_address : Option<String>,
    ) -> Result<Response<C>, Self::Err>;

    fn burn_batch(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_ids: Vec<String>,
        from_address : Option<String>,
    ) -> Result<Response<C>, Self::Err>;

    fn mint(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        to : String,
        token_uri: Option<String>,
        extension: T,
    ) -> Result<Response<C>, Self::Err>;

    fn mint_batch(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        to : String,
        token_uris: Vec<Option<String>>,
        extensions: Vec<T>,
    ) -> Result<Response<C>, Self::Err>;
}

pub trait Cw721Query<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    // TODO: use custom error?
    // How to handle the two derived error types?

    fn contract_info(&self, deps: Deps) -> StdResult<ContractInfoResponse>;

    fn token_supply(&self, deps: Deps) -> StdResult<TokenSupplyResponse>;

    fn token_info(&self, deps: Deps, token_id: String) -> StdResult<TokenInfoResponse<T>>;

    fn owner_of(
        &self,
        deps: Deps,
        token_id: String,
    ) -> StdResult<OwnerOfResponse>;

    fn all_balance(
        &self,
        deps: Deps,
        owner : String,
    ) -> StdResult<TokensResponse>;

    fn operators(
        &self,
        deps: Deps,
        env: Env,
        owner: String,
        include_expired: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<OperatorsResponse>;

    fn allowance(
        &self,
        deps: Deps,
        env: Env,
        owner: String,
        spender: String,
    ) -> StdResult<AllowanceResponse>;

    fn tokens(
        &self,
        deps: Deps,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse>;

    fn all_tokens(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse>;

    fn all_token_info(
        &self,
        deps: Deps,
        token_id: String,
    ) -> StdResult<AllTokenInfoResponse<T>>;
}
