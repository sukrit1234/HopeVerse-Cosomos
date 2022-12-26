use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, QuerierWrapper, StdResult, WasmMsg, WasmQuery,Uint128,
};
use crate::{
    TokenId,BalanceResponse,BatchBalanceResponse,AllBalanceResponse,OperatorsResponse,Approval,
    TokenInfoResponse,TokenSupplyResponse,TokensResponse,TokenSuppliesResponse,ContractInfoResponse,
    AllowanceResponse,
};
use serde::de::DeserializeOwned;
use cosmwasm_std::Binary;
use crate::{Cw1155ExecuteMsg, Cw1155QueryMsg};

#[cw_serde]
pub struct Cw1155Contract(pub Addr);

#[allow(dead_code)]
impl Cw1155Contract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call(&self, msg: Cw1155ExecuteMsg) -> StdResult<CosmosMsg> {
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
        req: Cw1155QueryMsg,
    ) -> StdResult<T> {
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&req)?,
        }
        .into();
        querier.query(&query)
    }

    /*** queries ***/
    pub fn balance (
        &self,
        querier: &QuerierWrapper,
        owner: String,
        token_id: String,
    ) -> StdResult<BalanceResponse> {
        let req = Cw1155QueryMsg::Balance {
            owner: owner,
            token_id: token_id,
        };
        self.query(querier, req)
    }
    pub fn batch_balance(
        &self,
        querier: &QuerierWrapper,
        owner: String,
        token_ids: Vec<TokenId>,
    ) -> StdResult<BatchBalanceResponse> {
        let req = Cw1155QueryMsg::BatchBalance {
            owner: owner,
            token_ids : token_ids,
        };
        self.query(querier, req)
    }

    pub fn all_balance(
        &self,
        querier: &QuerierWrapper,
        owner: String,
    ) -> StdResult<AllBalanceResponse> {
        let req = Cw1155QueryMsg::AllBalance {owner: owner};
        self.query(querier, req)
    }

    pub fn all_operators(
        &self,
        querier: &QuerierWrapper,
        owner: String,
        include_expired: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<Vec<Approval>> {
        let req = Cw1155QueryMsg::AllOperators {
            owner,
            include_expired,
            start_after,
            limit,
        };
        let res: OperatorsResponse = self.query(querier, req)?;
        Ok(res.operators)
    }

    /// With metadata extension
    pub fn token_info(
        &self,
        querier: &QuerierWrapper,
        token_id: String,
    ) -> StdResult<TokenInfoResponse> {
        let req = Cw1155QueryMsg::TokenInfo {token_id: token_id};
        self.query(querier, req)
    }

    pub fn token_supply(
        &self,
        querier: &QuerierWrapper,
        token_id: String,
    ) -> StdResult<TokenSupplyResponse>
    {
        let req = Cw1155QueryMsg::TokenSupply {token_id};
        self.query(querier, req)
    }

    pub fn token_supplies(
        &self,
        querier: &QuerierWrapper,
        token_ids: Vec<TokenId>,
    ) -> StdResult<TokenSuppliesResponse>
    {
        let req = Cw1155QueryMsg::TokenSupplies {token_ids};
        self.query(querier, req)
    }

    /// With enumerable extension
    pub fn tokens (
        &self,
        querier: &QuerierWrapper,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let req = Cw1155QueryMsg::Tokens {
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
        let req = Cw1155QueryMsg::AllTokens { start_after, limit };
        self.query(querier, req)
    }

    /// With metadata extension
    pub fn contract_info(
        &self, 
        querier: &QuerierWrapper
    ) -> StdResult<ContractInfoResponse> {
        let req = Cw1155QueryMsg::ContractInfo {};
        self.query(querier, req)
    }

    pub fn allowance (
        &self,
        querier: &QuerierWrapper,
        owner: String, 
        operator: String,
    ) -> StdResult<AllowanceResponse> {
        let req = Cw1155QueryMsg::Allowance {owner, operator};
        self.query(querier, req)
    }

    /*** Calls ***/
    pub fn mint_for (
        &self,
        to: String,
        token_id: TokenId,
        amount: Uint128,
        ) -> StdResult<CosmosMsg> {

        let mint_msg = Cw1155ExecuteMsg::Mint{to,token_id,amount};
        Ok(self.call(mint_msg)?)
    }
    
    pub fn mint_batch_for (
        &self,
        to: String,
        batch: Vec<(TokenId, Uint128)>,
        ) -> StdResult<CosmosMsg> {
            
        let mint_msg = Cw1155ExecuteMsg::BatchMint{to,batch};
        Ok(self.call(mint_msg)?)
    }

    pub fn burn_from (
        &self,
        from: String,
        token_id: TokenId,
        amount: Uint128,
        ) -> StdResult<CosmosMsg> {

        let burn_msg = Cw1155ExecuteMsg::Burn{from,token_id,amount};
        Ok(self.call(burn_msg)?)
    }

    pub fn burn_batch_from (
        &self,
        from: String,
        batch: Vec<(TokenId, Uint128)>,
        ) -> StdResult<CosmosMsg> {

        let burn_msg = Cw1155ExecuteMsg::BatchBurn{from,batch};
        Ok(self.call(burn_msg)?)
    }

    pub fn transfer (
        &self,
        to: String,
        token_id: TokenId,
        amount: Uint128,
    ) -> StdResult<CosmosMsg> {

        let transfer_msg = Cw1155ExecuteMsg::Transfer{to,token_id,amount};
        Ok(self.call(transfer_msg)?)
    }
    
    pub fn transfer_from (
        &self,
        from: String,
        to: String,
        token_id: TokenId,
        amount: Uint128,
    ) -> StdResult<CosmosMsg> {

        let transfer_msg = Cw1155ExecuteMsg::TransferFrom{from,to,token_id,amount};
        Ok(self.call(transfer_msg)?)
    }

    pub fn transfer_batch (
        &self,
        to : String,
        batch: Vec<(TokenId, Uint128)>,
    ) -> StdResult<CosmosMsg> {

        let transfer_msg = Cw1155ExecuteMsg::BatchTransfer{to,batch};
        Ok(self.call(transfer_msg)?)
    }

    pub fn transfer_batch_from (
        &self,
        from: String,
        to : String,
        batch: Vec<(TokenId, Uint128)>
    ) -> StdResult<CosmosMsg> {

        let transfer_msg = Cw1155ExecuteMsg::BatchTransferFrom{from,to,batch};
        Ok(self.call(transfer_msg)?)
    }

    fn send (
        &self,
        contract: String,
        token_id: TokenId,
        amount: Uint128,
        msg: Binary,
    ) -> StdResult<CosmosMsg> {
        
        let transfer_msg = Cw1155ExecuteMsg::Send {contract,token_id,amount,msg};
        Ok(self.call(transfer_msg)?)
    }

    fn send_from (
        &self,
        from: String,
        contract: String,
        token_id: TokenId,
        amount: Uint128,
        msg: Binary,
    ) -> StdResult<CosmosMsg> {
        
        let transfer_msg = Cw1155ExecuteMsg::SendFrom{from,contract,token_id,amount,msg};
        Ok(self.call(transfer_msg)?)
    }

    fn send_batch (
        &self,
        contract : String,
        batch: Vec<(TokenId, Uint128)>,
        msg: Binary,
    ) -> StdResult<CosmosMsg> {
        
        let transfer_msg = Cw1155ExecuteMsg::BatchSend {contract,batch,msg};
        Ok(self.call(transfer_msg)?)
    }

    fn send_batch_from (
        &self,
        from: String,
        contract : String,
        batch: Vec<(TokenId, Uint128)>,
        msg: Binary,
    ) -> StdResult<CosmosMsg> {
        
        let transfer_msg = Cw1155ExecuteMsg::BatchSendFrom {from,contract,batch,msg};
        Ok(self.call(transfer_msg)?)
    }

}