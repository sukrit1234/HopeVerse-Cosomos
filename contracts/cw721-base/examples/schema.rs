use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;

use cw721_base::{InstantiateMsg};
use cw721::{Cw721ExecuteMsg, Cw721QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: Cw721ExecuteMsg<Empty, Empty>,
        query: Cw721QueryMsg<Empty>,
    }
}
