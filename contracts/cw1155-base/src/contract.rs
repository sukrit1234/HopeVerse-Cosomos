use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, 
    StdResult, Uint128,Attribute,attr,
};
use cw_storage_plus::Bound;
use cw_utils::{maybe_addr};
use cw1155::{
    OperatorsResponse, BalanceResponse, BatchBalanceResponse,
    Cw1155BatchReceiveMsg, Cw1155ExecuteMsg, Cw1155QueryMsg, Cw1155ReceiveMsg, Expiration,TokenSupply,
    AllowanceResponse, TokenId, TokenInfoResponse, TokensResponse,AllBalanceResponse,TokenSupplyResponse,
    TokenSuppliesResponse,ContractInfoResponse,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use crate::state::{APPROVES, BALANCES, CONTRACT_INFO,TOKEN_RUNNING_NO,TOKEN_SUPPLIES,COOPERATIVES, TOKENS,ContractInfo,TransferAction,CooperativeData};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw1155-base";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 30;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    // check valid token info
    msg.validate()?;
    
    // store token info
    let data = ContractInfo {
        name: msg.name,
        symbol: msg.symbol,
        owner : info.sender.clone(),
    };
    CONTRACT_INFO.save(deps.storage, &data)?;
    
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Cw1155ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        
        Cw1155ExecuteMsg::ChangeOwner {new_owner} => execute_change_owner(deps,info,new_owner),

        Cw1155ExecuteMsg::Transfer {
            to,
            token_id,
            amount,
        } => execute_transfer(deps,env,info, to, token_id, amount),
 
        Cw1155ExecuteMsg::TransferFrom {
            from,
            to,
            token_id,
            amount,
        } => execute_transfer_from(deps,env,info, from, to, token_id, amount),
        
        Cw1155ExecuteMsg::BatchTransfer {
            to,
            batch,
        } => execute_batch_transfer(deps,env,info, to, batch),

        Cw1155ExecuteMsg::BatchTransferFrom {
            from,
            to,
            batch,
        } => execute_batch_transfer_from(deps,env,info, from, to, batch),

        Cw1155ExecuteMsg::Send {
            contract,
            token_id,
            amount,
            msg,
        } => execute_send(deps,env,info, contract, token_id, amount, msg),
 
        Cw1155ExecuteMsg::SendFrom {
            from,
            contract,
            token_id,
            amount,
            msg,
        } => execute_send_from(deps,env,info, from, contract, token_id, amount, msg),
        
        Cw1155ExecuteMsg::BatchSend {
            contract,
            batch,
            msg,
        } => execute_batch_send(deps,env,info, contract, batch, msg),

        Cw1155ExecuteMsg::BatchSendFrom {
            from,
            contract,
            batch,
            msg,
        } => execute_batch_send_from(deps,env,info, from, contract, batch, msg),
        
        Cw1155ExecuteMsg::Mint {to,token_id,amount} => execute_mint(deps,info, to, token_id, amount),

        Cw1155ExecuteMsg::BatchMint { to, batch } => execute_batch_mint(deps,info, to, batch),
        
        Cw1155ExecuteMsg::Burn {from,token_id,amount} => execute_burn(deps,env,info, from, token_id, amount),

        Cw1155ExecuteMsg::BatchBurn { from, batch } => execute_batch_burn(deps,env,info, from, batch),
        
        Cw1155ExecuteMsg::ApproveAll { operator, expires } => execute_approve_all(deps,env,info, operator, expires),
        
        Cw1155ExecuteMsg::RevokeAll { operator } => execute_revoke_all(deps,info, operator),

        Cw1155ExecuteMsg::DefineToken {token_uri,max_supply} => execute_define_token(deps,info,token_uri,max_supply),

        Cw1155ExecuteMsg::UpdateTokenUri {token_id,token_uri} => execute_update_token_uri(deps,info,token_id,token_uri),
    
        Cw1155ExecuteMsg::UpdateMaxSupply{token_id,max_supply} => execute_update_max_supply(deps,info,token_id,max_supply),
    
        Cw1155ExecuteMsg::SetCooperative { cooperative , can_mint_for  , can_burn_from } => execute_set_cooperative(deps,info , cooperative , can_mint_for  , can_burn_from),

        Cw1155ExecuteMsg::UnsetCooperative { cooperative} => execute_unset_cooperative(deps,info,cooperative),
    }
}


fn determine_transfer_action( 
    from: Option<&Addr>,
    to: Option<&Addr>,) -> TransferAction {

    if from.is_none() && to.is_none(){return TransferAction::None;}
        
    if from.is_none(){return TransferAction::Mint;}
    else if to.is_none() {return TransferAction::Burn;}
    return TransferAction::Transfer;
}

pub fn execute_change_owner(
    deps: DepsMut,
    info: MessageInfo,
    new_owner : String,
)-> Result<Response, ContractError> {
    
    let new_owner_addr = deps.api.addr_validate(&new_owner)?;
    let mut config = CONTRACT_INFO.load(deps.storage)?;
    
    //Only current owner can change new owner.
    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    config.owner = new_owner_addr;
    CONTRACT_INFO.save(deps.storage,&config)?;
    let res = Response::new()
    .add_attribute("action", "change_owner")
    .add_attribute("new_owner", config.owner.to_string());
    Ok(res)
}

