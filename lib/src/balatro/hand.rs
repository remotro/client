use crate::net::Connection;
use super::deck::Card;

pub struct SelectHand<'a> {
    info: protocol::HandInfo,
    connection: &'a mut Connection,
}

impl<'a> SelectHand<'a> {
    pub(crate) fn new(info: protocol::HandInfo,connection: &'a mut Connection) -> Self {
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
    pub struct HandInfo {
        pub hand: Vec<Card>
    }

    impl Response for HandInfo {
    }

    impl Packet for HandInfo {
        fn kind() -> String {
            "select_hand/info".to_string()
        }
    }
}