use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, QuerierWrapper, StdResult, Uint128,
    WasmMsg, WasmQuery,Binary,
};
use serde::de::DeserializeOwned;
use crate::{
    AllowanceResponse, BalanceResponse, Cw20ExecuteMsg, Cw20QueryMsg,
    TokenInfoResponse,TokenSupplyResponse,
};

/// Cw20Contract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
///
/// If you wish to persist this, convert to Cw20CanonicalContract via .canonical()
#[cw_serde]
pub struct Cw20Contract(pub Addr);

impl Cw20Contract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call(&self, msg: Cw20ExecuteMsg) -> StdResult<CosmosMsg> {
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
        req: Cw20QueryMsg,
    ) -> StdResult<T> {
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&req)?,
        }
        .into();
        querier.query(&query)
    }

    /// Get token balance for the given address
    pub fn balance(
        &self, 
        querier: &QuerierWrapper, 
        address: String
    ) 
    -> StdResult<BalanceResponse> {
        let req = Cw20QueryMsg::Balance {address};
        self.query(querier,req)
    }

    /// Get metadata from the contract. This is a good check that the address
    /// is a valid Cw20 contract.
    pub fn meta (
        &self,
        querier: &QuerierWrapper,
    ) -> StdResult<TokenInfoResponse> {
        let req = Cw20QueryMsg::TokenInfo {};
        self.query(querier,req)
    }

    /// Get allowance of spender to use owner's account
    pub fn allowance (
        &self,
        querier: &QuerierWrapper,
        owner: String,
        spender: String,
    ) -> StdResult<AllowanceResponse>{
        let req = Cw20QueryMsg::Allowance {owner,spender};
        self.query(querier,req)
    }
 
    pub fn total_supply (
        &self,
        querier: &QuerierWrapper,
    ) -> StdResult<TokenSupplyResponse>{
        let req = Cw20QueryMsg::TokenSupply {};
        self.query(querier,req)
    }

    /*Caller*/
    pub fn mint_for(
        &self,
        to : String,
        amount : Uint128,
        ) -> StdResult<CosmosMsg> {
        let mint_msg = Cw20ExecuteMsg::Mint{to,amount,};
        Ok(self.call(mint_msg)?)
    }

    pub fn burn_from(
        &self,
        from : String,
        amount : Uint128,
        ) -> StdResult<CosmosMsg> {

        let burn_msg = Cw20ExecuteMsg::BurnFrom{from ,amount};
        Ok(self.call(burn_msg)?)
    }

    pub fn burn(
        &self,
        amount : Uint128,
        ) -> StdResult<CosmosMsg> {

        let burn_msg = Cw20ExecuteMsg::Burn{amount};
        Ok(self.call(burn_msg)?)
    }
    
    pub fn transfer(
        &self,
        to: String,
        amount: Uint128,
    ) -> StdResult<CosmosMsg> {

        let transfer_msg = Cw20ExecuteMsg::Transfer{to,amount};
        Ok(self.call(transfer_msg)?)
    }
    
    pub fn sent(
        &self,
        contract: String,
        amount: Uint128,
        msg: Binary,
    ) -> StdResult<CosmosMsg> {

        let send_msg = Cw20ExecuteMsg::Send{
            contract: contract,
            amount: amount,
            msg : msg,
        };
        Ok(self.call(send_msg)?)
    }

    pub fn transfer_from(
        &self,
        from: String,
        to: String,
        amount: Uint128,
    ) -> StdResult<CosmosMsg> {

        let transfer_msg = Cw20ExecuteMsg::TransferFrom{from,to,amount};
        Ok(self.call(transfer_msg)?)
    }
    
    pub fn sent_from(
        &self,
        from: String,
        contract: String,
        amount: Uint128,
        msg: Binary,
    ) -> StdResult<CosmosMsg> {

        let send_msg = Cw20ExecuteMsg::SendFrom{
            from: from,
            contract: contract,
            amount: amount,
            msg : msg,
        };
        Ok(self.call(send_msg)?)
    }
    
}
