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
    }
    
    impl Response for BlindInfo {
    }
    
    impl Packet for BlindInfo {
        fn kind() -> &'static str {
            "blind_select/info"
        }
    }

    #[derive(Serialize)]
    pub struct SelectBlind;

    impl Request for SelectBlind {
        type Expect = Result<Vec<()>, String>;
    }

    impl Packet for SelectBlind {
        fn kind() -> &'static str {
            "blind_select/select"
        }
    }

    #[derive(Serialize)]
    pub struct SkipBlind;

    impl Request for SkipBlind {
        type Expect = Result<Vec<()>, String>;
    }

    impl Packet for SkipBlind {
        fn kind() -> &'static str {
            "blind_select/skip"
        }
    }
    
}