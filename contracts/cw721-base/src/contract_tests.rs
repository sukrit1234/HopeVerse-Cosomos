#![cfg(test)]
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{from_binary, to_binary, CosmosMsg, DepsMut, Empty,Uint128, Response, WasmMsg};

use cw721::{
    ContractInfoResponse, Cw721ReceiveMsg, Expiration,
    TokenInfoResponse, OperatorsResponse, OwnerOfResponse,Cw721Query
};

use crate::{
    ContractError, Cw721Contract, Extension, InstantiateMsg,
};

pub type ExecuteMsg = cw721::Cw721ExecuteMsg<Extension, Empty>;
pub type QueryMsg = cw721::Cw721QueryMsg<Empty>;

const MINTER: &str = "merlin";
const CONTRACT_NAME: &str = "Magic Power";
const SYMBOL: &str = "MGK";

fn setup_contract(deps: DepsMut<'_>) -> Cw721Contract<'static, Extension, Empty, Empty, Empty> {
    let contract = Cw721Contract::default();
    let msg = InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        max_supply : Uint128::from(10000u128),
    };
    let info = mock_info("creator", &[]);
    let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    contract
}

#[test]
fn proper_instantiation() {
    let mut deps = mock_dependencies();
    let contract = Cw721Contract::<Extension, Empty, Empty, Empty>::default();

    let msg = InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        max_supply : Uint128::from(10000u128),
    };
    let info = mock_info("creator", &[]);
    let creator_addr = info.sender.clone();

    // we can just call .unwrap() to assert this was a success
    let res = contract
        .instantiate(deps.as_mut(), mock_env(), info, msg)
        .unwrap();
    assert_eq!(0, res.messages.len());

    let info = contract.contract_info(deps.as_ref()).unwrap();
    assert_eq!(
        info,
        ContractInfoResponse {
            name: CONTRACT_NAME.to_string(),
            symbol: SYMBOL.to_string(),
            owner : creator_addr.to_string(),
        }
    );

    let supply = contract.token_supply(deps.as_ref()).unwrap();
    assert_eq!(Uint128::from(0u128), supply.supply.total_supply);

    // list the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert_eq!(0, tokens.tokens.len());
}

#[test]
fn minting() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();
    let mint_msg = ExecuteMsg::Mint {
        token_owner: String::from("medusa"),
        token_uri: Some(token_uri.clone()),
        extension: None,
    };

    // random cannot mint
    let random = mock_info("random", &[]);
    let err = contract
        .execute(deps.as_mut(), mock_env(), random, mint_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});
    

    // creator can mint
    let allowed = mock_info("creator", &[]);
    let res = contract.execute(deps.as_mut(), mock_env(), allowed, mint_msg).unwrap();
    let token_id = res.attributes[3].value.clone();

    // ensure num tokens increases
    let supply = contract.token_supply(deps.as_ref()).unwrap();
    assert_eq!(Uint128::from(1u128), supply.supply.total_supply);

    
  
    // unknown nft returns error
    let _ = contract
        .token_info(deps.as_ref(), "unknown".to_string())
        .unwrap_err();

    // this nft info is correct
    let info = contract.token_info(deps.as_ref(), token_id.clone()).unwrap();
    assert_eq!(
        info,
        TokenInfoResponse::<Extension> {
            token_uri: Some(token_uri),
            extension: None,
        }
    );

    //Now test change token url - Token owner / other can't change url
    let token_uri_2 = "https://1234.com/55/666".to_string();
    let medusa = mock_info("medusa", &[]);
    let update_msg = ExecuteMsg::UpdateTokenURI {
        token_id: token_id.clone(),
        token_uri: Some(token_uri_2.clone()),
    };
    let err = contract.execute(deps.as_mut(), mock_env(), medusa, update_msg).unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    let update_msg = ExecuteMsg::UpdateTokenURI {
        token_id: token_id.clone(),
        token_uri: Some(token_uri_2.clone()),
    };
    
    let allowed = mock_info("creator", &[]);
    contract.execute(deps.as_mut(), mock_env(), allowed, update_msg).unwrap();
    let info = contract.token_info(deps.as_ref(), token_id.clone()).unwrap();
    assert_eq!(
        info,
        TokenInfoResponse::<Extension> {
            token_uri: Some(token_uri_2),
            extension: None,
        }
    );

    // owner info is correct
    let owner = contract
        .owner_of(deps.as_ref(), token_id.clone())
        .unwrap();
    assert_eq!(
        owner,
        OwnerOfResponse {
            owner: String::from("medusa"),
        }
    );

    // list the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert_eq!(1, tokens.tokens.len());
    assert_eq!(vec![token_id], tokens.tokens);
}

