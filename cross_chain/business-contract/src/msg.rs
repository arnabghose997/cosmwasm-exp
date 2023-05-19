use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{IbcTimeout};
use crate::ics999::{Action};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    /// Send a packet consisting of a series of actions
    Act {
        /// The connection via which to send the actions.
        /// The contract will query the appropriate channel.
        connection_id: String,

        /// One or more actions to take
        actions: Vec<Action>,

        /// How many seconds from how will the packet timeout
        /// TODO: make this optional
        timeout: Option<IbcTimeout>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
