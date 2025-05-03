use crate::net::Connection;
use super::{deck::Card, Error};

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

    pub async fn click(self, indices: &[u32]) -> Result<Self, Error> {
        let info = self.connection.request(protocol::PlayClick { indices: indices.to_vec() }).await??;
        Ok(Self::new(info, self.connection))
    }
}

pub(crate) mod protocol {
    use serde::{Deserialize, Serialize};
    use crate::{balatro::deck::Card, net::protocol::{Packet, Request, Response}};

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

    #[derive(Serialize, Deserialize, Clone)]
    pub struct PlayClick {
        pub indices: Vec<u32>
    }

    impl Request for PlayClick {
        type Expect = Result<PlayInfo, String>;
    }

    impl Packet for PlayClick {
        fn kind() -> String {
            "play/click".to_string()
        }
    }
}