use cosmwasm_schema::write_api;
use cw20_base::msg::{InstantiateMsg};
use cw20::{Cw20ExecuteMsg,Cw20QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: Cw20ExecuteMsg,
        query: Cw20QueryMsg,
    }
}
