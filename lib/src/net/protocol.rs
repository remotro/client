use serde::{de::DeserializeOwned, Serialize};

pub trait Packet {
    fn kind() -> &'static str;
}

pub trait Request: Serialize + Packet {
    type Expect: Response;
}

pub trait Response : DeserializeOwned + Packet {}

impl Response for Result<Vec<()>, String> {

}

impl Packet for Result<Vec<()>, String> {
    fn kind() -> &'static str {
        "placeholder/result"
    }
}