#[test]
fn burning() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();
    let mint_msg = ExecuteMsg::Mint{
        token_owner: MINTER.to_string(),
        token_uri: Some(token_uri),
        extension: None,
    };

    // mint some NFT
    let allowed = mock_info("creator", &[]);
    let res = contract.execute(deps.as_mut(), mock_env(), allowed.clone(), mint_msg).unwrap();
    let token_id = res.attributes[3].value.clone();
    
    let burn_msg = ExecuteMsg::Burn { 
        token_id : token_id,
        from_address : None,
    };

    // random not allowed to burn
    let random = mock_info("random", &[]);
    let err = contract
        .execute(deps.as_mut(), mock_env(), random, burn_msg.clone())
        .unwrap_err();

    assert_eq!(err, ContractError::Unauthorized {});

    
    let _ = contract.execute(deps.as_mut(), mock_env(), allowed, burn_msg).unwrap();
    
    // ensure num tokens decreases
    let supply = contract.token_supply(deps.as_ref()).unwrap();
    assert_eq!(Uint128::from(0u128), supply.supply.total_supply);

    // trying to get nft returns error
    let _ = contract
        .token_info(deps.as_ref(), "petrify".to_string())
        .unwrap_err();

    // list the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert!(tokens.tokens.is_empty());
}

#[test]
fn transferring_nft() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let token_uri = "https://www.merriam-webster.com/dictionary/melt".to_string();
    let mint_msg = ExecuteMsg::Mint{
        token_owner: String::from("venus"),
        token_uri: Some(token_uri),
        extension: None,
    };

    let creator = mock_info("creator", &[]);
    let res = contract.execute(deps.as_mut(), mock_env(), creator, mint_msg).unwrap();
    let token_id = res.attributes[3].value.clone(); //Get return token id.

    // random cannot transfer
    let random = mock_info("random", &[]);
    let transfer_msg = ExecuteMsg::Transfer {
        to: String::from("random"),
        token_id: token_id.clone(),
    };

    let err = contract
        .execute(deps.as_mut(), mock_env(), random, transfer_msg)
        .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // owner can
    let random = mock_info("venus", &[]);
    let transfer_msg = ExecuteMsg::Transfer {
        to: String::from("random"),
        token_id: token_id.clone(),
    };
    let res = contract.execute(deps.as_mut(), mock_env(), random, transfer_msg).unwrap();
    assert_eq!(
        res,
        Response::new()
            .add_attribute("action", "transfer")
            .add_attribute("from", "venus")
            .add_attribute("recipient", "random")
            .add_attribute("token_id", token_id.clone())
    );

    //Owner force transform from random to venus
    let creator = mock_info("creator", &[]);
    let transfer_msg = ExecuteMsg::TransferFrom { 
        from: String::from("random"),
        to: String::from("venus"),
        token_id: token_id.clone(),
    };
    let res = contract.execute(deps.as_mut(), mock_env(), creator, transfer_msg).unwrap();

    assert_eq!(
        res,
        Response::new()
            .add_attribute("action", "transfer_from")
            .add_attribute("from", "random")
            .add_attribute("recipient", "venus")
            .add_attribute("token_id", token_id.clone())
    );
}

