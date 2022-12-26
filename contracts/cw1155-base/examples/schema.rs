use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use cw1155::{
    OperatorsResponse, BalanceResponse, BatchBalanceResponse, Cw1155BatchReceiveMsg,
    Cw1155ExecuteMsg, Cw1155QueryMsg, Cw1155ReceiveMsg, AllowanceResponse,
    TokenInfoResponse, TokensResponse,ContractInfoResponse,
};
use cw1155_base::msg::InstantiateMsg;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(Cw1155ExecuteMsg), &out_dir);
    export_schema(&schema_for!(Cw1155QueryMsg), &out_dir);
    export_schema(&schema_for!(Cw1155ReceiveMsg), &out_dir);
    export_schema(&schema_for!(Cw1155BatchReceiveMsg), &out_dir);
    export_schema(&schema_for!(BalanceResponse), &out_dir);
    export_schema(&schema_for!(BatchBalanceResponse), &out_dir);
    export_schema(&schema_for!(OperatorsResponse), &out_dir);
    export_schema(&schema_for!(AllowanceResponse), &out_dir);
    export_schema(&schema_for!(TokenInfoResponse), &out_dir);
    export_schema(&schema_for!(TokensResponse), &out_dir);
    export_schema(&schema_for!(ContractInfoResponse), &out_dir);
}
