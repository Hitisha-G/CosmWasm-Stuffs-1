use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult
};
use serde:: { Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
struct QueryResp {
    message: String,
}

pub enum QueryMsg {
    Greet {},
}

#[entry_point]
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
            let resp = QueryResp{
                message: "Hello CosmWasm".to_owned(),
            };
            to_json_binary(&resp)
        }

        
    }
}