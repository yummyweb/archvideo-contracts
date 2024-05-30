#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{Entry, ExecuteMsg, InstantiateMsg, QueryMsg, VideoResponse};
use crate::state::{Video, LIST};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:archvideo";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UploadVideo {
            video_hash,
            title,
            issue,
            id,
        } => try_upload_video(deps, env, info, video_hash, title, issue, id),
        // ExecuteMsg::LikeVideo { id, new } => try_transfer_domain(deps, env, info, id, new),
    }
}

pub fn try_upload_video(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    video_hash: String,
    title: String,
    issue: String,
    id: String,
) -> Result<Response, ContractError> {
    let video = Video {
        author: info.sender,
        video_hash: video_hash,
        title,
        issue,
        likes: 0,
    };
    LIST.save(deps.storage, &id, &video)?;

    Ok(Response::new().add_attribute("method", "try_upload_video"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryVideos {} => to_binary(&query_videos(deps)?),
    }
}

fn query_videos(deps: Deps) -> StdResult<VideoResponse> {
    let all: StdResult<Vec<_>> = LIST
        .range(deps.storage, None, None, Order::Ascending)
        .collect();

    let mut resp: Vec<Entry> = Vec::new();
    for (_id, data) in all? {
        resp.push(Entry {
            video_hash: data.video_hash,
            author: data.author,
            title: data.title,
            issue: data.issue,
        })
    }
    Ok(VideoResponse { entries: resp })
}

#[cfg(test)]
mod tests {}