/// When from is None: mint new coins
/// When to is None: burn coins
/// When both are None: no token balance is changed, pointless but valid
/// Make sure permissions are checked before calling this.
fn execute_transfer_internal(
    deps: DepsMut,
    from: Option<&Addr>,
    to: Option<&Addr>,
    token_id: &str,
    amount: Uint128,
) -> Result<Response, ContractError> {

    if !TOKENS.has(deps.storage, token_id) {
        return Err(ContractError::TokenUndefined{token_id:token_id.to_string()});
    }

    let action = determine_transfer_action(from,to);
    if action == TransferAction::None {
        return Err(ContractError::InvalidTransferAddress{});
    }

    if let Some(from_addr) = from {
        BALANCES.update(
            deps.storage,
            (from_addr, token_id),
            |balance: Option<Uint128>| -> StdResult<_> {
                Ok(balance.unwrap_or_default().checked_sub(amount)?)
            },
        )?;
    }

    if let Some(to_addr) = to {
        BALANCES.update(
            deps.storage,
            (to_addr, token_id),
            |balance: Option<Uint128>| -> StdResult<_> {
                Ok(balance.unwrap_or_default().checked_add(amount)?)
            },
        )?;
    }

    
    if action != TransferAction::Transfer{
        let token_supply = TOKEN_SUPPLIES.load(deps.storage,&token_id)?;
        if action == TransferAction::Mint {
            let new_total_supply = token_supply.total_supply.checked_add(amount).unwrap();
            if new_total_supply > token_supply.max_supply{
                return Err(ContractError::ExceedMaxSupply{});
            }
            let new_token_supply = TokenSupply{
                total_supply : new_total_supply,
                max_supply : token_supply.max_supply,
            };
            TOKEN_SUPPLIES.save(deps.storage,&token_id,&new_token_supply)?;
        }
        else if action == TransferAction::Burn {
            let new_token_supply = TokenSupply{
                total_supply : token_supply.total_supply.checked_sub(amount).unwrap(),
                max_supply : token_supply.max_supply,
            };
            TOKEN_SUPPLIES.save(deps.storage,&token_id,&new_token_supply)?;
        }
    }

    let mut resp_attrs : Vec<Attribute> = vec![];
    resp_attrs.push(attr("action", action.to_string()));
    resp_attrs.push(attr("token_id", token_id));
    resp_attrs.push(attr("amount", amount));
    if let Some(_from) = from.clone() {
        resp_attrs.push(attr("from", _from));
    }
    if let Some(_to) = to.clone() {
        resp_attrs.push(attr("to", _to));
    }
    Ok(Response::new().add_attributes(resp_attrs))
}

fn execute_transfer_batch_internal(
    deps: DepsMut,
    from: Option<&Addr>,
    to: Option<&Addr>,
    batch: &Vec<(TokenId, Uint128)>,
) -> Result<Response, ContractError> {
   
    let mut index : u32 = 0;
    let mut resp_attrs : Vec<Attribute> = vec![];

    let action = determine_transfer_action(from,to);
    if action == TransferAction::None {
        return Err(ContractError::InvalidTransferAddress{});
    }

    resp_attrs.push(attr("action", action.to_string()));
    for (token_id, amount) in batch.into_iter()
    {
        if !TOKENS.has(deps.storage, token_id) {
            return Err(ContractError::TokenUndefined{token_id : token_id.to_string()});
        }

        let amt = *amount;
        if let Some(from_addr) = from {
            BALANCES.update(
                deps.storage,
                (from_addr, token_id),
                |balance: Option<Uint128>| -> StdResult<_> {
                    Ok(balance.unwrap_or_default().checked_sub(amt)?)
                },
            )?;
        }

        if let Some(to_addr) = to {
            BALANCES.update(
                deps.storage,
                (to_addr, token_id),
                |balance: Option<Uint128>| -> StdResult<_> {
                    Ok(balance.unwrap_or_default().checked_add(amt)?)
                },
            )?;
        }

        if action != TransferAction::Transfer{
            let token_supply = TOKEN_SUPPLIES.load(deps.storage,&token_id)?;
            if action == TransferAction::Mint {
                let new_total_supply = token_supply.total_supply.checked_add(amt).unwrap();
                if new_total_supply > token_supply.max_supply{
                    return Err(ContractError::ExceedMaxSupply{});
                }
                let new_token_supply = TokenSupply{
                    total_supply : new_total_supply,
                    max_supply : token_supply.max_supply,
                };
                TOKEN_SUPPLIES.save(deps.storage,&token_id,&new_token_supply)?;
            }
            else if action == TransferAction::Burn {
                let new_token_supply = TokenSupply{
                    total_supply : token_supply.total_supply.checked_sub(amt).unwrap(),
                    max_supply : token_supply.max_supply,
                };
                TOKEN_SUPPLIES.save(deps.storage,&token_id,&new_token_supply)?;
            }
        }

        resp_attrs.push(attr(format!("token_id[{}]",index),token_id));
        resp_attrs.push(attr(format!("amount[{}]",index),amt));
        index = index+1;
    }

    if let Some(_from) = from.clone() {resp_attrs.push(attr("from", _from));}
    if let Some(_to) = to.clone() {resp_attrs.push(attr("to", _to));}
    Ok(Response::new().add_attributes(resp_attrs))
}


/// returns true iff the sender can execute approve or reject on the contract
fn check_can_approve(deps: Deps,env : &Env, owner: &Addr, operator: &Addr) -> bool {
    if owner == operator { return true };
    return APPROVES.may_load(deps.storage, (&owner, &operator)).map_or(false,
        |operator| {
            operator.map_or(false,|ex|{
                !ex.is_expired(&env.block)
            })
        });
}

fn check_can_send(
    deps: Deps,
    env: &Env,
    from: &Addr,
    sender: &Addr,
) -> Result<(), ContractError> {
        if !check_as_contract_owner(deps,&sender) 
        {
            if *from != *sender {
                if !check_as_cooperative(deps,&sender,false,false) {
                    return Err(ContractError::Unauthorized{});
                }
            }
            if !check_can_approve(deps, env, &from, &sender){
                return Err(ContractError::Unauthorized{});
            }
            
        }
        return Ok(());   
}

pub fn execute_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    to : String,
    token_id: TokenId,
    amount: Uint128,
) -> Result<Response, ContractError> {

    check_can_send(deps.as_ref(), &env, &info.sender, &info.sender)?;
    
    let to_addr = deps.api.addr_validate(&to)?;
    Ok(execute_transfer_internal( deps,Some(&info.sender),Some(&to_addr),&token_id,amount)?)
}

pub fn execute_send(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contract : String,
    token_id: TokenId,
    amount: Uint128,
    msg: Binary,
) -> Result<Response, ContractError> {

    check_can_send(deps.as_ref(), &env, &info.sender, &info.sender)?;

    let to_addr = deps.api.addr_validate(&contract)?;
    let rsp = execute_transfer_internal( deps,Some(&info.sender),Some(&to_addr),&token_id,amount)?;
    let send = Cw1155ReceiveMsg {
        operator: info.sender.to_string(),
        from: Some(info.sender.to_string()),
        amount,
        token_id: token_id.clone(),
        msg,
    };
    Ok(rsp.add_message(send.into_cosmos_msg(contract.clone())?))
}

pub fn execute_transfer_from(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    from: String,
    to : String,
    token_id: TokenId,
    amount: Uint128,
) -> Result<Response, ContractError> {

    let from_addr = deps.api.addr_validate(&from)?;
    
    check_can_send(deps.as_ref(), &env, &from_addr, &info.sender)?;

    let to_addr = deps.api.addr_validate(&to)?;
    Ok(execute_transfer_internal( deps,Some(&from_addr),Some(&to_addr),&token_id,amount)?)
}

