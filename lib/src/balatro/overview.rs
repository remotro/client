use crate::net::Connection;
use crate::balatro::{
    Error,
    shop::Shop,
};

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

    pub async fn cash_out(self) -> Result<Shop<'a>, Error> {
        let info = self.connection.request(protocol::CashOut).await??;
        Ok(Shop::new(info, self.connection))
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
    use crate::{
        net::protocol::{Packet, Request, Response},
        balatro::shop::protocol::ShopInfo,
    };

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

    #[derive(Serialize, Deserialize, Clone)]
    pub struct CashOut;

    impl Request for CashOut {
        type Expect = Result<ShopInfo, String>;
    }

    impl Packet for CashOut {
        fn kind() -> String {
            "overview/cash_out".to_string()
        }
    }
}
