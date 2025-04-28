use serde::{Serialize, de::DeserializeOwned};

pub trait Packet {
    fn kind() -> String;
}

pub trait Request: Serialize + Packet {
    type Expect: Response;
}

pub trait Response: DeserializeOwned + Packet {}

impl<P: Response> Response for Result<P, String> {}

impl<P: Response> Packet for Result<P, String> {
    fn kind() -> String {
        "result/".to_string() + &P::kind()
    }
}
