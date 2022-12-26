use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

use cw20::{AllowanceResponse, Logo, MarketingInfoResponse};

#[cw_serde]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub owner: Addr,
}

#[cw_serde]
pub struct TokenSupply {
    pub total_supply: Uint128,
    pub max_supply: Uint128,
}

#[cw_serde]
pub struct CooperativeData{
    pub can_mint_for : bool,
    pub can_burn_from : bool,
}

pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");
pub const TOKEN_SUPPLY: Item<TokenSupply> = Item::new("token_supply");
pub const MARKETING_INFO: Item<MarketingInfoResponse> = Item::new("marketing_info");
pub const LOGO: Item<Logo> = Item::new("logo");
pub const COOPERATIVES: Map<&Addr, CooperativeData> = Map::new("cooperatives");
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");
pub const ALLOWANCES: Map<(&Addr, &Addr), AllowanceResponse> = Map::new("allowance");
// TODO: After https://github.com/CosmWasm/cw-plus/issues/670 is implemented, replace this with a `MultiIndex` over `ALLOWANCES`
pub const ALLOWANCES_SPENDER: Map<(&Addr, &Addr), AllowanceResponse> =
    Map::new("allowance_spender");
