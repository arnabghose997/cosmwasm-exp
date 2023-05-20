use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Empty, IbcOrder, QueryRequest};

/// Expected channel packet ordering rule
pub const ORDER: IbcOrder = IbcOrder::Unordered;

/// Expected channel version string
pub const VERSION: &str = "ics999-1";


/// ICS-999 packet data structure
#[cw_serde]
pub struct PacketData {
    /// The account who sends this packet
    pub sender: String,

    /// Actions to take.
    /// The actions will be executed in order and atomically.
    pub action: Vec<Action>,
}

/// ICS-999 packet acknowledgement
///
/// Related: ICS-4 recommand acknowldgement envelop format:
/// https://github.com/cosmos/ibc/tree/main/spec/core/ics-004-channel-and-packet-semantics#acknowledgement-envelope
///
/// ** Notes regarding error messages **
///
/// Error messages are not merklized; that is, validators do not reach
/// consensus over the specific error string). This means that error
/// messages are NOT guaranteed to be deterministic.
///
/// Due to this concern, wasmd redacts error messages:
///   https://github.com/CosmWasm/wasmd/issues/759
///
/// In principle, contracts should only have access to data that are
/// included in the chain's state commitment.
///
/// Therefore, although we return a String here, in reality it will only
/// include the error code, not the message. It will look something like
/// this:
///
/// ```json
/// {
///   "error": "codespace: wasm, code: 5"
/// }
/// ```
#[cw_serde]
pub enum PacketAck {
    /// All actions were executed successfully. In this case, we return the
    /// result of each action.
    ///
    /// ICS-4 recommends a raw binary here, but we choose to use `Vec<ActionResult>`
    /// so that it's easier to consume by the sender contract
    Results(Vec<ActionResult>),

    /// One of the actions failed to execute. In this case, the entire queue of
    /// actions is considered to be failed. We inform the sender contract of the
    /// failure.
    Error(String),
}

#[cw_serde]
pub enum Action {
    /// Perform a query
    Query(QueryRequest<Empty>),
}

#[cw_serde]
pub enum ActionResult {
    /// Result of a successful query
    Query {
        /// The querying contract is responsible for decoding the response
        response: Binary,
    },
}


/// If the sender contract wishes to receive a callback after the completion of
/// a packet lifecycle, it must implement this execute message.
#[cw_serde]
pub enum SenderExecuteMsg {
    /// Called by ICS-999 core contract after the completion of a packet
    /// lifecycle (acknowledged or timed out)
    PacketCallback {
        channel_id: String,
        sequence: u64,
        /// The packet acknowledgement. None if the packet has timed out.
        ack: Option<PacketAck>,
    },
}

