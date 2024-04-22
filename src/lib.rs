use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult
};
use msg::QueryMsg;

mod contract;
mod msg;

#[entry_point]
pub fn instantiate ( 
    deps : DepsMut,
    env : Env,
    info : MessageInfo,
    msg : Empty
) -> StdResult<Response>
{
    contract::instantiate(deps, env, info, msg)
}

pub fn query (
    deps : Deps,
    env : Env,
    msg : QueryMsg
) -> StdResult<Binary> 
{
    contract::query(deps, env, msg)
}