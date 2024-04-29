use serde:: { Serialize, Deserialize};
use cosmwasm_std::Addr;

#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
pub struct GreetResp {
    pub message: String,
}

#[derive(Serialize,Deserialize,Debug,PartialEq,Clone)]
pub struct AdminResp {
    pub admins: Vec<Addr>,
}


#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
pub enum QueryMsg {
    Greet {},
    AdminsList {},
}

#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
pub struct InstantiateMsg{
    pub admins: Vec<String>,
}

#[derive(Serialize,Debug,Deserialize,PartialEq,Clone)]
pub enum ExecuteMsg {
    AddMembers {admins: Vec<String>},
    Leave{},
}