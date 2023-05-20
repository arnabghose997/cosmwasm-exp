use cosmwasm_std::{
    ChannelResponse, IbcQuery, PortIdResponse, QuerierWrapper, QueryRequest, StdResult,
};
use crate::error::ContractError;

/// Query the connection ID associated with the specified channel
pub fn connection_of_channel(
    querier: &QuerierWrapper,
    channel_id: &str,
) -> Result<String, ContractError> {
    let chan_res: ChannelResponse = querier.query(&QueryRequest::Ibc(IbcQuery::Channel {
        channel_id: channel_id.into(),
        port_id: None, // default to the contract's own port
    }))?;

    if let Some(chan) = chan_res.channel {
        Ok(chan.connection_id)
    } else {
        return Err(ContractError::ChannelNotFound {
            port_id: query_port(querier)?,
            channel_id: channel_id.into(),
        });
    }
}

pub fn query_port(querier: &QuerierWrapper) -> StdResult<String> {
    querier.query::<PortIdResponse>(&QueryRequest::Ibc(IbcQuery::PortId {}))
        .map(|res| res.port_id)
}
