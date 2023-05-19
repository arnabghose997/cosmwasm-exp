use cw_storage_plus::{Map};


pub const ACTIVE_CHANNELS: Map<&str, String> = Map::new("act_chan");


pub const AFTER_ACTION: u64 = 1111;
pub const AFTER_ALL_ACTIONS: u64 = 2222;
pub const AFTER_CALLBACK: u64 = 3333;
