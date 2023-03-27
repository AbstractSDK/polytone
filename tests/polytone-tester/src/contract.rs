#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    CallbackHistoryResponse, ExecuteMsg, HelloHistoryResponse, InstantiateMsg, QueryMsg,
};
use crate::state::{CALLBACK_HISTORY, HELLO_CALL_HISTORY};

const CONTRACT_NAME: &str = "crates.io:ica-host-proxy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CALLBACK_HISTORY.save(deps.storage, &vec![])?;
    HELLO_CALL_HISTORY.save(deps.storage, &vec![])?;

    Ok(Response::default().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Hello { data } => {
            let mut h = HELLO_CALL_HISTORY.load(deps.storage)?;
            h.push(info.sender.into_string());
            HELLO_CALL_HISTORY.save(deps.storage, &h)?;
            let mut response = Response::default().add_attribute("method", "hello");
            if let Some(data) = data {
                response = response.set_data(data);
            }
            Ok(response)
        }
        ExecuteMsg::Callback(callback) => {
            let mut h = CALLBACK_HISTORY.load(deps.storage)?;
            h.push(callback);
            CALLBACK_HISTORY.save(deps.storage, &h)?;
            Ok(Response::default().add_attribute("method", "get_callback"))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::History {} => to_binary(&CallbackHistoryResponse {
            history: CALLBACK_HISTORY.load(deps.storage)?,
        }),
        QueryMsg::HelloHistory {} => to_binary(&HelloHistoryResponse {
            history: HELLO_CALL_HISTORY.load(deps.storage)?,
        }),
    }
}
