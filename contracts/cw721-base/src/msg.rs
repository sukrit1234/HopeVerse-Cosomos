use cosmwasm_schema::{cw_serde};
use cosmwasm_std::{Uint128/*,Binary */};

#[cw_serde]
#[cfg_attr(test, derive(Default))]
pub struct InstantiateMsg {
    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,
    /// Symbol of the NFT contract
    pub max_supply : Uint128,
}