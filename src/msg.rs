use serde:: { Serialize, Deserialize};

#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
pub struct GreetResp {
    pub message: String,
}


#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
pub enum QueryMsg {
    Greet {},
}

#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
pub struct InstantiateMsg{
    pub admins: Vec<String>,
}