#[test]
fn sending_nft() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    // Mint a token
    let token_uri = "https://www.merriam-webster.com/dictionary/melt".to_string();

    let mint_msg = ExecuteMsg::Mint{
        token_owner: String::from("venus"),
        token_uri: Some(token_uri),
        extension: None,
    };

    let creator = mock_info("creator", &[]);
    let res = contract.execute(deps.as_mut(), mock_env(), creator, mint_msg).unwrap();
    let token_id = res.attributes[3].value.clone(); //Get return token id.

    let msg = to_binary("You now have the melting power").unwrap();
    let target = String::from("another_contract");
    let send_msg = ExecuteMsg::Send {
        contract: target.clone(),
        token_id: token_id.clone(),
        msg: msg.clone(),
    };

    let random = mock_info("random", &[]);
    let err = contract
        .execute(deps.as_mut(), mock_env(), random, send_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // but owner can
    let random = mock_info("venus", &[]);
    let res = contract.execute(deps.as_mut(), mock_env(), random, send_msg).unwrap();

    let payload = Cw721ReceiveMsg {
        operator: String::from("venus"),
        from: String::from("venus"),
        token_id: token_id.clone(),
        msg,
    };
    let expected = payload.into_cosmos_msg(target.clone()).unwrap();
    // ensure expected serializes as we think it should
    match &expected {
        CosmosMsg::Wasm(WasmMsg::Execute { contract_addr, .. }) => {
            assert_eq!(contract_addr, &target)
        }
        m => panic!("Unexpected message type: {:?}", m),
    }
    // and make sure this is the request sent by the contract
    assert_eq!(
        res,
        Response::new()
            .add_message(expected)
            .add_attribute("action", "send")
            .add_attribute("from", "venus")
            .add_attribute("recipient", "another_contract")
            .add_attribute("token_id", token_id)
    );
}

#[test]
fn approving_revoking() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let token_uri = "https://www.merriam-webster.com/dictionary/grow".to_string();
    let mint_msg = ExecuteMsg::Mint{
        token_owner: String::from("demeter"),
        token_uri: Some(token_uri),
        extension: None,
    };

    let creator = mock_info("creator", &[]);
    let res = contract.execute(deps.as_mut(), mock_env(), creator, mint_msg).unwrap();
    let token_id = res.attributes[3].value.clone(); //Get return token id.

    // Give random transferring power
    let approve_msg = ExecuteMsg::ApproveAll {
        spender: String::from("random"),
        expires: None,
    };
    let owner = mock_info("demeter", &[]);
    let res = contract
        .execute(deps.as_mut(), mock_env(), owner, approve_msg)
        .unwrap();
    assert_eq!(
        res,
        Response::new()
            .add_attribute("action", "approve_all")
            .add_attribute("sender", "demeter")
            .add_attribute("spender", "random")
    );

    
    // random can now transfer
    let random = mock_info("random", &[]);
    let transfer_msg = ExecuteMsg::Transfer {
        to: String::from("person"),
        token_id: token_id.clone(),
    };
    contract.execute(deps.as_mut(), mock_env(), random, transfer_msg).unwrap();

    // Approvals are removed / cleared
    let query_msg = QueryMsg::OwnerOf {token_id: token_id.clone()};
    let res: OwnerOfResponse = from_binary(
        &contract
            .query(deps.as_ref(), mock_env(), query_msg.clone())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        res,
        OwnerOfResponse {
            owner: String::from("person"),
        }
    );

    // Approve, revoke, and check for empty, to test revoke
    let approve_msg = ExecuteMsg::ApproveAll {
        spender: String::from("random"),
        expires: None,
    };
    let owner = mock_info("person", &[]);
    contract
        .execute(deps.as_mut(), mock_env(), owner.clone(), approve_msg)
        .unwrap();

    let revoke_msg = ExecuteMsg::RevokeAll {
        spender: String::from("random"),
    };
    contract
        .execute(deps.as_mut(), mock_env(), owner, revoke_msg)
        .unwrap();

    // Approvals are now removed / cleared
    let res: OwnerOfResponse = from_binary(
        &contract
            .query(deps.as_ref(), mock_env(), query_msg)
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        res,
        OwnerOfResponse {
            owner: String::from("person"),
        }
    );
}

