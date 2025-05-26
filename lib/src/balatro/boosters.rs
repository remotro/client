

use protocol::{BoosterSelect, BoosterSelectResult, BoosterSkip};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::{balatro_enum, net::Connection, balatro::Error};
use super::Screen;

balatro_enum!(BoosterKind {
    ArcanaNormal = "p_arcana_normal",
    ArcanaMega = "p_arcana_mega",
    ArcanaJumbo = "p_arcana_jumbo",
    BuffoonNormal = "p_buffoon_normal",
    BuffoonMega = "p_buffoon_mega",
    BuffoonJumbo = "p_buffoon_jumbo",
    CelestialNormal = "p_celestial_normal",
    CelestialMega = "p_celestial_mega",
    CelestialJumbo = "p_celestial_jumbo",
    SpectralNormal = "p_spectral_normal",
    SpectralMega = "p_spectral_mega",
    SpectralJumbo = "p_spectral_jumbo",
    StandardNormal = "p_standard_normal",
    StandardMega = "p_standard_mega",
    StandardJumbo = "p_standard_jumbo",
});

pub struct OpenBooster<'a, C : for<'de> Deserialize<'de>, S : Screen<'a>> {
    connection: &'a mut Connection,
    info: protocol::BoosterInfo<'a, C, S::Info>,
}

impl <'a, C : for<'de> Deserialize<'de>, S : Screen<'a>> OpenBooster<'a, C, S> {
    pub fn booster(&self) -> BoosterKind {
        self.info.booster
    }

    pub fn selections_left(&self) -> SelectionsLeft {
        self.info.selections_left
    }

    pub fn options(&self) -> &[C] {
        &self.info.options
    }

    async fn skip(self) -> Result<S, Error> {
        let response = self.connection.request(BoosterSkip::<'a, S::Info> {
            _r_marker: std::marker::PhantomData,
        }).await?;
        Ok(S::new(response, self.connection))
    }

    async fn select(self, index: u32) -> Result<SelectResult<'a, C, S>, Error> {
        let response = self.connection.request(BoosterSelect::<'a, C, S::Info> {
            index,
            _r_marker: std::marker::PhantomData,
            _c_marker: std::marker::PhantomData,
        }).await?;
        match response {
            BoosterSelectResult::Again(info) => Ok(SelectResult::Again(OpenBooster::new(info, self.connection))),
            BoosterSelectResult::Done(result) => Ok(SelectResult::Done(S::new(result, self.connection))),
        }
    }
}

impl <'a, C : for<'de> Deserialize<'de>, S : Screen<'a>> Screen<'a> for OpenBooster<'a, C, S> {
    type Info = protocol::BoosterInfo<'a, C, S::Info>;
    fn name() -> &'static str {
        "booster"
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionsLeft {
    One = 1,
    Two = 2,
}

pub enum SelectResult<'a, C : DeserializeOwned, S : Screen<'a>> {
    Again(OpenBooster<'a, C, S>),
    Done(S),
}

pub(crate) mod protocol {
    use serde::{Deserialize, Serialize};
    use crate::net::protocol::{Packet, Request, Response};
    use super::{BoosterKind, SelectionsLeft};

    #[derive(Deserialize)]
    pub struct BoosterInfo<'a, C, R> {
        pub booster: BoosterKind,
        pub options: Vec<C>,
        pub selections_left: SelectionsLeft,
        pub _r_marker: std::marker::PhantomData<&'a R>,
        pub _c_marker: std::marker::PhantomData<&'a C>,
    }

    impl<'a, C: for<'de> Deserialize<'de>, R: for<'de> Deserialize<'de>> Response for BoosterInfo<'a, C, R> {}

    impl<'a, C: for<'de> Deserialize<'de>, R:for<'de> Deserialize<'de>> Packet for BoosterInfo<'a, C, R> {
        fn kind() -> String {
            "booster/info".to_string()
        }
    }

    #[derive(Serialize)]
    pub struct BoosterSelect<'a, C, R> {
        pub index: u32,
        pub _r_marker: std::marker::PhantomData<&'a R>,
        pub _c_marker: std::marker::PhantomData<&'a C>, 
    }

    impl<'a, C: for<'de> Deserialize<'de>, R: for<'de> Deserialize<'de>> Request for BoosterSelect<'a, C, R> {
        type Expect = BoosterSelectResult<'a, C, R>;
    }

    impl<C: for<'de> Deserialize<'de>, R: for<'de> Deserialize<'de>> Packet for BoosterSelect<'_, C, R> {
        fn kind() -> String {
            "booster/select".to_string()
        }
    }

    #[derive(Deserialize)]
    pub enum BoosterSelectResult<'a, C, R> {
        Again(BoosterInfo<'a, C, R>),
        Done(R),
    }

    impl<C: for<'de> Deserialize<'de>, R: for<'de> Deserialize<'de>> Response for BoosterSelectResult<'_, C, R> {}

    impl<C: for<'de> Deserialize<'de>, R: for<'de> Deserialize<'de>> Packet for BoosterSelectResult<'_, C, R> {
        fn kind() -> String {
            "booster/select/result".to_string()
        }
    }

    #[derive(Serialize)]
    pub struct BoosterSkip<'a, R : Response> {
        pub _r_marker: std::marker::PhantomData<&'a R>,
    }

    impl<R: Response> Request for BoosterSkip<'_, R> {
        type Expect = R;
    }

    impl<R: Response> Packet for BoosterSkip<'_, R> {
        fn kind() -> String {
            "booster/skip".to_string()  
        }
    }
}