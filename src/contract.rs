use std::sync::Arc;

use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

use crate::msg::{self, GreetResp, InstantiateMsg, QueryMsg, ExecuteMsg,AdminResp};
use crate::state::ADMINS;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let admins: StdResult<Vec<_>> = msg
    .admins
    .into_iter()
    .map(|addr| deps.api.addr_validate(&addr))
    .collect();

    ADMINS.save(deps.storage, &admins?)?;

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

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Greet {} => to_json_binary(&query::greet()?),
        AdminsList {  } => to_json_binary(&query::admins_list(deps)?),
    }
}

mod query {
    use self::msg::AdminResp;

    use super::*;

    pub fn greet() -> StdResult<GreetResp> {
        let resp = GreetResp {
            message: "Hello CosmWasm".to_owned(),
        };

        Ok(resp)
        // to_json_binary(&resp)
    }

    pub fn admins_list(deps: Deps) -> StdResult<AdminResp>{
        let admins = ADMINS.load(deps.storage)?;
        let resp: AdminResp = AdminResp{admins};
        Ok(resp)
    }
}





#[cfg(test)]
mod tests {
    use crate::msg::AdminResp;

    use super::*;
    use cosmwasm_std::from_json;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn greet_query() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        instantiate(deps.as_mut(), env, mock_info("sender", &[]), InstantiateMsg{admins: vec![]}).unwrap();

        let resp = query(deps.as_ref(), mock_env(), QueryMsg::Greet {}).unwrap();

        let resp: GreetResp = from_json(&resp).unwrap();
        // let resp:AdminResp = from_bina(&resp).unwrap();

        assert_eq!(
            resp,
            GreetResp {
                message: "Hello CosmWasm".to_owned(),
                // admins: vec![]
            }
        );

        let admin_resp = query(deps.as_ref(),mock_env(),QueryMsg::AdminsList {  }).unwrap();
        let admin_resp : AdminResp = from_json(&admin_resp).unwrap();
        assert_eq!(
            admin_resp,
            AdminResp{
                admins: vec![]
            }
        )

    }
    #[test]
    fn greet_is_working() {
        let resp = query::greet().unwrap();

        assert_ne!(
            resp,
            GreetResp {
                message: "not me".to_owned(),
            }
        )
    }

    // fn
}