#[test]
fn approving_all_revoking_all() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    // Mint a couple tokens (from the same owner)
    //let token_id1 = "grow1".to_string();
    let token_uri1 = "https://www.merriam-webster.com/dictionary/grow1".to_string();

    //let token_id2 = "grow2".to_string();
    let token_uri2 = "https://www.merriam-webster.com/dictionary/grow2".to_string();

    let mint_msg1 = ExecuteMsg::Mint{
        token_owner: String::from("demeter"),
        token_uri: Some(token_uri1),
        extension: None,
    };

    let creator = mock_info("creator", &[]);
    let res1 = contract.execute(deps.as_mut(), mock_env(), creator.clone(), mint_msg1).unwrap();
    let token_id1 = res1.attributes[3].value.clone(); //Get return token id.

    let mint_msg2 = ExecuteMsg::Mint{
        token_owner: String::from("demeter"),
        token_uri: Some(token_uri2),
        extension: None,
    };

    let res2 = contract.execute(deps.as_mut(), mock_env(), creator, mint_msg2).unwrap();
    let token_id2 = res2.attributes[3].value.clone(); //Get return token id.

    // paginate the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, Some(1)).unwrap();
    assert_eq!(1, tokens.tokens.len());
    assert_eq!(vec![token_id1.clone()], tokens.tokens);
    let tokens = contract
        .all_tokens(deps.as_ref(), Some(token_id1.clone()), Some(3))
        .unwrap();
    assert_eq!(1, tokens.tokens.len());
    assert_eq!(vec![token_id2.clone()], tokens.tokens);

    // demeter gives random full (operator) power over her tokens
    let approve_all_msg = ExecuteMsg::ApproveAll {
        spender: String::from("random"),
        expires: None,
    };
    let owner = mock_info("demeter", &[]);
    let res = contract
        .execute(deps.as_mut(), mock_env(), owner, approve_all_msg)
        .unwrap();
    assert_eq!(
        res,
        Response::new()
            .add_attribute("action", "approve_all")
            .add_attribute("sender", "demeter")
            .add_attribute("spender", "random")
    );

    // random can now transfer
    let random = mock_info("random", &[]);
    let transfer_msg = ExecuteMsg::Transfer {
        to: String::from("person"),
        token_id: token_id1,
    };
    contract
        .execute(deps.as_mut(), mock_env(), random.clone(), transfer_msg)
        .unwrap();

    // random can now send
    let inner_msg = WasmMsg::Execute {
        contract_addr: "another_contract".into(),
        msg: to_binary("You now also have the growing power").unwrap(),
        funds: vec![],
    };
    let msg: CosmosMsg = CosmosMsg::Wasm(inner_msg);

    let send_msg = ExecuteMsg::Send {
        contract: String::from("another_contract"),
        token_id: token_id2,
        msg: to_binary(&msg).unwrap(),
    };
    contract
        .execute(deps.as_mut(), mock_env(), random, send_msg)
        .unwrap();

    // Approve_all, revoke_all, and check for empty, to test revoke_all
    let approve_all_msg = ExecuteMsg::ApproveAll {
        spender: String::from("operator"),
        expires: None,
    };
    // person is now the owner of the tokens
    let owner = mock_info("person", &[]);
    contract
        .execute(deps.as_mut(), mock_env(), owner, approve_all_msg)
        .unwrap();

    let res = contract
        .operators(
            deps.as_ref(),
            mock_env(),
            String::from("person"),
            true,
            None,
            None,
        )
        .unwrap();
    assert_eq!(
        res,
        OperatorsResponse {
            operators: vec![cw721::Approval {
                spender: String::from("operator"),
                expires: Expiration::Never {}
            }]
        }
    );

    // second approval
    let buddy_expires = Expiration::AtHeight(1234567);
    let approve_all_msg = ExecuteMsg::ApproveAll {
        spender: String::from("buddy"),
        expires: Some(buddy_expires),
    };
    let owner = mock_info("person", &[]);
    contract
        .execute(deps.as_mut(), mock_env(), owner.clone(), approve_all_msg)
        .unwrap();

    // and paginate queries
    let res = contract
        .operators(
            deps.as_ref(),
            mock_env(),
            String::from("person"),
            true,
            None,
            Some(1),
        )
        .unwrap();
    assert_eq!(
        res,
        OperatorsResponse {
            operators: vec![cw721::Approval {
                spender: String::from("buddy"),
                expires: buddy_expires,
            }]
        }
    );
    let res = contract
        .operators(
            deps.as_ref(),
            mock_env(),
            String::from("person"),
            true,
            Some(String::from("buddy")),
            Some(2),
        )
        .unwrap();
    assert_eq!(
        res,
        OperatorsResponse {
            operators: vec![cw721::Approval {
                spender: String::from("operator"),
                expires: Expiration::Never {}
            }]
        }
    );

    let revoke_all_msg = ExecuteMsg::RevokeAll {
        spender: String::from("operator"),
    };
    contract
        .execute(deps.as_mut(), mock_env(), owner, revoke_all_msg)
        .unwrap();

    // Approvals are removed / cleared without affecting others
    let res = contract
        .operators(
            deps.as_ref(),
            mock_env(),
            String::from("person"),
            false,
            None,
            None,
        )
        .unwrap();
    assert_eq!(
        res,
        OperatorsResponse {
            operators: vec![cw721::Approval {
                spender: String::from("buddy"),
                expires: buddy_expires,
            }]
        }
    );

    // ensure the filter works (nothing should be here
    let mut late_env = mock_env();
    late_env.block.height = 1234568; //expired
    let res = contract
        .operators(
            deps.as_ref(),
            late_env,
            String::from("person"),
            false,
            None,
            None,
        )
        .unwrap();
    assert_eq!(0, res.operators.len());
}

