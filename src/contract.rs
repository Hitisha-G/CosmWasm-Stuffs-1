use cosmwasm_std::{to_json_binary, Binary,Event, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{self, ExecuteMsg, GreetResp, InstantiateMsg, QueryMsg};
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
        AdminsList {} => to_json_binary(&query::admins_list(deps)?),
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

    pub fn admins_list(deps: Deps) -> StdResult<AdminResp> {
        let admins = ADMINS.load(deps.storage)?;
        let resp: AdminResp = AdminResp { admins };
        Ok(resp)
    }
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        AddMembers { admins } => exec::add_members(deps, info, admins),
        Leave {} => exec::leave(deps, info).map_err(Into::into),
    }
}

mod exec {
    // use std::f32::consts::E;

    use cosmwasm_std::Event;

    use super::*;
    // use cosmwasm_std::StdError;

    pub fn add_members(
        deps: DepsMut,
        info: MessageInfo,
        admins: Vec<String>,
    ) -> Result<Response, ContractError> {
        let mut curr_admins = ADMINS.load(deps.storage)?;
        if !curr_admins.contains(&info.sender) {
            // return Err(StdError::generic_err("Unauthorised Access"));
            return Err(ContractError::Unauthorised {
                sender: info.sender,
            });
        }

        let events = admins
            .iter()
            .map(|admin| Event::new("admin_added").add_attribute("addr", admin));
        let resp = Response::new()
            .add_events(events)
            .add_attribute("action", "add_members")
            .add_attribute("added_count", admins.len().to_string());

        let admins: StdResult<Vec<_>> = admins
            .into_iter()
            .map(|addr| deps.api.addr_validate(&addr))
            .collect();

        curr_admins.append(&mut admins?);
        ADMINS.save(deps.storage, &curr_admins)?;

        Ok(resp)
    }

    pub fn leave(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        ADMINS.update(deps.storage, move |admins| -> StdResult<_> {
            let admins = admins
                .into_iter()
                .filter(|admin| *admin != info.sender)
                .collect();
            Ok(admins)
        })?;
        Ok(Response::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::msg::AdminResp;
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    use super::*;
    use cosmwasm_std::from_json;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn greet_query() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        instantiate(
            deps.as_mut(),
            env,
            mock_info("sender", &[]),
            InstantiateMsg { admins: vec![] },
        )
        .unwrap();

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

        let admin_resp = query(deps.as_ref(), mock_env(), QueryMsg::AdminsList {}).unwrap();
        let admin_resp: AdminResp = from_json(&admin_resp).unwrap();
        assert_eq!(admin_resp, AdminResp { admins: vec![] })
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

    #[test]
    fn add_members() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    admins: vec!["owner".to_owned()],
                },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp = app
            .execute_contract(
                Addr::unchecked("owner"),
                addr,
                &ExecuteMsg::AddMembers {
                    admins: vec!["user".to_owned()],
                },
                &[],
            )
            .unwrap();

        let wasm = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();
        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "action")
                .unwrap()
                .value,
            "add_members"
        );
        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "added_count")
                .unwrap()
                .value,
            "1"
        );

        let admin_added: Vec<_> = resp
            .events
            .iter()
            .filter(|ev| ev.ty == "wasm-admin_added")
            .collect();
        assert_eq!(admin_added.len(), 1);

        assert_eq!(
            admin_added[0]
                .attributes
                .iter()
                .find(|attr| attr.key == "addr")
                .unwrap()
                .value,
            "user"
        );
    }


    // fn
}