pub fn execute_send_from(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    from: String,
    contract : String,
    token_id: TokenId,
    amount: Uint128,
    msg: Binary,
) -> Result<Response, ContractError> {

    let from_addr = deps.api.addr_validate(&from)?;
    
    check_can_send(deps.as_ref(), &env, &from_addr, &info.sender)?;

    let to_addr = deps.api.addr_validate(&contract)?;
    let rsp = execute_transfer_internal( deps,Some(&from_addr),Some(&to_addr),&token_id,amount)?;
    let send = Cw1155ReceiveMsg {
        operator: info.sender.to_string(),
        from: Some(from),
        amount,
        token_id: token_id.clone(),
        msg,
    };
    Ok(rsp.add_message(send.into_cosmos_msg(contract.clone())?))
}


pub fn check_as_cooperative(
    deps: Deps,
    address_to_check : &Addr,
    for_mint : bool,
    for_burn : bool,
) -> bool {
    return COOPERATIVES.may_load(deps.storage,address_to_check).map_or(
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
pub fn check_as_contract_owner(
    deps: Deps,
    address_to_check: &Addr,
) -> bool {
    return CONTRACT_INFO.may_load(deps.storage).map_or(false,
        |token_info_option|{
            token_info_option.map_or(false,|token_info|{
                (*address_to_check) == token_info.owner
            })
        });
    
}

pub fn execute_mint(
    deps: DepsMut,
    info: MessageInfo,
    to: String,
    token_id: TokenId,
    amount: Uint128,
) -> Result<Response, ContractError> {
    
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    if !check_as_contract_owner(deps.as_ref(),&info.sender) {
        if !check_as_cooperative(deps.as_ref(),&info.sender,true,false) {
            return Err(ContractError::Unauthorized {});
        }
    }

    let to_addr = deps.api.addr_validate(&to)?;
    Ok(execute_transfer_internal(deps, None, Some(&to_addr), &token_id, amount)?)
}

pub fn execute_burn(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    from: String,
    token_id: TokenId,
    amount: Uint128,
) -> Result<Response, ContractError> {

    let from_addr = deps.api.addr_validate(&from)?;
    if !check_as_contract_owner(deps.as_ref(),&info.sender) {
        if from_addr != info.sender{
            if !check_as_cooperative(deps.as_ref(),&info.sender,false,true) ||
               !check_can_approve(deps.as_ref(),&env,&from_addr,&info.sender) {
                return Err(ContractError::Unauthorized{});
            }
        }
    }
    // whoever can transfer these tokens can burn
    Ok(execute_transfer_internal(deps, Some(&from_addr), None, &token_id, amount)?)
}

pub fn execute_batch_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    to : String,
    batch: Vec<(TokenId, Uint128)>,
) -> Result<Response, ContractError> {
    
    check_can_send(deps.as_ref(), &env, &info.sender, &info.sender)?;
    let to_addr = deps.api.addr_validate(&to)?;
    Ok(execute_transfer_batch_internal(deps,Some(&info.sender),Some(&to_addr),&batch)?)
}

pub fn execute_batch_transfer_from(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    from: String,
    to : String,
    batch: Vec<(TokenId, Uint128)>,
) -> Result<Response, ContractError> {
    
    let from_addr = deps.api.addr_validate(&from)?;

    check_can_send(deps.as_ref(), &env, &from_addr, &info.sender)?;

    let to_addr = deps.api.addr_validate(&to)?;
    Ok(execute_transfer_batch_internal(deps,Some(&from_addr),Some(&to_addr),&batch)?)
}

pub fn execute_batch_send(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contract : String,
    batch: Vec<(TokenId, Uint128)>,
    msg: Binary,
) -> Result<Response, ContractError> {
    
    let to_addr = deps.api.addr_validate(&contract)?;

    check_can_send(deps.as_ref(), &env, &info.sender, &info.sender)?;

    let rsp = execute_transfer_batch_internal(deps,Some(&info.sender),Some(&to_addr),&batch)?;
    let send = Cw1155BatchReceiveMsg {
        operator: info.sender.to_string(),
        from: Some(info.sender.to_string()),
        batch,
        msg,
    };
    Ok(rsp.add_message(send.into_cosmos_msg(contract.clone())?))
}

pub fn execute_batch_send_from(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    from: String,
    contract : String,
    batch: Vec<(TokenId, Uint128)>,
    msg: Binary,
) -> Result<Response, ContractError> {
    
    let from_addr = deps.api.addr_validate(&from)?;
    let to_addr = deps.api.addr_validate(&contract)?;

    check_can_send(deps.as_ref(), &env, &from_addr, &info.sender)?;

    let rsp = execute_transfer_batch_internal(deps,Some(&from_addr),Some(&to_addr),&batch)?;
    let send = Cw1155BatchReceiveMsg {
        operator: info.sender.to_string(),
        from: Some(from),
        batch,
        msg,
    };
    Ok(rsp.add_message(send.into_cosmos_msg(contract.clone())?))
}

pub fn execute_batch_mint(
    deps: DepsMut,
    info: MessageInfo,
    to: String,
    batch: Vec<(TokenId, Uint128)>,
) -> Result<Response, ContractError> {

    if !check_as_contract_owner(deps.as_ref(),&info.sender) {
        if !check_as_cooperative(deps.as_ref(),&info.sender,true,false) {
            return Err(ContractError::Unauthorized {});
        }
    }

    let to_addr = deps.api.addr_validate(&to)?;
    Ok(execute_transfer_batch_internal(deps, None, Some(&to_addr), &batch)?)
}

pub fn execute_batch_burn(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    from: String,
    batch: Vec<(TokenId, Uint128)>,
) -> Result<Response, ContractError> {

    let from_addr = deps.api.addr_validate(&from)?;
    if !check_as_contract_owner(deps.as_ref(),&info.sender) {
        if from_addr != info.sender{
            if !check_as_cooperative(deps.as_ref(),&info.sender,false,true) ||
               !check_can_approve(deps.as_ref(),&env,&from_addr,&info.sender) {
                return Err(ContractError::Unauthorized{});
            }
        }
    }
    Ok(execute_transfer_batch_internal(deps, Some(&from_addr), None, &batch)?)
}

pub fn execute_approve_all(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    operator: String,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {

    // reject expired data as invalid
    let expires = expires.unwrap_or_default();
    if expires.is_expired(&env.block) {
        return Err(ContractError::Expired {});
    }

    // set the operator for us
    let operator_addr = deps.api.addr_validate(&operator)?;
    APPROVES.save(deps.storage, (&info.sender, &operator_addr), &expires)?;

    Ok(Response::new()
        .add_attributes(vec![
                        attr("sender",info.sender),
                        attr("operator",operator)
                    ])
    )
}

pub fn execute_revoke_all(
    deps: DepsMut,
    info: MessageInfo,
    operator: String
) -> Result<Response, ContractError> {

    let operator_addr = deps.api.addr_validate(&operator)?;
    APPROVES.remove(deps.storage, (&info.sender, &operator_addr));
    Ok(Response::new().add_attributes(vec![
        attr("sender",info.sender),
        attr("operator",operator),
    ]))
}


pub fn execute_define_token(
    deps: DepsMut,
    info: MessageInfo,
    token_uri: String,
    max_supply : Uint128,
) -> Result<Response, ContractError> {

    if !check_as_contract_owner(deps.as_ref(),&info.sender){
        return Err(ContractError::Unauthorized{});
    }
    
    let mut token_id = TOKEN_RUNNING_NO.may_load(deps.storage)?.unwrap_or_default();
    let token_id_str: String = token_id.to_string();
    TOKENS.save(deps.storage,&token_id_str,&token_uri)?;

    token_id = token_id + 1;
    TOKEN_RUNNING_NO.save(deps.storage,&token_id)?;
    
    let token_supply = TokenSupply{
        total_supply : Uint128::from(0u128),
        max_supply : max_supply,
    };
    TOKEN_SUPPLIES.save(deps.storage,&token_id_str,&token_supply)?;

    Ok(Response::new().add_attributes(vec![
        attr("action","define_token"),
        attr("creator",info.sender.to_string()),
        attr("token_id",token_id_str),
    ]))
}

pub fn execute_update_token_uri(
    deps: DepsMut,
    info: MessageInfo,
    token_id : String,
    token_uri: String,
) -> Result<Response, ContractError> {

    if !check_as_contract_owner(deps.as_ref(),&info.sender){
        return Err(ContractError::Unauthorized{});
    }
    if !TOKENS.has(deps.storage,&token_id){
        return Err(ContractError::TokenUndefined{token_id:token_id});
    }

    TOKENS.save(deps.storage,&token_id,&token_uri)?;
    Ok(Response::new().add_attributes(vec![
        attr("action","update_token_uri"),
        attr("token_id",token_id),
        attr("token_uri",token_uri),
    ]))
}

pub fn execute_update_max_supply(
    deps: DepsMut,
    info: MessageInfo,
    token_id : String,
    max_supply: Uint128,
) -> Result<Response, ContractError> {

    if !check_as_contract_owner(deps.as_ref(),&info.sender){
        return Err(ContractError::Unauthorized{});
    }
    if !TOKENS.has(deps.storage,&token_id){
        return Err(ContractError::TokenUndefined{token_id:token_id});
    }

    let token_supply = TOKEN_SUPPLIES.load(deps.storage,&token_id)?;
    if max_supply < token_supply.total_supply {
        return Err(ContractError::ExceedMaxSupply{});
    }

    let new_token_supply = TokenSupply{
        max_supply : max_supply,
        ..token_supply
    };
    TOKEN_SUPPLIES.save(deps.storage,&token_id,&new_token_supply)?;
    
    Ok(Response::new().add_attributes(vec![
        attr("action","update_max_supply"),
        attr("token_id",token_id),
        attr("max_supply",max_supply),
    ]))
}

pub fn execute_set_cooperative(
    deps: DepsMut,
    info: MessageInfo,
    cooperative: String,
    can_mint_for : bool,
    can_burn_from : bool,

) -> Result<Response, ContractError> {
    
    if !check_as_contract_owner(deps.as_ref(),&info.sender){
        return Err(ContractError::Unauthorized {});
    }
    let cooperative_addr = deps.api.addr_validate(&cooperative)?;
    let cooperative_data = CooperativeData{
        can_mint_for : can_mint_for,
        can_burn_from : can_burn_from,
    };
    COOPERATIVES.save(deps.storage,&cooperative_addr,&cooperative_data)?;
    let res = Response::new()
        .add_attribute("action", "set_cooperative")
        .add_attribute("cooperative", cooperative_addr)
        .add_attribute("can_mint_for", can_mint_for.to_string())
        .add_attribute("can_burn_from", can_burn_from.to_string());

    Ok(res)
}

pub fn execute_unset_cooperative(
    deps: DepsMut,
    info: MessageInfo,
    cooperative: String,
) -> Result<Response, ContractError> {
    
    if !check_as_contract_owner(deps.as_ref(),&info.sender){
        return Err(ContractError::Unauthorized {});
    }
    
    let cooperative_addr = deps.api.addr_validate(&cooperative)?;
    COOPERATIVES.remove(deps.storage,&cooperative_addr);
    let res = Response::new()
        .add_attribute("action", "unset_cooperative")
        .add_attribute("cooperative", cooperative_addr);
    Ok(res)
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: Cw1155QueryMsg) -> StdResult<Binary> {
    match msg {
        Cw1155QueryMsg::Balance { owner, token_id } => {
            to_binary(&query_balance(deps,owner,token_id)?)
        }
        Cw1155QueryMsg::BatchBalance { owner, token_ids } => {
            to_binary(&query_batch_balance(deps,owner,token_ids)?)
        }
        Cw1155QueryMsg::AllBalance{owner} =>{
            to_binary(&query_all_balance(deps,owner)?)
        }

        Cw1155QueryMsg::ContractInfo {} => {
            to_binary(&query_contract_info(deps)?)
        }

        Cw1155QueryMsg::Allowance { owner, operator } => {
            to_binary(&query_allowance(deps,env,owner, operator)?)
        }
        Cw1155QueryMsg::AllOperators {owner,include_expired,start_after,limit,} => {
            to_binary(&query_all_approvals(deps,env,owner,include_expired,start_after,limit)?)
        }
        Cw1155QueryMsg::TokenInfo { token_id } => {
            to_binary(&query_token_info(deps,token_id)?)
        }
        Cw1155QueryMsg::Tokens { owner,start_after,limit} => {
            to_binary(&query_tokens(deps, owner, start_after, limit)?)
        }
        Cw1155QueryMsg::AllTokens { start_after, limit } => {
            to_binary(&query_all_tokens(deps, start_after, limit)?)
        }
        Cw1155QueryMsg::TokenSupply {token_id} => {
            to_binary(&query_token_supply(deps, token_id)?)
        }

        Cw1155QueryMsg::TokenSupplies {token_ids} => {
            to_binary(&query_batch_token_supply(deps, token_ids)?)
        },

        Cw1155QueryMsg::LastTokenID {} => {
            to_binary(&query_last_token_id(deps)?)
        }
    }
}

fn parse_approval(item: StdResult<(Addr,Expiration)>) -> StdResult<cw1155::Approval> {
    item.map(|(spender, expires)| cw1155::Approval {
        spender: spender.to_string(),
        expires,
    })
}

fn query_all_approvals(
    deps: Deps,
    env: Env,
    owner: String,
    include_expired: bool,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<OperatorsResponse> {
    
    let owner_addr = deps.api.addr_validate(&owner)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    
    let start_addr = maybe_addr(deps.api, start_after)?;
    let start = start_addr.as_ref().map(Bound::exclusive);

    let operators = APPROVES
        .prefix(&owner_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .filter(|r| include_expired || r.is_err() || !r.as_ref().unwrap().1.is_expired(&env.block))
        .take(limit)
        .map(parse_approval)
        .collect::<StdResult<_>>()?;
    Ok(OperatorsResponse { operators })
}

fn query_tokens(
    deps: Deps,
    owner: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokensResponse> {
    
    let owner_addr = deps.api.addr_validate(&owner)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

    let tokens = BALANCES
        .prefix(&owner_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(k, _)| k.to_string()))
        .collect::<StdResult<_>>()?;
    Ok(TokensResponse { tokens })
}

fn query_all_tokens(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokensResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    
    let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));
    let tokens = TOKENS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(k, _)| k.to_string()))
        .collect::<StdResult<_>>()?;
    Ok(TokensResponse { tokens })
}

