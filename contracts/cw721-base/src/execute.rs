use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{Binary,Addr, CustomMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,Uint128,Attribute,attr};

use cw2::set_contract_version;
use cw721::{Cw721Execute,Cw721ExecuteMsg, Cw721ReceiveMsg, Expiration};

use crate::error::ContractError;
use crate::msg::{InstantiateMsg};
use crate::state::{Cw721Contract, TokenInfo,ContractInfo,CooperativeData};

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:cw721-base";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a, T, C, E, Q> Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response<C>> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let contract_info = ContractInfo {
            name: msg.name,
            symbol: msg.symbol,
            owner : info.sender,
        };
        self.contract_info.save(deps.storage, &contract_info)?;
        self.max_supply.save(deps.storage,&msg.max_supply.into())?;
        Ok(Response::default())
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw721ExecuteMsg<T, E>,
    ) -> Result<Response<C>, ContractError> {
        match msg {

            Cw721ExecuteMsg::ChangeOwner { new_owner} => {self.change_owner(deps, info, new_owner)}

            Cw721ExecuteMsg::ApproveAll { spender, expires } => {self.approve_all(deps, env, info, spender, expires)}

            Cw721ExecuteMsg::RevokeAll { spender } => self.revoke_all(deps, env, info, spender),

            Cw721ExecuteMsg::Transfer {to,token_id} => self.transfer(deps, env, info, to, token_id),

            Cw721ExecuteMsg::TransferFrom { from, to,token_id } => self.transfer_from(deps, env, info, from,to,token_id),

            Cw721ExecuteMsg::Send {contract,token_id,msg} => self.send(deps, env, info, contract, token_id, msg),

            Cw721ExecuteMsg::SendFrom {from, contract,token_id, msg} => self.send_from(deps, env, info, from,contract,token_id,msg),

            Cw721ExecuteMsg::UpdateTokenURI {token_id , token_uri } => self.update_token_uri(deps, info, token_id,token_uri), 
            
            Cw721ExecuteMsg::UpdateTokenExtension {token_id , extension }=> self.update_token_extension(deps, info, token_id,extension), 
            
            Cw721ExecuteMsg::SetCooperative { cooperative , can_mint_for, can_burn_from} => self.set_cooperative(deps,info,cooperative,can_mint_for,can_burn_from),

            Cw721ExecuteMsg::UnsetCooperative { cooperative } => self.unset_cooperative(deps,info,cooperative),

            Cw721ExecuteMsg::UpdateMaxSupply { max_supply } => self.update_max_supply(deps,info,max_supply),
            
            Cw721ExecuteMsg::Mint{token_owner,token_uri,extension} => self.mint(deps, info, token_owner,token_uri,extension),

            Cw721ExecuteMsg::MintBatch{token_owner,token_uris,extensions} => self.mint_batch(deps,info,token_owner,token_uris,extensions),

            Cw721ExecuteMsg::BurnBatch { token_ids ,from_address} => self.burn_batch(deps, env,info,token_ids,from_address),

            Cw721ExecuteMsg::Burn { token_id,from_address } => self.burn(deps, env, info, token_id,from_address),
            
            Cw721ExecuteMsg::Extension { msg: _ } => Ok(Response::default()),
        }
    }
}

