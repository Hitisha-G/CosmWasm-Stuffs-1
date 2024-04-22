use serde:: { Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct GreetResp {
    pub message: String,
}

pub enum QueryMsg {
    Greet {},
}