fn query_last_token_id(
    deps: Deps,
) -> StdResult<u128> {
    Ok(TOKEN_RUNNING_NO.may_load(deps.storage)?.unwrap_or_default())
}

fn query_balance(
   deps: Deps,
   owner : String,
   token_id : String,
) -> StdResult<BalanceResponse> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let balance = BALANCES.may_load(deps.storage, (&owner_addr, &token_id))?.unwrap_or_default();
    Ok(BalanceResponse { balance })
}

fn query_batch_balance(
    deps: Deps,
    owner : String,
    token_ids : Vec<TokenId>,
 ) -> StdResult<BatchBalanceResponse> {

    let owner_addr = deps.api.addr_validate(&owner)?;
    let balances = token_ids
        .into_iter()
        .map(|token_id| -> StdResult<_> {
            Ok(BALANCES
                .may_load(deps.storage, (&owner_addr, &token_id))?
                .unwrap_or_default())
        })
        .collect::<StdResult<_>>()?;
    Ok(BatchBalanceResponse { balances })
 }

 fn query_all_balance(
    deps: Deps,
    owner : String,
 ) -> StdResult<AllBalanceResponse> {

    let owner_addr = deps.api.addr_validate(&owner)?;
    
    let mut token_id = 0u128;
    let mut tokenids : Vec<String> = vec![];
    let mut amounts : Vec<Uint128> = vec![];
    
    let zero_balance = Uint128::from(0u128);
    let last_token_id = TOKEN_RUNNING_NO.may_load(deps.storage)?.unwrap_or_default();
    loop {
        if token_id >= last_token_id {break;}
        let token_id_str :String = token_id.to_string();
        let balance = BALANCES.may_load(deps.storage, (&owner_addr, &token_id_str))?.unwrap_or_default();
        if balance > zero_balance {
            tokenids.push(token_id_str);
            amounts.push(balance);
        }
        token_id = token_id + 1u128;
    }
    Ok(AllBalanceResponse { tokenids : tokenids,
                            amounts : amounts })
 }

 fn query_allowance(
    deps: Deps,
    env : Env,
    owner : String,
    operator : String,
 ) -> StdResult<AllowanceResponse> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let operator_addr = deps.api.addr_validate(&operator)?;
    let approved = check_can_approve(deps, &env, &owner_addr, &operator_addr);
    Ok(AllowanceResponse { approved })    
 }

 fn query_token_info(
    deps: Deps,
    token_id : String,
 ) -> StdResult<TokenInfoResponse> {
    let url = TOKENS.load(deps.storage, &token_id)?;
    Ok(TokenInfoResponse{ url })
 }
 
 fn query_token_supply(
    deps: Deps,
    token_id : String,
 ) -> StdResult<TokenSupplyResponse> {
    let supply = TOKEN_SUPPLIES.load(deps.storage, &token_id)?;
    Ok(TokenSupplyResponse{ supply})
 }

 fn query_batch_token_supply(
    deps: Deps,
    token_ids : Vec<TokenId>,
 ) -> StdResult<TokenSuppliesResponse> {
    let supplies = token_ids
        .into_iter()
        .map(|token_id| -> StdResult<_> {
            Ok(TOKEN_SUPPLIES.may_load(deps.storage, &token_id)?.unwrap_or_default())
        })
        .collect::<StdResult<_>>()?;
    Ok(TokenSuppliesResponse { supplies })
 }
 fn query_contract_info(
    deps: Deps,
 ) -> StdResult<ContractInfoResponse> {
    
    let info = CONTRACT_INFO.load(deps.storage)?;
    Ok( ContractInfoResponse{
        name: info.name,
        symbol: info.symbol,
        owner : info.owner.to_string(),
    })
 }

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use super::*;

    #[test]
    fn check_transfers() {
        // A long test case that try to cover as many cases as possible.
        // Summary of what it does:
        // - try mint without permission, fail
        // - mint with permission, success
        // - query balance of receipant, success
        // - try transfer without approval, fail
        // - approve
        // - transfer again, success
        // - query balance of transfer participants
        // - batch mint token2 and token3, success
        // - try batch transfer without approval, fail
        // - approve and try batch transfer again, success
        // - batch query balances
        // - user1 revoke approval to minter
        // - query approval status
        // - minter try to transfer, fail
        // - user1 burn token1
        // - user1 batch burn token2 and token3
        let minter = String::from("minter");
        let user1 = String::from("user1");
        let user2 = String::from("user2");

        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            name : String::from("NFT TOKEN TWO"),
            symbol : String::from("NFTTWO"),
        };

        let creator = mock_info("operator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(),creator , msg).unwrap();
        assert_eq!(0, res.messages.len());

        let creator = mock_info("operator", &[]);
        let res = execute(deps.as_mut(),mock_env(),creator,
            Cw1155ExecuteMsg::DefineToken {
                token_uri : String::from("www.token1.com"),
                max_supply : Uint128::from(10000u128),
            },
        ).unwrap();
        let token1 = res.attributes[2].value.clone();

        let creator = mock_info("operator", &[]);
        let res = execute(deps.as_mut(),mock_env(),creator,
            Cw1155ExecuteMsg::DefineToken {
                token_uri : String::from("www.token2.com"),
                max_supply : Uint128::from(10000u128),
            },
        ).unwrap();
        let token2 = res.attributes[2].value.clone();

        let creator = mock_info("operator", &[]);
        let res = execute(deps.as_mut(),mock_env(),creator,
            Cw1155ExecuteMsg::DefineToken {
                token_uri : String::from("www.token3.com"),
                max_supply : Uint128::from(10000u128),
            },
        ).unwrap();
        let token3 = res.attributes[2].value.clone();
        
        let mint_msg = Cw1155ExecuteMsg::Mint {
            to: user1.clone(),
            token_id: token1.clone(),
            amount: 1u64.into(),
        };
        assert!(matches!(
            execute(
                deps.as_mut(),
                mock_env(),
                mock_info(user1.as_ref(), &[]),
                mint_msg.clone(),
            ),
            Err(ContractError::Unauthorized {})
        ));

        let creator = mock_info("operator", &[]);
        // valid mint
        assert_eq!(
            execute(
                deps.as_mut(),
                mock_env(),
                creator,
                mint_msg,
            )
            .unwrap(),
            Response::new()
                .add_attribute("action", "mint")
                .add_attribute("token_id", &token1)
                .add_attribute("amount", 1u64.to_string())
                .add_attribute("to", &user1)
        );

        // query balance
        assert_eq!(
            to_binary(&BalanceResponse {
                balance: 1u64.into()
            }),
            query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::Balance {
                    owner: user1.clone(),
                    token_id: token1.clone(),
                }
            ),
        );

        let transfer_msg = Cw1155ExecuteMsg::TransferFrom {
            from: user1.clone(),
            to: user2.clone(),
            token_id: token1.clone(),
            amount: 1u64.into(),
        };

        // not approved yet
        assert!(matches!(
            execute(
                deps.as_mut(),
                mock_env(),
                mock_info(minter.as_ref(), &[]),
                transfer_msg.clone(),
            ),
            Err(ContractError::Unauthorized {})
        ));

        // approve
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(user1.as_ref(), &[]),
            Cw1155ExecuteMsg::ApproveAll {
                operator: minter.clone(),
                expires: None,
            },
        )
        .unwrap();

        //set as cooperative
        let creator = mock_info("operator", &[]);
        execute(
            deps.as_mut(),
            mock_env(),
            creator,
            Cw1155ExecuteMsg::SetCooperative {
                cooperative: minter.clone(),
                can_mint_for: true,
                can_burn_from: true,
            },
        )
        .unwrap();

        // transfer
        assert_eq!(
            execute(
                deps.as_mut(),
                mock_env(),
                mock_info(minter.as_ref(), &[]),
                transfer_msg,
            )
            .unwrap(),
            Response::new()
                .add_attribute("action", "transfer")
                .add_attribute("token_id", &token1)
                .add_attribute("amount", 1u64.to_string())
                .add_attribute("from", &user1)
                .add_attribute("to", &user2)
        );

        // query balance
        assert_eq!(
            query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::Balance {
                    owner: user2.clone(),
                    token_id: token1.clone(),
                }
            ),
            to_binary(&BalanceResponse {
                balance: 1u64.into()
            }),
        );
        assert_eq!(
            query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::Balance {
                    owner: user1.clone(),
                    token_id: token1.clone(),
                }
            ),
            to_binary(&BalanceResponse {
                balance: 0u64.into()
            }),
        );

        // batch mint token2 and token3
        assert_eq!(
            execute(
                deps.as_mut(),
                mock_env(),
                mock_info(minter.as_ref(), &[]),
                Cw1155ExecuteMsg::BatchMint {
                    to: user2.clone(),
                    batch: vec![(token2.clone(), 1u64.into()), (token3.clone(), 1u64.into())],
                },
            )
            .unwrap(),
            Response::new()
                .add_attribute("action", "mint")
                .add_attribute("token_id[0]", &token2)
                .add_attribute("amount[0]", 1u64.to_string())
                .add_attribute("token_id[1]", &token3)
                .add_attribute("amount[1]", 1u64.to_string())
                .add_attribute("to", &user2)
        );

        // invalid batch transfer, (user2 not approved yet)
        let batch_transfer_msg = Cw1155ExecuteMsg::BatchTransferFrom {
            from: user2.clone(),
            to: user1.clone(),
            batch: vec![
                (token1.clone(), 1u64.into()),
                (token2.clone(), 1u64.into()),
                (token3.clone(), 1u64.into()),
            ],
        };
        assert!(matches!(
            execute(
                deps.as_mut(),
                mock_env(),
                mock_info(minter.as_ref(), &[]),
                batch_transfer_msg.clone(),
            ),
            Err(ContractError::Unauthorized {}),
        ));

        // user2 approve
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(user2.as_ref(), &[]),
            Cw1155ExecuteMsg::ApproveAll {
                operator: minter.clone(),
                expires: None,
            },
        )
        .unwrap();

        // valid batch transfer
        assert_eq!(
            execute(
                deps.as_mut(),
                mock_env(),
                mock_info(minter.as_ref(), &[]),
                batch_transfer_msg,
            )
            .unwrap(),
            Response::new()
                .add_attribute("action", "transfer")
                .add_attribute("token_id[0]", &token1)
                .add_attribute("amount[0]", 1u64.to_string())
                .add_attribute("token_id[1]", &token2)
                .add_attribute("amount[1]", 1u64.to_string())
                .add_attribute("token_id[2]", &token3)
                .add_attribute("amount[2]", 1u64.to_string())
                .add_attribute("from", &user2)
                .add_attribute("to", &user1)
        );

        // batch query
        assert_eq!(
            query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::BatchBalance {
                    owner: user1.clone(),
                    token_ids: vec![token1.clone(), token2.clone(), token3.clone()],
                }
            ),
            to_binary(&BatchBalanceResponse {
                balances: vec![1u64.into(), 1u64.into(), 1u64.into()]
            }),
        );

        // user1 revoke approval
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(user1.as_ref(), &[]),
            Cw1155ExecuteMsg::RevokeAll {
                operator: minter.clone(),
            },
        )
        .unwrap();

        // query approval status
        assert_eq!(
            query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::Allowance {
                    owner: user1.clone(),
                    operator: minter.clone(),
                }
            ),
            to_binary(&AllowanceResponse { approved: false }),
        );

        // tranfer without approval
        assert!(matches!(
            execute(
                deps.as_mut(),
                mock_env(),
                mock_info(minter.as_ref(), &[]),
                Cw1155ExecuteMsg::TransferFrom {
                    from: user1.clone(),
                    to: user2,
                    token_id: token1.clone(),
                    amount: 1u64.into(),
                },
            ),
            Err(ContractError::Unauthorized {})
        ));

        // burn token1
        assert_eq!(
            execute(
                deps.as_mut(),
                mock_env(),
                mock_info(user1.as_ref(), &[]),
                Cw1155ExecuteMsg::Burn {
                    from: user1.clone(),
                    token_id: token1.clone(),
                    amount: 1u64.into(),
                }
            )
            .unwrap(),
            Response::new()
                .add_attribute("action", "burn")
                .add_attribute("token_id", &token1)
                .add_attribute("amount", 1u64.to_string())
                .add_attribute("from", &user1)
        );

        // burn them all
        assert_eq!(
            execute(
                deps.as_mut(),
                mock_env(),
                mock_info(user1.as_ref(), &[]),
                Cw1155ExecuteMsg::BatchBurn {
                    from: user1.clone(),
                    batch: vec![(token2.clone(), 1u64.into()), (token3.clone(), 1u64.into())]
                }
            )
            .unwrap(),
            Response::new()
                .add_attribute("action", "burn")
                .add_attribute("token_id[0]", &token2)
                .add_attribute("amount[0]", 1u64.to_string())
                .add_attribute("token_id[1]", &token3)
                .add_attribute("amount[1]", 1u64.to_string())
                .add_attribute("from", &user1)
        );
    }

    #[test]
    fn check_send_contract() {
        let receiver = String::from("receive_contract");
        let user1 = String::from("user1");
        //let operator = String::from("operator");
        let dummy_msg = Binary::default();

        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            name : String::from("NFT TOKEN ONE"),
            symbol : String::from("NFTONE"),
        };

        let creator = mock_info("operator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(),creator , msg).unwrap();
        assert_eq!(0, res.messages.len());

        //Define token1 NFT
        let creator = mock_info("operator", &[]);
        /*let res =*/ execute(
            deps.as_mut(),
            mock_env(),
            creator,
            Cw1155ExecuteMsg::DefineToken {
               token_uri : String::from("www.token1.com"),
               max_supply : Uint128::from(10000u128),
            },
        )
        .unwrap();
        //let token1 = res.attributes[2].value.clone();

        //Define token2 NFT
        let creator = mock_info("operator", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            creator,
            Cw1155ExecuteMsg::DefineToken {
                token_uri : String::from("www.token2.com"),
               max_supply : Uint128::from(10000u128),
            },
        )
        .unwrap();
        let token2 = res.attributes[2].value.clone();

        let creator = mock_info("operator", &[]);
        execute(
            deps.as_mut(),
            mock_env(),
            creator,
            Cw1155ExecuteMsg::Mint {
                to: user1.clone(),
                token_id: token2.clone(),
                amount: 1u64.into(),
            },
        )
        .unwrap();

        // BatchSendFrom
        let user1_info = mock_info("user1", &[]);
        assert_eq!(
            execute(
                deps.as_mut(),
                mock_env(),
                user1_info,
                Cw1155ExecuteMsg::BatchSendFrom {
                    from: user1.clone(),
                    contract: receiver.clone(),
                    batch: vec![(token2.clone(), 1u64.into())],
                    msg: dummy_msg.clone(),
                },
            )
            .unwrap(),
            Response::new()
                .add_message(
                    Cw1155BatchReceiveMsg {
                        operator: user1.clone(),
                        from: Some(user1.clone()),
                        batch: vec![(token2.clone(), 1u64.into())],
                        msg: dummy_msg,
                    }
                    .into_cosmos_msg(receiver.clone())
                    .unwrap()
                )
                .add_attribute("action", "transfer")
                .add_attribute("token_id[0]", &token2)
                .add_attribute("amount[0]", 1u64.to_string())
                .add_attribute("from", &user1)
                .add_attribute("to", &receiver)
        );
    }

    #[test]
    fn check_queries() {
        // mint multiple types of tokens, and query them
        // grant approval to multiple operators, and query them
        //let tokens = (0..10).map(|i| format!("token{}", i)).collect::<Vec<_>>();
        let users = (0..10).map(|i| format!("user{}", i)).collect::<Vec<_>>();
        
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            name : String::from("NFT TOKEN FIVE"),
            symbol : String::from("NFTFIVE"),
        };

        let creator = mock_info("operator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(),creator , msg).unwrap();
        assert_eq!(0, res.messages.len());

        let mut index = 0;
        let mut tokens : Vec<String> = vec![];
        while index < 10 {
            let creator = mock_info("operator", &[]);
            let res = execute(
                deps.as_mut(),
                mock_env(),
                creator,
                Cw1155ExecuteMsg::DefineToken {
                    token_uri : String::from(""), 
                    max_supply : Uint128::from(10000u128)
                },
            )
            .unwrap();
            let tokenid = res.attributes[2].value.clone(); //Get return token id.
            tokens.push(tokenid);
            index = index + 1;
        }

        let creator = mock_info("operator", &[]);
        execute(
            deps.as_mut(),
            mock_env(),
            creator,
            Cw1155ExecuteMsg::BatchMint {
                to: users[0].clone(),
                batch: tokens
                    .iter()
                    .map(|token_id| (token_id.clone(), 1u64.into()))
                    .collect::<Vec<_>>(),
            },
        )
        .unwrap();

        //Token supply
        assert_eq!(
            query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::TokenSupply {
                    token_id: String::from("0")
                },
            ),
            to_binary(&TokenSupplyResponse {
                supply : TokenSupply{
                        total_supply : Uint128::from(1u128),
                        max_supply : Uint128::from(10000u128)
                    }
            })
        );

        //Check batch total supply
        assert_eq!(
            query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::TokenSupplies {
                    token_ids: tokens.clone(),
                },
            ),
            to_binary(&TokenSuppliesResponse {
                supplies : vec![
                    TokenSupply{total_supply : Uint128::from(1u128),max_supply : Uint128::from(10000u128)},
                    TokenSupply{total_supply : Uint128::from(1u128),max_supply : Uint128::from(10000u128)},
                    TokenSupply{total_supply : Uint128::from(1u128),max_supply : Uint128::from(10000u128)},
                    TokenSupply{total_supply : Uint128::from(1u128),max_supply : Uint128::from(10000u128)},
                    TokenSupply{total_supply : Uint128::from(1u128),max_supply : Uint128::from(10000u128)},
                    TokenSupply{total_supply : Uint128::from(1u128),max_supply : Uint128::from(10000u128)},
                    TokenSupply{total_supply : Uint128::from(1u128),max_supply : Uint128::from(10000u128)},
                    TokenSupply{total_supply : Uint128::from(1u128),max_supply : Uint128::from(10000u128)},
                    TokenSupply{total_supply : Uint128::from(1u128),max_supply : Uint128::from(10000u128)},
                    TokenSupply{total_supply : Uint128::from(1u128),max_supply : Uint128::from(10000u128)},
                ]
            })
        );

        assert_eq!(
            query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::Tokens {
                    owner: users[0].clone(),
                    start_after: None,
                    limit: Some(5),
                },
            ),
            to_binary(&TokensResponse {
                tokens: tokens[..5].to_owned()
            })
        );

        assert_eq!(
            query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::Tokens {
                    owner: users[0].clone(),
                    start_after: Some("5".to_owned()),
                    limit: Some(5),
                },
            ),
            to_binary(&TokensResponse {
                tokens: tokens[6..].to_owned()
            })
        );

        assert_eq!(
            query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::AllTokens {
                    start_after: Some("5".to_owned()),
                    limit: Some(5),
                },
            ),
            to_binary(&TokensResponse {
                tokens: tokens[6..].to_owned()
            })
        );

        assert_eq!(
            query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::TokenInfo {
                    token_id: "5".to_owned()
                },
            ),
            to_binary(&TokenInfoResponse { url: "".to_owned() })
        );

        for user in users[1..].iter() {
            execute(
                deps.as_mut(),
                mock_env(),
                mock_info(users[0].as_ref(), &[]),
                Cw1155ExecuteMsg::ApproveAll {
                    operator: user.clone(),
                    expires: None,
                },
            )
            .unwrap();
        }

        assert_eq!(
            query(
                deps.as_ref(),
                mock_env(),
                Cw1155QueryMsg::AllOperators {
                    owner: users[0].clone(),
                    include_expired: false,
                    start_after: Some(String::from("user2")),
                    limit: Some(1),
                },
            ),
            to_binary(&OperatorsResponse {
                operators: vec![cw1155::Approval {
                    spender: users[3].clone(),
                    expires: Expiration::Never {}
                }],
            })
        );
    }

    #[test]
    fn approval_expires() {
        let mut deps = mock_dependencies();
        let user1 = String::from("user1");
        let user2 = String::from("user2");

        let env = {
            let mut env = mock_env();
            env.block.height = 10;
            env
        };

        let msg = InstantiateMsg {
            name : String::from("NFT TOKEN FOUR"),
            symbol : String::from("NFTFOUR"),
        };

        let creator = mock_info("operator", &[]);
        let res = instantiate(deps.as_mut(), env.clone(), creator, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let creator = mock_info("operator", &[]);
        let res = execute(
            deps.as_mut(),
            env.clone(),
            creator,
            Cw1155ExecuteMsg::DefineToken {
                token_uri: String::from("www.abc.xyz"),
                max_supply : Uint128::from(10000u128)
            },
        )
        .unwrap();
        let token1 = res.attributes[2].value.clone();


        let creator = mock_info("operator", &[]);
        execute(
            deps.as_mut(),
            env.clone(),
            creator,
            Cw1155ExecuteMsg::Mint {
                to: user1.clone(),
                token_id: token1,
                amount: 1u64.into(),
            },
        )
        .unwrap();

        // invalid expires should be rejected
        assert!(matches!(
            execute(
                deps.as_mut(),
                env.clone(),
                mock_info(user1.as_ref(), &[]),
                Cw1155ExecuteMsg::ApproveAll {
                    operator: user2.clone(),
                    expires: Some(Expiration::AtHeight(5)),
                },
            ),
            Err(_)
        ));

        execute(
            deps.as_mut(),
            env.clone(),
            mock_info(user1.as_ref(), &[]),
            Cw1155ExecuteMsg::ApproveAll {
                operator: user2.clone(),
                expires: Some(Expiration::AtHeight(100)),
            },
        )
        .unwrap();

        let query_msg = Cw1155QueryMsg::Allowance {
            owner: user1,
            operator: user2,
        };
        assert_eq!(
            query(deps.as_ref(), env, query_msg.clone()),
            to_binary(&AllowanceResponse { approved: true })
        );

        let env = {
            let mut env = mock_env();
            env.block.height = 100;
            env
        };

        assert_eq!(
            query(deps.as_ref(), env, query_msg,),
            to_binary(&AllowanceResponse { approved: false })
        );
    }

    #[test]
    fn mint_overflow() {
        let mut deps = mock_dependencies();
        let user1 = String::from("user1");

        let env = mock_env();
        let msg = InstantiateMsg {
            name : String::from("NFT TOKEN THREE"),
            symbol : String::from("NFTTHREE"),
        };

        let creator = mock_info("operator", &[]);
        let res = instantiate(deps.as_mut(), env.clone(),creator , msg).unwrap();
        assert_eq!(0, res.messages.len());

        let creator = mock_info("operator", &[]);
        let res = execute(
            deps.as_mut(),
            env.clone(),
            creator,
            Cw1155ExecuteMsg::DefineToken {
                token_uri : String::from("www.bbb.xyz"),
                max_supply: Uint128::from(10000u128),
            },
        )
        .unwrap();
        let token1 = res.attributes[2].value.clone(); //Get return token id.

        //Mint this it will reach max supply
        let creator = mock_info("operator", &[]);
        execute(
            deps.as_mut(),
            env.clone(),
            creator,
            Cw1155ExecuteMsg::Mint {
                to: user1.clone(),
                token_id: token1.clone(),
                amount: Uint128::from(10000u128),
            },
        )
        .unwrap();

        //Mint another and overflow
        let creator = mock_info("operator", &[]);
        let err = execute(
            deps.as_mut(),
            env.clone(),
            creator,
            Cw1155ExecuteMsg::Mint {
                to: user1.clone(),
                token_id: token1.clone(),
                amount : Uint128::from(1u128),
            },
        )
        .unwrap_err();

        assert_eq!(err, ContractError::ExceedMaxSupply{});
    }
}
