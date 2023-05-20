use cw_storage_plus::{Item, Map};
use cosmwasm_std::Addr;

pub const ACTIVE_CHANNELS: Map<&str, String> = Map::new("act_chan");

pub const ACCOUNT_CODE_ID: Item<u64> = Item::new("acc_cid");
pub const ACCOUNTS: Map<(&str, &str), Addr> = Map::new("acct");
