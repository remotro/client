use crate::net::Connection;
pub struct RoundOverview<'a> {
    connection: &'a mut Connection,
    info: protocol::RoundOverviewInfo,
}

impl<'a> RoundOverview<'a> {
    pub(crate) fn new(info: protocol::RoundOverviewInfo, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }

    pub fn total_money(&self) -> u64 {
        self.info.total_money
    }
}

pub struct GameOverview<'a> {
    connection: &'a mut Connection,
}

impl<'a> GameOverview<'a> {
    pub(crate) fn new(connection: &'a mut Connection) -> Self {
        Self { connection }
    }
}

pub(crate) mod protocol {
    use serde::{Deserialize, Serialize};

    use crate::net::protocol::{Packet, Request, Response};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct RoundOverviewInfo {
        pub total_money: u64,
    }

    impl Response for RoundOverviewInfo {}

    impl Packet for RoundOverviewInfo {
        fn kind() -> String {
            "overview/round".to_string()
        }
    }
}
