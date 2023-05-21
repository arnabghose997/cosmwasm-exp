use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, Binary, ContractResult, DepsMut,
    Env, IbcEndpoint, Response, StdResult, Storage, SystemResult,
};
use cw_storage_plus::Item;

use crate::ics999::{Action, ActionResult};

use crate::{
    error::ContractError,
    state::{ACCOUNTS}
};

const HANDLER: Item<Handler> = Item::new("handler");

/// An ICS-999 packet contains one or more `Action`'s that need to be executed
/// one at a time and atomically.
///
/// Handler is an object that contains necessary states and methods for
/// executing the actions. It also implements serde traits so that it can be
/// saved/loaded from the contract store.
#[cw_serde]
pub(super) struct Handler {
    /// The chain where the packet was sent from, i.e. the controller chain
    pub src: IbcEndpoint,

    /// The current chain, i.e. the host chain
    pub dest: IbcEndpoint,

    /// The account who sent the packet on the sender chain
    pub controller: String,

    /// The interchain account controlled by the sender
    pub host: Option<Addr>,

    /// The action is to be executed at the current step.
    /// None means all actions have finished executing.
    pub action: Option<Action>,

    /// The actions that are to be executed later, in reverse order.
    pub pending_actions: Vec<Action>,

    /// The results from executing the earlier actions
    ///
    /// At the end of each step, the response data is parsed and pushed into
    /// this queue.
    ///
    /// Once all actions have finished executing, this enture queue is returned
    /// in the packet acknowledgement.
    pub results: Vec<ActionResult>,
}

impl Handler {
    pub fn create(
        store: &dyn Storage,
        src: IbcEndpoint,
        dest: IbcEndpoint,
        controller: String,
        mut actions: Vec<Action>,
    ) -> StdResult<Self> {
        // load the controller's ICA host, which may or may not have already
        // been instantiated
        let host = ACCOUNTS.may_load(store, (&dest.channel_id, &controller))?;

        // reverse the actions, so that we can use pop() to grab the 1st action
        actions.reverse();

        Ok(Self {
            src,
            dest,
            controller,
            host,
            action: None,
            pending_actions: actions,
            results: vec![],
        })
    }

    pub fn load(store: &dyn Storage) -> StdResult<Self> {
        HANDLER.load(store)
    }

    fn save(&self, store: &mut dyn Storage) -> StdResult<()> {
        HANDLER.save(store, self)
    }

    fn remove(store: &mut dyn Storage) {
        HANDLER.remove(store)
    }

    /// Execute the next action in the queue. Saved the updated handler state.
    pub fn handle_next_action(
        mut self,
        deps: DepsMut,
        env: Env,
        response: Option<Response>,
    ) -> Result<Response, ContractError> {
        let mut response = response.unwrap_or_else(|| self.default_handle_action_response());

        // grab the first action in the queue
        self.action = self.pending_actions.pop();

        // if there is no more action to execute
        // delete handler state from contract store, return the results as data
        // in the response
        if let Some(action) = &self.action {
            // convert the action to the appropriate msgs and event attributes
            let _ = match action.clone() {
                
                Action::Query(query_req) => {
                    let query_res = deps.querier.raw_query(&to_binary(&query_req)?);
                    
                    if let SystemResult::Ok(inner) = query_res {
                        if let ContractResult::Ok(query_res_bin) = inner {
                            self.results.push(ActionResult::Query {
                                response: query_res_bin,
                            });        
                        } else {
                            return Err(ContractError::QueryFailed);
                        }
                    } else {
                        return Err(ContractError::QueryFailed);
                    }
    
                    response = response.add_attribute("action", "query");

                    self.save(deps.storage)?;
            
                    return self.handle_next_action(deps, env, Some(response));
                },
            };
        } else {
            Handler::remove(deps.storage);
        
            return Ok(response.set_data(to_binary(&self.results)?));
        }
    }

    /// After an `Execute` action has been completed, parse the response
    pub fn after_action(&mut self, _data: Option<Binary>) -> Result<(), ContractError> {
        // // the action that was executed
        // let action = self.action.as_ref().expect("missing active action");

        // // we only need to parse the result if the action is an msg execution
        // if let Action::Execute(_) = action {
        //     // note that the contract being executed does not necessarily return
        //     // any data
        //     let data = data
        //         .map(|bin| parse_execute_response_data(&bin))
        //         .transpose()?
        //         .and_then(|res| res.data);

        //     self.results.push(ActionResult::Execute {
        //         data,
        //     });
        // }

        Ok(())
    }

    fn default_handle_action_response<T>(&self) -> Response<T> {
        Response::new()
            .add_attribute("method", "handle_next_action")
            .add_attribute("actions_left", self.pending_actions.len().to_string())
    }
}
