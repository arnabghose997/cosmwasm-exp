use crate::ics999::{Action, PacketData, SenderExecuteMsg};

use cosmwasm_std::{
    from_slice, to_binary, Binary, Deps, DepsMut, Env, IbcBasicResponse, IbcEndpoint, IbcMsg,
    IbcPacket, IbcTimeout, Response, StdResult, SubMsg, WasmMsg,
    PortIdResponse, QuerierWrapper, QueryRequest, IbcQuery, MessageInfo
};
use crate::{error::ContractError};
use crate::state::{ACTIVE_CHANNELS, AFTER_CALLBACK};

pub fn query_port(querier: &QuerierWrapper) -> StdResult<String> {
    querier.query::<PortIdResponse>(&QueryRequest::Ibc(IbcQuery::PortId {}))
        .map(|res| res.port_id)
}

fn localhost(deps: Deps, connection_id: &str) -> StdResult<IbcEndpoint> {
    Ok(IbcEndpoint {
        port_id: query_port(&deps.querier)?,
        channel_id: ACTIVE_CHANNELS.load(deps.storage, connection_id)?,
    })
}

pub fn act(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    connection_id: String,
    actions: Vec<Action>,
    timeout: Option<IbcTimeout>,
) -> Result<Response, ContractError> {
    // find the current chain's port and channel IDs
    let localhost = localhost(deps.as_ref(), &connection_id)?;

    // if the user does not specify a timeout, we use the default
    let timeout = match timeout {
        None => {
            let default_secs = 30;
            IbcTimeout::with_timestamp(env.block.time.plus_seconds(default_secs))
        },
        Some(to) => to,
    };

    Ok(Response::new()
        .add_message(IbcMsg::SendPacket {
            channel_id: localhost.channel_id,
            data: to_binary(&PacketData {
                sender: info.sender.into(),
                actions,
            })?,
            timeout,
    }))
}

pub fn packet_lifecycle_complete(
    _deps: DepsMut,
    _env: Env,
    packet: IbcPacket,
    ack_bin: Option<Binary>,
) -> Result<IbcBasicResponse, ContractError> {
    // deserialize the original packet
    let packet_data: PacketData = from_slice(&packet.data)?;

    // deserialize the ack
    let ack = ack_bin.map(|bin| from_slice(&bin)).transpose()?;

    Ok(IbcBasicResponse::new()
        .add_attribute("method", "packet_lifecycle_complete")
        .add_attribute("channel_id", &packet.src.channel_id)
        .add_attribute("sequence", packet.sequence.to_string())
        .add_attribute("acknowledged", ack.is_some().to_string())
        .add_attribute("sender", &packet_data.sender)
        .add_submessage(SubMsg::reply_always(
            WasmMsg::Execute {
                contract_addr: packet_data.sender,
                msg: to_binary(&SenderExecuteMsg::PacketCallback {
                    channel_id: packet.src.channel_id,
                    sequence: packet.sequence,
                    ack,
                })?,
                funds: vec![],
            },
            AFTER_CALLBACK,
        )))
}
