#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use cw2::set_contract_version;
use bellman::groth16::verify_proof;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, VerifyTruthResponse, VerifyProofResponse};
use crate::state::{Config, CONFIG, get_verification_key, parse_proof, get_public_signal};

const CONTRACT_NAME: &str = "crates.io:verifier-contract";
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
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // This method checks if sum of a and b equates to sum
        QueryMsg::VerifyTruth { a, b, sum } => query_verify_truth(a, b, sum),
        QueryMsg::VerifyProof { proof, expected_value } => verify_the_zk_proof(proof, expected_value)
    }
}

fn verify_the_zk_proof(proof: String, expected_value: u64) -> StdResult<Binary> {
    let vkey = get_verification_key();
    let parsed_proof = parse_proof(proof);
    let public_signal = get_public_signal(expected_value);
    let result = verify_proof(&vkey, &parsed_proof, &public_signal).is_ok();
    
    if result {
        to_binary(&VerifyProofResponse{ result: "Done".to_string() })
    } else {
        to_binary(&VerifyProofResponse{ result: "Not Done".to_string() })
    }
}

fn query_verify_truth(a: u64, b: u64, sum: u64) -> StdResult<Binary> {
    let mut is_valid_sum = false;

    if a + b == sum {
        is_valid_sum = true;
    }

    to_binary(&VerifyTruthResponse { valid: is_valid_sum })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::{mock_dependencies, mock_env, mock_info}, from_binary};
    use crate::{msg::{InstantiateMsg, QueryMsg, VerifyTruthResponse}};
    use super::{instantiate, query};

    #[test]
    fn test_verify_truth_invalid_case() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg{
            admin_address: "addr1".to_string()
        };
        let _ = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        
        // Invalid Inputs
        let invalid_query_msg = QueryMsg::VerifyTruth { a: 1, b: 2, sum: 3 };
        let query_response = query(deps.as_ref(), env.clone(), invalid_query_msg).unwrap();
        let get_validity: VerifyTruthResponse = from_binary(&query_response).unwrap();
        assert_eq!(
            get_validity,
            VerifyTruthResponse {
                valid: false
            }
        );
    }

    #[test]
    fn test_verify_truth_valid_case() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg{
            admin_address: "addr1".to_string()
        };
        let _ = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Valid inputs
        let valid_query_msg = QueryMsg::VerifyTruth { a: 10, b: 2, sum: 12 };
        let query_response = query(deps.as_ref(), env.clone(), valid_query_msg).unwrap();
        let get_validity: VerifyTruthResponse = from_binary(&query_response).unwrap();
        assert_eq!(
            get_validity,
            VerifyTruthResponse {
                valid: true
            }
        );
    }
}