// TODO pull this into some sort of trait extension??
impl<'a, T, C, E, Q> Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn update_token_uri(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        token_id : String,
        token_uri: Option<String>,
    ) -> Result<Response<C>, ContractError> {
        
        if !self.is_contract_owner(deps.as_ref(),&info.sender) {
            if !self.check_as_cooperative(deps.as_ref(),&info.sender,true,false) {
                return Err(ContractError::Unauthorized {});
            }
        }

        self.tokens.update(deps.storage, &token_id, 
            |old| {
                match old {
                    Some(token_info) => {
                        Ok(TokenInfo{ 
                            token_uri : token_uri.clone(),
                            ..token_info
                        })},
                    None => { Err(ContractError::TokenNotFound{token_id : token_id.clone()})},
                }
         })?;

        Ok(Response::new()
            .add_attribute("action", "update_token_uri")
            .add_attribute("by", info.sender)
            .add_attribute("token_id", token_id.clone())
        )
    }

    pub fn update_token_extension(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        token_id : String,
        extension: T,
    ) -> Result<Response<C>, ContractError> {
        
        if !self.is_contract_owner(deps.as_ref(),&info.sender) {
            if !self.check_as_cooperative(deps.as_ref(),&info.sender,true,false) {
                return Err(ContractError::Unauthorized {});
            }
        }

        self.tokens.update(deps.storage, &token_id, 
            |old| {
                match old {
                    Some(token_info) => {
                        Ok(TokenInfo{ 
                            extension : extension,
                            ..token_info
                        })},
                    None => { Err(ContractError::TokenNotFound{token_id : token_id.clone()})},
                }
         })?;

        Ok(Response::new()
            .add_attribute("action", "update_token_uri")
            .add_attribute("by", info.sender)
            .add_attribute("token_id", token_id.clone())
        )
    }

    pub fn set_cooperative(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        cooperative: String,
        can_mint_for : bool,
        can_burn_from : bool,
    ) -> Result<Response<C>, ContractError> {
        
        let contract_info = self.contract_info.may_load(deps.storage)?.ok_or(ContractError::Unauthorized {})?;
        //Check is token contract owner account
        if contract_info.owner != info.sender {
            return Err(ContractError::Unauthorized {});
        }
    
        let cooperative_addr = deps.api.addr_validate(&cooperative)?;
        let cooperative_data = CooperativeData{
            can_mint_for : can_mint_for,
            can_burn_from : can_burn_from,
        };
        self.cooperatives.save(deps.storage,&cooperative_addr,&cooperative_data)?;
        let res = Response::new()
            .add_attribute("action", "set_cooperative")
            .add_attribute("cooperative", cooperative_addr)
            .add_attribute("can_mint_for", can_mint_for.to_string())
            .add_attribute("can_burn_from", can_burn_from.to_string());
        Ok(res)
    }
    
    pub fn unset_cooperative(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        cooperative: String,
    ) -> Result<Response<C>, ContractError> {
        
        let contract_info = self.contract_info.may_load(deps.storage)?.ok_or(ContractError::Unauthorized {})?;
        //Check is token contract owner account
        if contract_info.owner != info.sender {
            return Err(ContractError::Unauthorized {});
        }
        
        let cooperative_addr = deps.api.addr_validate(&cooperative)?;
        self.cooperatives.remove(deps.storage,&cooperative_addr);
        let res = Response::new()
            .add_attribute("action", "unset_cooperative")
            .add_attribute("cooperative", cooperative_addr);
        Ok(res)
    }

    pub fn update_max_supply(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        max_supply : Uint128, 
    ) -> Result<Response<C>, ContractError> {
       
        let contract_info = self.contract_info.may_load(deps.storage)?.ok_or(ContractError::Unauthorized {})?;
        if contract_info.owner != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        let total_supply = self.get_total_supply(deps.storage)?;
        if max_supply < Uint128::from(total_supply) {
            return Err(ContractError::ExceedMaxSupply{});
        }

        self.max_supply.save(deps.storage,&max_supply.into())?;
        Ok(Response::new()
            .add_attribute("action", "update_max_supply")
            .add_attribute("by", info.sender.to_string())
            .add_attribute("max_supply", max_supply)
        )
    }

    pub fn change_owner(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        new_owner : String,

    ) -> Result<Response<C>, ContractError> {
       
        let new_owner_addr = deps.api.addr_validate(&new_owner)?;
        let mut contract_info = self.contract_info.may_load(deps.storage)?.ok_or(ContractError::Unauthorized {})?;
        //Only owner can do this.
        if contract_info.owner != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        contract_info.owner = new_owner_addr;
        self.contract_info.save(deps.storage,&contract_info)?;
        Ok(Response::new()
            .add_attribute("action", "change_owner")
            .add_attribute("new_owner", contract_info.owner.to_string())
        )
    }
}

