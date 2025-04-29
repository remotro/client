use crate::net::Connection;
use super::deck::Card;

pub struct Play<'a> {
    info: protocol::PlayInfo,
    connection: &'a mut Connection,
}

impl<'a> Play<'a> {
    pub(crate) fn new(info: protocol::PlayInfo, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }

    pub fn hand(&self) -> &[Card] {
        &self.info.hand
    }
}

pub(crate) mod protocol {
    use serde::{Deserialize, Serialize};
    use crate::{balatro::deck::Card, net::protocol::{Packet, Response}};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct PlayInfo {
        pub hand: Vec<Card>
    }

    impl Response for PlayInfo {
    }

    impl Packet for PlayInfo {
        fn kind() -> String {
            "play/hand".to_string()
        }
    }
}