#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryVerifyTruthMsg, QueryVerifyTruthMsgResponse, CallVerifyTruthMethodResponse};
use crate::state::{CONFIG, Config};

const CONTRACT_NAME: &str = "crates.io:business-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    let validated_admin_address = deps.api.addr_validate(&msg.admin_address)?;

    let config = Config {
        admin_address: validated_admin_address
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::CallVerifyTruthMethod { 
            contract_address, 
            a, 
            b, 
            expected_sum 
        } => call_verify_truth_method(deps, contract_address, a, b, expected_sum
            ),
    }
}

fn call_verify_truth_method(deps: Deps, contract_address: String, a: u64, b: u64, expected_sum: u64) -> StdResult<Binary> {
    let verifier_contract_response: QueryVerifyTruthMsgResponse = deps.querier.query_wasm_smart(contract_address, &QueryVerifyTruthMsg::VerifyTruth { a, b, sum: expected_sum })?;
    to_binary(&CallVerifyTruthMethodResponse { result: verifier_contract_response.valid })
}

#[cfg(test)]
mod tests {}