impl<'a, T, C, E, Q> Cw721Execute<T, C> for Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    type Err = ContractError;

    fn transfer(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,
        token_id: String,
    ) -> Result<Response<C>, ContractError> {
        self._transfer(deps, &env,&info, &info.sender, &recipient, &token_id)?;

        Ok(Response::new()
            .add_attribute("action", "transfer")
            .add_attribute("from", info.sender)
            .add_attribute("recipient", recipient)
            .add_attribute("token_id", token_id))
    }

    fn send(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        contract: String,
        token_id: String,
        msg: Binary,
    ) -> Result<Response<C>, ContractError> {
        // Transfer token
        self._transfer(deps, &env,&info, &info.sender, &contract, &token_id)?;

        let send = Cw721ReceiveMsg {
            from : info.sender.to_string(),
            operator: info.sender.to_string(),
            token_id: token_id.clone(),
            msg,
        };

        // Send message
        Ok(Response::new()
            .add_message(send.into_cosmos_msg(contract.clone())?)
            .add_attribute("action", "send")
            .add_attribute("from", info.sender)
            .add_attribute("recipient", contract)
            .add_attribute("token_id", token_id))
    }

    fn transfer_from(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        from : String,
        recipient: String,
        token_id: String,
    ) -> Result<Response<C>, ContractError> {

        let from_addr = deps.api.addr_validate(&from)?;
        self._transfer(deps, &env,&info, &from_addr, &recipient, &token_id)?;

        Ok(Response::new()
            .add_attribute("action", "transfer_from")
            .add_attribute("from", from)
            .add_attribute("recipient", recipient)
            .add_attribute("token_id", token_id))
    }

    fn send_from(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        from : String,
        contract: String,
        token_id: String,
        msg: Binary,
    ) -> Result<Response<C>, ContractError> {
        // Transfer token
        let from_addr = deps.api.addr_validate(&from)?;
        self._transfer(deps, &env,&info, &from_addr, &contract, &token_id)?;

        let send = Cw721ReceiveMsg {
            from : from.clone(),
            operator: info.sender.to_string(),
            token_id: token_id.clone(),
            msg,
        };

        // Send message
        Ok(Response::new()
            .add_message(send.into_cosmos_msg(contract.clone())?)
            .add_attribute("action", "send_from")
            .add_attribute("from", from)
            .add_attribute("recipient", contract)
            .add_attribute("token_id", token_id))
    }

    fn approve_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, ContractError> {
        // reject expired data as invalid
        let expires = expires.unwrap_or_default();
        if expires.is_expired(&env.block) {
            return Err(ContractError::Expired {});
        }

        // set the operator for us
        let spender_addr = deps.api.addr_validate(&spender)?;
        self.spenders.save(deps.storage, (&info.sender, &spender_addr), &expires)?;

        Ok(Response::new()
            .add_attribute("action", "approve_all")
            .add_attribute("sender", info.sender)
            .add_attribute("spender", spender))
    }

    fn revoke_all(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        spender: String,
    ) -> Result<Response<C>, ContractError> {
        let spender_addr = deps.api.addr_validate(&spender)?;
        self.spenders.remove(deps.storage, (&info.sender, &spender_addr));

        Ok(Response::new()
            .add_attribute("action", "revoke_all")
            .add_attribute("sender", info.sender)
            .add_attribute("spender", spender))
    }

    fn burn(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        from_address : Option<String>,
    ) -> Result<Response<C>, ContractError> {

        if  !self.is_contract_owner(deps.as_ref(),&info.sender) {
            
            let token = self.tokens.load(deps.storage, &token_id)?;
            if !self.check_is_token_owner_ifneed(deps.as_ref(),&from_address,&token) {
                return Err(ContractError::Unauthorized{});
            }
            
            if !self.check_is_token_owner(&info.sender,&token){
                if !self.check_as_cooperative(deps.as_ref(),&info.sender,false,true) ||
                   !self.check_is_token_operator(deps.as_ref(),&env,&info,&token) {
                    return Err(ContractError::Unauthorized{});
                }

            }
        }

        self.tokens.remove(deps.storage, &token_id)?;
        
        let new_token_supply = self.get_total_supply(deps.storage)?.checked_sub(1).unwrap();
        self.total_supply.save(deps.storage, &new_token_supply)?;

        Ok(Response::new()
            .add_attribute("action", "burn")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

    fn burn_batch(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_ids: Vec<String>,
        from_address : Option<String>,
    ) -> Result<Response<C>, ContractError> {

        let amount_to_burn = token_ids.len();
        if  !self.is_contract_owner(deps.as_ref(),&info.sender) {
            
            if from_address.is_some(){
                let mut check_for_index = 0;
                loop {
                    if check_for_index >= amount_to_burn {
                        break;
                    }
    
                    let token = self.tokens.load(deps.storage, &token_ids[check_for_index])?;
                    if !self.check_is_token_owner_ifneed(deps.as_ref(),&from_address,&token) {
                        return Err(ContractError::Unauthorized{});
                    }

                    if !self.check_is_token_owner(&info.sender,&token){
                        if  !self.check_as_cooperative(deps.as_ref(),&info.sender,false,true) ||
                            !self.check_is_token_operator(deps.as_ref(),&env,&info,&token) {
                                return Err(ContractError::Unauthorized{});
                        }
                    }
                    check_for_index = check_for_index + 1;
                };
            }
        }

        let mut index = 0;
        let mut burnt_attrs : Vec<Attribute> = vec![];
        let mut total_supply = self.get_total_supply(deps.storage)?;
        if total_supply < (amount_to_burn as u128) {
            return Err(ContractError::TotalSupplyUnderflow{})
        }

        loop{
            if index >= amount_to_burn {
                break;
            }
            self.tokens.remove(deps.storage, &token_ids[index])?;
            burnt_attrs.push(attr(format!("token_id[{}]",index),&token_ids[index]));
            total_supply = total_supply - 1;
            index = index + 1;
        }
        self.total_supply.save(deps.storage, &total_supply)?;

        Ok(Response::new()
            .add_attribute("action", "burn")
            .add_attribute("sender", info.sender)
            .add_attributes(burnt_attrs))
    }

    fn mint(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        to : String,
        token_uri: Option<String>,
        extension: T,
    ) -> Result<Response<C>, ContractError> {

        if !self.is_contract_owner(deps.as_ref(),&info.sender) {
            if !self.check_as_cooperative(deps.as_ref(),&info.sender,true,false) {
                return Err(ContractError::Unauthorized {});
            }
        }

        let total_supply = self.get_total_supply(deps.storage)?;
        let max_supply = self.get_max_supply(deps.storage)?;

        if total_supply >= max_supply{
            return Err(ContractError::ExceedMaxSupply{});
        }

        // create the token
        let token = TokenInfo {
            owner: deps.api.addr_validate(&to)?,
            token_uri: token_uri,
            extension: extension,
        };

        let token_id = self.get_last_running_id(deps.storage)?;
        let token_id_str: String = token_id.to_string();

        //Token ID is generate and alway uniqued so just save instead updated.
        self.tokens.save(deps.storage, &token_id_str,&token)?;

        let new_total_supply = total_supply + 1;
        self.total_supply.save(deps.storage, &new_total_supply)?;
        
        let next_token_id = token_id + 1;
        self.token_running_id.save(deps.storage, &next_token_id)?;

        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("minter", info.sender)
            .add_attribute("token_owner", to)
            .add_attribute("token_id", token_id_str))
    }

    fn mint_batch(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        to : String,
        token_uris: Vec<Option<String>>,
        extensions: Vec<T>,
    ) -> Result<Response<C>, ContractError> {

        if !self.is_contract_owner(deps.as_ref(),&info.sender) {
            if !self.check_as_cooperative(deps.as_ref(),&info.sender,true,false) {
                return Err(ContractError::Unauthorized {});
            }
        }

        if token_uris.len() != extensions.len(){
            return Err(ContractError::BatchDimensionMismatch {}); 
        }

        let amount_to_mint = token_uris.len();
        let total_supply = self.get_total_supply(deps.storage)?;
        let max_supply = self.get_max_supply(deps.storage)?;

        let new_total_supply = total_supply + (amount_to_mint as u128);
        if new_total_supply >= max_supply {
            return Err(ContractError::ExceedMaxSupply{});
        }

        let mut index = 0;
        let mut mint_attrs : Vec<Attribute> = vec![];
        let mut token_id = self.get_last_running_id(deps.storage)?;
        loop {

            if index >= amount_to_mint {break;}
            let token_id_str : String = token_id.to_string();
            // create the token
            let token = TokenInfo {
                owner: deps.api.addr_validate(&to)?,
                token_uri: token_uris[index].clone(),
                extension: extensions[index].clone(),
            };
            self.tokens.save(deps.storage, &token_id_str,&token)?;

            mint_attrs.push(attr(format!("token_id[{}]",index),token_id_str));
 
            token_id = token_id + 1;
            index = index + 1;
       }
        self.total_supply.save(deps.storage, &new_total_supply)?;
        self.token_running_id.save(deps.storage, &token_id)?;

        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("minter", info.sender)
            .add_attribute("token_owner", to)
            .add_attributes(mint_attrs))
    }
}

