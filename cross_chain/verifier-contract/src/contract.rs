#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, to_binary,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryRandomNumResponse};
use crate::host::{handle};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:verifier-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Handle {
            src,
            dest,
            controller,
            actions,
        } => {
            // if info.sender != env.contract.address {
            //     return Err(ContractError::Unauthorized);
            // }

            handle(deps, env, src, dest, controller, actions)
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryRandomNum {  } => return_random_num(),
    }
}

pub fn return_random_num() -> StdResult<Binary> {
    to_binary(&QueryRandomNumResponse { result: 2356 })
}

#[cfg(test)]
mod tests {}
