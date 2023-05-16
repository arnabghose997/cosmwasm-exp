use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CustomMsg { val: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    CallVerifyTruthMethod {
        contract_address: String,
        a: u64,
        b: u64,
        expected_sum: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CallVerifyTruthMethodResponse {
    pub result: bool,
}

// Structs for External contracts

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryVerifyTruthMsg {
    VerifyTruth { 
        a: u64,
        b: u64,
        sum: u64,  
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryVerifyTruthMsgResponse {
    pub valid: bool,
}