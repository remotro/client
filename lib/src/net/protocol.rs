use serde::{Deserialize, de::DeserializeOwned, Serialize};

pub trait Request: Serialize {
    type Expect: Response;
    fn kind() -> &'static str;
}

pub trait Response : DeserializeOwned {
    fn kind() -> &'static str;
}

#[derive(Serialize)]
pub struct NewRun {
    pub deck_id: String,
    pub stake_id: String,
    pub seed: Option<String>,
}

impl Request for NewRun {
    type Expect = Ack;
    fn kind() -> &'static str {
        "new_run"
    }
}

#[derive(Deserialize)]
pub struct Ack {
    pub result: Result<(), String>
}

impl Response for Ack {
    fn kind() -> &'static str {
        "ack"
    }
}
