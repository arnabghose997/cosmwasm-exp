#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, IbcBasicResponse, IbcChannelCloseMsg,
    IbcChannelConnectMsg, IbcChannelOpenMsg, IbcChannelOpenResponse,
    IbcPacketReceiveMsg, IbcReceiveResponse, MessageInfo, Response,
    StdResult, to_binary,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryRandomNumResponse};
use crate::host::{handle, packet_receive};
use crate::ibc::{open_connect, open_init, open_try, close};

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

#[entry_point]
pub fn ibc_channel_open(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelOpenMsg,
) -> Result<IbcChannelOpenResponse, ContractError> {
    match msg {
        IbcChannelOpenMsg::OpenInit {
            channel,
        } => open_init(deps, channel),
        IbcChannelOpenMsg::OpenTry {
            channel,
            counterparty_version,
        } => open_try(deps, channel, counterparty_version),
    }
}

#[entry_point]
pub fn ibc_channel_connect(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse, ContractError> {
    open_connect(deps, msg.channel(), msg.counterparty_version())
}

#[entry_point]
pub fn ibc_channel_close(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelCloseMsg,
) -> Result<IbcBasicResponse, ContractError> {
    close(msg)
}

#[entry_point]
pub fn ibc_packet_receive(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, ContractError> {
    packet_receive(deps, env, msg.packet)
}

#[cfg(test)]
mod tests {}
