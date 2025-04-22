use protocol::BlindInfo;

pub struct SelectBlind {
}

impl SelectBlind {
    pub fn new(state: BlindInfo) -> Self {
        Self {}
    }
}

pub(crate) mod protocol {
    use crate::net::protocol::{Packet, Response};
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct BlindInfo {
        
    }
    
    impl Response for BlindInfo {
    }
    
    impl Packet for BlindInfo {
        fn kind() -> &'static str {
            "blind_select/info"
        }
    }
}