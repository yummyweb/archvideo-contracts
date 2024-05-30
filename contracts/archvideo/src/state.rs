use cosmwasm_std::Addr;
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Video {
    pub author: Addr,
    pub title: String,
    pub issue: String,
    pub video_hash: String,
    pub likes: u8,
}

pub const LIST: Map<&str, Video> = Map::new("video_list");