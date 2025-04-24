use protocol::BlindInfo;

use crate::net::Connection;

pub struct SelectBlind<'a> {
    info: protocol::BlindInfo,
    connection: &'a mut Connection
}

impl<'a> SelectBlind<'a> {
    pub(crate) fn new(info: BlindInfo, connection: &'a mut Connection) -> Self {
        Self {
            info,
            connection
        }
    }

    pub async fn skip(self) -> Result<(), super::Error> {
        self.connection.request(protocol::SkipBlind).await??;
        Ok(())
    }

    pub async fn select(self) -> Result<(), super::Error> {
        self.connection.request(protocol::SelectBlind).await??;
        Ok(())
    }
}

pub(crate) mod protocol {
    use crate::net::protocol::{Packet, Request, Response};
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize)]
    pub struct BlindInfo {
        small: u32,
        big: u32,
        boss: u32,
    }
    
    impl Response for BlindInfo {
    }
    
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