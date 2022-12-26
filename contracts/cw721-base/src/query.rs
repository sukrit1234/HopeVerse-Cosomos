use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{
    to_binary, Addr, Binary, CustomMsg, Deps, Env, Order, StdResult,Uint128,
};

use cw721::{
    AllTokenInfoResponse, ContractInfoResponse, Cw721Query,
    Expiration, TokenInfoResponse, TokenSupplyResponse, OperatorsResponse, OwnerOfResponse,
    TokensResponse,TokenSupply,Cw721QueryMsg,AllowanceResponse,
};
use cw_storage_plus::Bound;
use cw_utils::maybe_addr;
use crate::state::{Cw721Contract};

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 100;

impl<'a, T, C, E, Q> Cw721Query<T> for Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn contract_info(&self, deps: Deps) -> StdResult<ContractInfoResponse> {
       let contract_info = self.contract_info.load(deps.storage)?;
       Ok(ContractInfoResponse{
            name : contract_info.name,
            symbol : contract_info.symbol,
            owner : contract_info.owner.to_string(),
       })
    }

    fn token_supply(&self, deps: Deps) -> StdResult<TokenSupplyResponse> {
        let supply = TokenSupply{
            total_supply : self.get_total_supply(deps.storage)?.into(),
            max_supply : self.get_max_supply(deps.storage)?.into(),
        };
        Ok(TokenSupplyResponse { supply })
    }

    fn token_info(&self, deps: Deps, token_id: String) -> StdResult<TokenInfoResponse<T>> {
        let info = self.tokens.load(deps.storage, &token_id)?;
        Ok(TokenInfoResponse {
            token_uri: info.token_uri,
            extension: info.extension,
        })
    }
    fn owner_of(
        &self,
        deps: Deps,
        token_id: String,
    ) -> StdResult<OwnerOfResponse> {
        let info = self.tokens.load(deps.storage, &token_id)?;
        Ok(OwnerOfResponse {
            owner: info.owner.to_string()
        })        
    }

    fn all_balance(
        &self,
        deps: Deps,
        owner: String,
    ) -> StdResult<TokensResponse> {
        
        let owner_addr = deps.api.addr_validate(&owner)?;
        let tokens: Vec<String> = self
            .tokens
            .idx
            .owner
            .prefix(owner_addr)
            .keys(deps.storage, None, None, Order::Ascending)
            .collect::<StdResult<Vec<_>>>()?;

        Ok(TokensResponse { tokens })
    }

    /// operators returns all operators owner given access to
    fn operators(
        &self,
        deps: Deps,
        env: Env,
        owner: String,
        include_expired: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<OperatorsResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        
        let start_addr = maybe_addr(deps.api, start_after)?;
        let start = start_addr.as_ref().map(Bound::exclusive);

        let owner_addr = deps.api.addr_validate(&owner)?;
        let res: StdResult<Vec<_>> = self
            .spenders
            .prefix(&owner_addr)
            .range(deps.storage, start, None, Order::Ascending)
            .filter(|r| {
                include_expired || r.is_err() || !r.as_ref().unwrap().1.is_expired(&env.block)
            })
            .take(limit)
            .map(parse_approval)
            .collect();
        Ok(OperatorsResponse { operators: res? })
    }

    fn allowance(
        &self,
        deps: Deps,
        env: Env,
        owner: String,
        spender: String,
    ) -> StdResult<AllowanceResponse> {
        
        let owner_addr = deps.api.addr_validate(&owner)?;
        let spender_addr = deps.api.addr_validate(&spender)?;
        
        let expires = self.spenders.may_load(deps.storage, (&owner_addr, &spender_addr))?.unwrap_or_default();
        Ok( AllowanceResponse{
            allowance : Uint128::from(if expires.is_expired(&env.block)  {0u128} else {1u128}),
            expires : expires,
        })
    }

    fn tokens(
        &self,
        deps: Deps,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

        let owner_addr = deps.api.addr_validate(&owner)?;
        let tokens: Vec<String> = self
            .tokens
            .idx
            .owner
            .prefix(owner_addr)
            .keys(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .collect::<StdResult<Vec<_>>>()?;

        Ok(TokensResponse { tokens })
    }

    fn all_tokens(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

        let tokens: StdResult<Vec<String>> = self
            .tokens
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .map(|item| item.map(|(k, _)| k))
            .collect();

        Ok(TokensResponse { tokens: tokens? })
    }

    fn all_token_info(
        &self,
        deps: Deps,
        token_id: String,
    ) -> StdResult<AllTokenInfoResponse<T>> {
        let info = self.tokens.load(deps.storage, &token_id)?;
        Ok(AllTokenInfoResponse {
            owner: info.owner.to_string(),
            info: TokenInfoResponse {
                token_uri: info.token_uri,
                extension: info.extension,
            },
        })
    }
}

impl<'a, T, C, E, Q> Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn query(&self, deps: Deps, env: Env, msg: Cw721QueryMsg<Q>) -> StdResult<Binary> {
        match msg {
            
            Cw721QueryMsg::ContractInfo {} => to_binary(&self.contract_info(deps)?),
            
            Cw721QueryMsg::TokenInfo { token_id } => to_binary(&self.token_info(deps, token_id)?),
            
            Cw721QueryMsg::OwnerOf {token_id} => to_binary(&self.owner_of(deps, token_id)?),

            Cw721QueryMsg::AllBalance{owner} => to_binary(&self.all_balance(deps, owner)?),

            Cw721QueryMsg::AllTokenInfo {token_id} => to_binary(&self.all_token_info(deps,token_id)?),
            
            Cw721QueryMsg::AllOperators { owner,include_expired,start_after,limit} => to_binary(&self.operators(deps,env,owner,include_expired,start_after,limit)?),

            Cw721QueryMsg::TokenSupply {} => to_binary(&self.token_supply(deps)?),
            
            Cw721QueryMsg::Tokens { owner,start_after,limit,} => to_binary(&self.tokens(deps, owner, start_after, limit)?),
            
            Cw721QueryMsg::AllTokens { start_after, limit } => to_binary(&self.all_tokens(deps, start_after, limit)?),

            Cw721QueryMsg::Allowance { owner, spender } => to_binary(&self.allowance(deps, env, owner, spender)?),
            
            Cw721QueryMsg::Extension { msg: _ } => Ok(Binary::default()),
        }
    }
}

fn parse_approval(item: StdResult<(Addr, Expiration)>) -> StdResult<cw721::Approval> {
    item.map(|(spender, expires)| cw721::Approval {
        spender: spender.to_string(),
        expires,
    })
}