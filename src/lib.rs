use cosmwasm_std::{
    entry_point, Deps, Env, DepsMut, Empty, MessageInfo, StdResult, Response
};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info : MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}