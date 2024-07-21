use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
    pub table: bool,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    FlipTable {},
    Reset { count: i32, table:bool },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
    #[returns(GetFlipResponse)]
    GetFlip {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}

#[cw_serde]
pub struct GetFlipResponse {
    pub table: bool,
}

#[cw_serde]
pub struct GetStateResponse {
    pub table: bool,
    pub count: i32,
}