#[test]
fn query_tokens_by_owner() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());
    let creator = mock_info("creator", &[]);

    // Mint a couple tokens (from the same owner)
    let demeter = String::from("demeter");
    let ceres = String::from("ceres");
    
    let mint_msg = ExecuteMsg::Mint{
        token_owner: demeter.clone(),
        token_uri: None,
        extension: None,
    };
    let res1 = contract.execute(deps.as_mut(), mock_env(), creator.clone(), mint_msg).unwrap();
    let token_id1 = res1.attributes[3].value.clone(); //Get return token id.

    let mint_msg = ExecuteMsg::Mint{
        token_owner: ceres.clone(),
        token_uri: None,
        extension: None,
    };
    let res2 = contract.execute(deps.as_mut(), mock_env(), creator.clone(), mint_msg).unwrap();
    let token_id2 = res2.attributes[3].value.clone(); //Get return token id.

    let mint_msg = ExecuteMsg::Mint{
        token_owner: demeter.clone(),
        token_uri: None,
        extension: None,
    };
    let res3 = contract.execute(deps.as_mut(), mock_env(), creator, mint_msg).unwrap();
    let token_id3 = res3.attributes[3].value.clone(); //Get return token id.

    // get all tokens in order:
    let expected = vec![token_id1.clone(), token_id2.clone(), token_id3.clone()];
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert_eq!(&expected, &tokens.tokens);
    // paginate
    let tokens = contract.all_tokens(deps.as_ref(), None, Some(2)).unwrap();
    assert_eq!(&expected[..2], &tokens.tokens[..]);
    let tokens = contract
        .all_tokens(deps.as_ref(), Some(expected[1].clone()), None)
        .unwrap();
    assert_eq!(&expected[2..], &tokens.tokens[..]);

    // get by owner
    let by_ceres = vec![token_id2];
    let by_demeter = vec![token_id1, token_id3];
    // all tokens by owner
    let tokens = contract
        .tokens(deps.as_ref(), demeter.clone(), None, None)
        .unwrap();
    assert_eq!(&by_demeter, &tokens.tokens);
    let tokens = contract.tokens(deps.as_ref(), ceres, None, None).unwrap();
    assert_eq!(&by_ceres, &tokens.tokens);

    // paginate for demeter
    let tokens = contract
        .tokens(deps.as_ref(), demeter.clone(), None, Some(1))
        .unwrap();
    assert_eq!(&by_demeter[..1], &tokens.tokens[..]);
    let tokens = contract
        .tokens(deps.as_ref(), demeter, Some(by_demeter[0].clone()), Some(3))
        .unwrap();
    assert_eq!(&by_demeter[1..], &tokens.tokens[..]);
}