// helpers
impl<'a, T, C, E, Q> Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn _transfer(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        from : &Addr,
        recipient: &str,
        token_id: &str,
    ) -> Result<TokenInfo<T>, ContractError> {
        
        let mut token = self.tokens.load(deps.storage, token_id)?;
        // ensure we have permissions
        self.check_can_send(deps.as_ref(), env,info, from, &token)?;
        // set owner and remove existing approvals
        token.owner = deps.api.addr_validate(recipient)?;
        //token.approvals = vec![];
        self.tokens.save(deps.storage, token_id, &token)?;
        Ok(token)
    }

    /// returns true iff the sender can execute approve or reject on the contract
    pub fn check_is_token_owner(
        &self,
        address_to_check : &Addr,
        token: &TokenInfo<T>,
    ) -> bool {return token.owner == (*address_to_check);}

    pub fn check_is_approved_to_send(
        &self,
        deps: Deps,
        env: &Env,
        address_to_check: &Addr,
        token: &TokenInfo<T>,
    ) -> bool {
        
        return self.spenders.may_load(deps.storage, (&token.owner, address_to_check)).map_or(false,
        |operator| {
            operator.map_or(false,|ex|{
                !ex.is_expired(&env.block)
            })
        });
    }

    pub fn check_is_token_operator(
        &self,
        deps: Deps,
        env: &Env,
        info: &MessageInfo,
        token: &TokenInfo<T>,
    ) -> bool {
        
        return self.spenders.may_load(deps.storage, (&token.owner, &info.sender)).map_or(false,
        |operator| {
            operator.map_or(false,|ex|{
                !ex.is_expired(&env.block)
            })
        });
    }

    /// returns true iff the sender can transfer ownership of the token
    pub fn check_can_send(
        &self,
        deps: Deps,
        env: &Env,
        info: &MessageInfo,
        from: &Addr,
        token: &TokenInfo<T>,
    ) -> Result<(), ContractError> {
        
        let mut transfer_from = false;
        //If from not same as sender
        if *from != info.sender {
            transfer_from = true;
            if !self.check_is_token_owner(from,&token){return Err(ContractError::NotTokenOwner{});} 
        }

        if !self.is_contract_owner(deps,&info.sender) 
        {
            if !self.check_is_token_owner(&info.sender,&token){
                if transfer_from && !self.check_as_cooperative(deps,&info.sender,false,true) {
                    return Err(ContractError::Unauthorized{});
                }
                if !self.check_is_token_operator(deps,&env,&info,&token) {
                    return Err(ContractError::Unauthorized{});
                }
            }
        }
        return Ok(());
    }

     /// returns true iff the sender can transfer ownership of the token
     pub fn check_as_cooperative(
        &self,
        deps: Deps,
        address_to_check : &Addr,
        for_mint : bool,
        for_burn : bool,
    ) -> bool {
        return self.cooperatives.may_load(deps.storage,address_to_check).map_or(
            false,
            |cooperative| {
                return cooperative.map_or(false,|coop|{
                    if for_mint && !coop.can_mint_for { return false;}
                    if for_burn && !coop.can_burn_from { return false;}
                    return true;
                });
            }
        );
    }

    pub fn check_is_token_owner_ifneed(
        &self,
        deps: Deps,
        address_to_check : &Option<String>,
        token: &TokenInfo<T>,
    ) -> bool {
        
        match address_to_check {
            Some(addr_str) => {
                return deps.api.addr_validate(addr_str).map_or(
                    false,
                    |addr| {addr == token.owner}
                );
            },
            None => {}
        };
        return true;
    }

    pub fn is_contract_owner(
        &self,
        deps: Deps,
        address_to_check: &Addr,
    ) -> bool {
        return self.contract_info.may_load(deps.storage).map_or(false,
            |contract_info|{
                contract_info.map_or(false,|cinfo|{
                    (*address_to_check) == cinfo.owner
                })
            });
        
    }
}
