use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult
};
use msg::{QueryMsg,InstantiateMsg,ExecuteMsg};

mod contract;
mod msg;
mod state;

#[entry_point]
pub fn instantiate ( 
    deps : DepsMut,
    env : Env,
    info : MessageInfo,
    msg : InstantiateMsg
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

// pub fn execute(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     msg: ExecuteMsg,
// ) -> StdResult<Response> {
//     contract::execute(deps,env,info,msg)
// }