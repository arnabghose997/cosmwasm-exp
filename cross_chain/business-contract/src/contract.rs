#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point};
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, IbcBasicResponse, IbcChannelCloseMsg,
    IbcChannelConnectMsg, IbcChannelOpenMsg, IbcChannelOpenResponse, IbcPacketAckMsg, IbcPacketTimeoutMsg, MessageInfo, Response,
    StdResult,
};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::controller::{act, packet_lifecycle_complete};

// IBC
use crate::ibc::{open_init, open_connect, open_try, close};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:business-contract";
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
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Act {
            connection_id,
            actions,
            timeout,
        } => {
            if actions.is_empty() {
                return Err(ContractError::EmptyActionQueue);
            }

            act(deps, env, info, connection_id, actions, timeout)
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
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
pub fn ibc_packet_ack(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
    packet_lifecycle_complete(deps, env, msg.original_packet, Some(msg.acknowledgement.data))
}

#[entry_point]
pub fn ibc_packet_timeout(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    packet_lifecycle_complete(deps, env, msg.packet, None)
}

#[cfg(test)]
mod tests {}
