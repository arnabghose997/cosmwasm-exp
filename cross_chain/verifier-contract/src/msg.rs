use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{IbcEndpoint};
use crate::ics999::Action;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Handle {
        src: IbcEndpoint,
        dest: IbcEndpoint,
        controller: String,
        actions: Vec<Action>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryRandomNumResponse)]
    QueryRandomNum {  }
}

#[cw_serde]
pub struct QueryRandomNumResponse {
    pub result: u64,
}
