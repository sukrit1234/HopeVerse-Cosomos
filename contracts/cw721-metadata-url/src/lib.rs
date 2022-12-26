use cosmwasm_std::{Empty};
use cw2::set_contract_version;
pub use cw721_base::{ContractError, InstantiateMsg};

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:cw721-metadata-url";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Extension = Option<Empty>;
pub type Cw721URLContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type ExecuteMsg = cw721::Cw721ExecuteMsg<Extension, Empty>;
pub type QueryMsg = cw721::Cw721QueryMsg<Empty>;



#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let res = Cw721URLContract::default().instantiate(deps.branch(), env, info, msg)?;
        // Explicitly set contract name and version, otherwise set to cw721-base info
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION).map_err(ContractError::Std)?;
        Ok(res)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Cw721URLContract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw721URLContract::default().query(deps, env, msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw721::Cw721Query;
    use cosmwasm_std::Uint128;
    const CREATOR: &str = "creator";

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = Cw721URLContract::default();
        
        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "Vehicles".to_string(),
            symbol: "VEHICLE".to_string(),
            max_supply : Uint128::from(10000u128),
        };
        contract.instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();

        let token_uri = Some("https://vehicle.example.com/Garage/Enterprise.json".into());
        let exec_msg =  ExecuteMsg::Mint{
            token_owner: "john".to_string(),
            token_uri: token_uri.clone(),
            extension: Some(Empty{}),
        };

        let res = contract.execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();
        let token_id = res.attributes[3].value.clone();

        let res = contract.token_info(deps.as_ref(), token_id.into()).unwrap();
        assert_eq!(res.token_uri, token_uri);
        assert_eq!(res.extension, Some(Empty{}));
    }
}
