use std::marker::PhantomData;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, CustomMsg, QuerierWrapper, StdResult, WasmMsg, WasmQuery,Empty,
};
use crate::{
    AllTokenInfoResponse, ContractInfoResponse,AllowanceResponse,
    TokenInfoResponse, TokenSupplyResponse, OperatorsResponse, OwnerOfResponse, TokensResponse,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use cosmwasm_std::Binary;
use crate::{Cw721ExecuteMsg, Cw721QueryMsg};

#[cw_serde]
pub struct Cw721Contract<Q: CustomMsg, E: CustomMsg>(
    pub Addr,
    pub PhantomData<Q>,
    pub PhantomData<E>,
);

#[allow(dead_code)]
impl<Q: CustomMsg, E: CustomMsg> Cw721Contract<Q, E> {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Serialize>(&self, msg: Cw721ExecuteMsg<T, E>) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg)?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    pub fn query<T: DeserializeOwned>(
        &self,
        querier: &QuerierWrapper,
        req: Cw721QueryMsg<Q>,
    ) -> StdResult<T> {
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&req)?,
        }
        .into();
        querier.query(&query)
    }

    
    /*** queries ***/

    pub fn owner_of(
        &self,
        querier: &QuerierWrapper,
        token_id: String,
    ) -> StdResult<OwnerOfResponse> {
        let req = Cw721QueryMsg::OwnerOf {token_id};
        self.query(querier, req)
    }
    pub fn all_balance (
        &self,
        querier: &QuerierWrapper,
        owner: String,
    ) -> StdResult<OwnerOfResponse> {
        let req = Cw721QueryMsg::AllBalance {owner: owner};
        self.query(querier, req)
    }
    /*pub fn approval (
        &self,
        querier: &QuerierWrapper,
        token_id: String,
        spender: String,
        include_expired: bool,
    ) -> StdResult<ApprovalResponse> {
        let req = Cw721QueryMsg::Approval {token_id,spender,include_expired};
        self.query(querier, req)
    }

    pub fn approvals(
        &self,
        querier: &QuerierWrapper,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<ApprovalsResponse> {
        let req = Cw721QueryMsg::Approvals {token_id,include_expired};
        self.query(querier, req)
    }*/

    pub fn all_operators (
        &self,
        querier: &QuerierWrapper,
        owner: String,
        include_expired: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<OperatorsResponse> {
        let req = Cw721QueryMsg::AllOperators {owner,include_expired,start_after,limit};
        self.query(querier, req)
    }

    pub fn allowance (
        &self,
        querier: &QuerierWrapper,
        owner: String, 
        spender: String,
    ) -> StdResult<AllowanceResponse> {
        let req = Cw721QueryMsg::Allowance {owner, spender};
        self.query(querier, req)
    }

    /*pub fn allowance_token (
        &self,
        querier: &QuerierWrapper,
        owner: String,
        token_id : String, 
        spender: String,
    ) -> StdResult<AllowanceResponse> {
        let req = Cw721QueryMsg::AllowanceToken {owner,token_id,spender};
        self.query(querier, req)
    }*/

    pub fn token_supply(&self, querier: &QuerierWrapper) -> StdResult<TokenSupplyResponse> {
        let req = Cw721QueryMsg::TokenSupply {};
        self.query(querier, req)
    }

    /// With metadata extension
    pub fn contract_info(&self, querier: &QuerierWrapper) -> StdResult<ContractInfoResponse> {
        let req = Cw721QueryMsg::ContractInfo {};
        self.query(querier, req)
    }

    /// With metadata extension
    pub fn token_info<T:DeserializeOwned>(
        &self,
        querier: &QuerierWrapper,
        token_id: String,
    ) -> StdResult<TokenInfoResponse<T>> {
        let req = Cw721QueryMsg::TokenInfo {
            token_id: token_id.into(),
        };
        self.query(querier, req)
    }

    /// With metadata extension
    pub fn all_token_info<T:DeserializeOwned>(
        &self,
        querier: &QuerierWrapper,
        token_id: String,
    ) -> StdResult<AllTokenInfoResponse<T>> {
        let req = Cw721QueryMsg::AllTokenInfo {token_id};
        self.query(querier, req)
    }

    /// With enumerable extension
    pub fn tokens(
        &self,
        querier: &QuerierWrapper,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let req = Cw721QueryMsg::Tokens {
            owner: owner.into(),
            start_after,
            limit,
        };
        self.query(querier, req)
    }

    /// With enumerable extension
    pub fn all_tokens(
        &self,
        querier: &QuerierWrapper,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let req = Cw721QueryMsg::AllTokens { start_after, limit };
        self.query(querier, req)
    }

    /// returns true if the contract supports the metadata extension
    pub fn has_metadata(&self, querier: &QuerierWrapper) -> bool {
        self.contract_info(querier).is_ok()
    }

    /// returns true if the contract supports the enumerable extension
    pub fn has_enumerable(&self, querier: &QuerierWrapper) -> bool {
        self.tokens(querier, self.addr().to_string(), None, Some(1)).is_ok()
    }

    /*** Calls ***/
    pub fn mint_for<T: Serialize>(
        &self,
        recipient : String,
        token_uri : Option<String>,
        extension : T,
        ) -> StdResult<CosmosMsg> {
        let mint_msg = Cw721ExecuteMsg::<T, E>::Mint{
            token_owner: recipient,
            token_uri: token_uri,
            extension: extension,
        };
        Ok(self.call(mint_msg)?)
    }
    
    pub fn burn_from (
        &self,
        token_id : String,
        from_address : String,
        ) -> StdResult<CosmosMsg> {

        let burn_msg = Cw721ExecuteMsg::<Empty, E>::Burn{
            token_id : token_id.into(), 
            from_address : Some(from_address)
        };

        Ok(self.call(burn_msg)?)
    }

    pub fn mint_batch_for<T: Serialize>(
        &self,
        to : String,
        token_uris : Vec<Option<String>>,
        extensions : Vec<T>,
        ) -> StdResult<CosmosMsg> {
        let mint_msg = Cw721ExecuteMsg::<T, E>::MintBatch{
            token_owner: to,
            token_uris: token_uris,
            extensions: extensions,
        };
        Ok(self.call(mint_msg)?)
    }

    pub fn burn_batch_from (
        &self,
        token_ids : Vec<String>,
        from_address : String,
        ) -> StdResult<CosmosMsg> {

        let token_ids_str : Vec<String> = token_ids.into_iter().map(|i| i.into()).collect();
        let burn_msg = Cw721ExecuteMsg::<Empty, E>::BurnBatch{
            token_ids : token_ids_str, 
            from_address : Some(from_address)
        };

        Ok(self.call(burn_msg)?)
    }

    pub fn transfer (
        &self,
        token_id: String,
        to: String,
    ) -> StdResult<CosmosMsg> {

        let transfer_msg = Cw721ExecuteMsg::<Empty, E>::Transfer{
            token_id : token_id.into(), 
            to : to
        };
        Ok(self.call(transfer_msg)?)
    }
    
    fn send (
        &self,
        contract: String,
        token_id: String,
        msg: Binary,
    ) -> StdResult<CosmosMsg> {
        
        let transfer_msg = Cw721ExecuteMsg::<Empty, E>::Send {
            contract : contract,
            token_id : token_id.into(),
            msg : msg,
        };
        Ok(self.call(transfer_msg)?)
    }

    pub fn transfer_from (
        &self,
        token_id: String,
        from : String,
        to: String,
    ) -> StdResult<CosmosMsg> {

        let transfer_msg = Cw721ExecuteMsg::<Empty, E>::TransferFrom{
            from : from,
            token_id : token_id.into(), 
            to : to,
        };
        Ok(self.call(transfer_msg)?)
    }
    
    fn send_from (
        &self,
        token_id: String,
        from : String,
        contract: String,
        msg: Binary,
    ) -> StdResult<CosmosMsg> {
        
        let transfer_msg = Cw721ExecuteMsg::<Empty, E>::SendFrom{
            from : from,
            contract : contract,
            token_id : token_id.into(),
            msg : msg,
        };
        Ok(self.call(transfer_msg)?)
    }   
}
