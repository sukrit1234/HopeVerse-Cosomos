use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw1155::{Expiration,TokenSupply};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct ContractInfo {
    pub name: String,
    pub symbol: String,
    pub owner: Addr,
}

#[cw_serde]
pub struct CooperativeData{
    pub can_mint_for : bool,
    pub can_burn_from : bool,
}

use std::fmt;
#[derive(Debug, Clone, Copy,PartialEq)]
pub enum TransferAction {
    None,    //Invalid Transfer action both from and to are none.
    Mint,    //Mint (Transfer None to Addr) and increase total supply
    Burn,    //Burn (Transfer Addr to None) and decrease total supply
    Transfer //Simple transfer - Addr to Addr.
}
impl fmt::Display for TransferAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransferAction::Mint => write!(f, "mint"),
            TransferAction::Burn => write!(f, "burn"),
            TransferAction::Transfer => write!(f, "transfer"),
            _ => write!(f, "None")
        }
    }
}


/// Store the minter address who have permission to mint new tokens.
pub const CONTRACT_INFO: Item<ContractInfo> = Item::new("contract_info");

/// Lasten token_id for runing next id.
pub const TOKEN_RUNNING_NO: Item<u128> = Item::new("token_running_no");

/// Cooperatove for another contract can call function.
pub const COOPERATIVES: Map<&Addr, CooperativeData> = Map::new("cooperatives");

/// Store the balance map, `(owner, token_id) -> balance`
pub const BALANCES: Map<(&Addr, &str), Uint128> = Map::new("balances");
/// Store the approval status, `(owner, spender) -> expiration`
pub const APPROVES: Map<(&Addr, &Addr), Expiration> = Map::new("approves");

/// Store the tokens metadata url, also supports enumerating tokens,
/// An entry for token_id must exist as long as there's tokens in circulation.
pub const TOKENS: Map<&str, String> = Map::new("tokens");

//Keep track of token supply
//An entry for token_id => TokenSupply
pub const TOKEN_SUPPLIES: Map<&str, TokenSupply> = Map::new("token_supplies");
