use cosmwasm_std::{
     to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult
};

use crate::msg::{GreetResp, QueryMsg};

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info : MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

// #[entry_point]
// pub fn query(
//     _deps: Deps,
//     _env: Env,
//     _msg: Empty
// ) -> StdResult<Binary> {
//     let resp = QueryResp {
//         message : "Hello CosmWasm".to_owned(),
//     };
//     to_json_binary(&resp)
// }

pub fn query(
    _deps : Deps,
    _env : Env,
    msg: QueryMsg
) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Greet {  } => {
           
            to_json_binary(&query::greet()?)
        }
        
    }
}

mod query {
    use super::*;

    pub fn greet() -> StdResult<GreetResp> {
        let resp = GreetResp{
            message : "Hello CosmWasm".to_owned(),
        };

        Ok(resp)
        // to_json_binary(&resp)
    }
}