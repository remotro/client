use protocol::BlindInfo;
use serde::Deserialize;

use crate::net::Connection;

pub struct SelectBlind<'a> {
    info: protocol::BlindInfo,
    connection: &'a mut Connection,
}

impl<'a> SelectBlind<'a> {
    pub(crate) fn new(info: BlindInfo, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }

    pub async fn skip(self) -> Result<(), super::Error> {
        self.connection.request(protocol::SkipBlind).await??;
        Ok(())
    }

    pub async fn select(self) -> Result<(), super::Error> {
        self.connection.request(protocol::SelectBlind).await??;
        Ok(())
    }

    pub fn small(&self) -> PlainBlind {
        self.info.small
    }

    pub fn big(&self) -> PlainBlind {
        self.info.big
    }

    pub fn boss(&self) -> PlainBlind {
        self.info.boss
    }
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct PlainBlind {
    chips: u32,
    state: BlindState,
}

#[derive(Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum BlindState {
    Select,
    Skipped,
    Upcoming,
    Defeated,
}

pub(crate) mod protocol {
    use crate::net::protocol::{Packet, Request, Response};
    use serde::{Deserialize, Serialize};

    use super::PlainBlind;

    #[derive(Deserialize)]
    pub struct BlindInfo {
        pub small: PlainBlind,
        pub big: PlainBlind,
        pub boss: PlainBlind,
    }

    impl Response for BlindInfo {}

    impl Packet for BlindInfo {
        fn kind() -> String {
            "blind_select/info".to_string()
        }
    }

    #[derive(Serialize)]
    pub struct SelectBlind;

    impl Request for SelectBlind {
        type Expect = Result<BlindInfo, String>;
    }

    impl Packet for SelectBlind {
        fn kind() -> String {
            "blind_select/select".to_string()
        }
    }

    #[derive(Serialize)]
    pub struct SkipBlind;

    impl Request for SkipBlind {
        type Expect = Result<BlindInfo, String>;
    }

    impl Packet for SkipBlind {
        fn kind() -> String {
            "blind_select/skip".to_string()
        }
    }
}
