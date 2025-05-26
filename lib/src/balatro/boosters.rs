use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::{balatro_enum, net::Connection, balatro::Error};
use super::{consumables::{PlanetKind, SpectralKind, TarotKind}, deck::PlayingCard, jokers::Joker, overview::GameOverview, Screen};

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

pub trait OpenBooster<'a>: Sized + Screen<'a> {
    type Options: for<'de> Deserialize<'de>;
    type ReturnTo: Screen<'a>;
    fn booster(&self) -> BoosterKind;
    fn selections_left(&self) -> SelectionsLeft;
    fn options(&self) -> &[Self::Options];
    async fn skip(self) -> Result<Self::ReturnTo, Error>;
}

pub trait OpenPlainBooster<'a>: Sized + OpenBooster<'a> {
    async fn select(self, index: u32) -> Result<PlainSelectResult<'a, Self>, Error>;
}

pub trait OpenCardBooster<'a>: Sized + OpenBooster<'a> {
    async fn hand(&self) -> &[BoosterCard];
    async fn click(self, indices: &[u32]) -> Result<Self, Error>;
    async fn select(self, index: u32) -> Result<HandSelectResult<'a, Self>, Error>;
}

#[derive(Serialize, Deserialize)]
pub struct BoosterCard {
    card: PlayingCard,
    selected: bool,
}

macro_rules! impl_open_booster {
    ($ty:ident, $options:ty) => {
        impl<'a, S : Screen<'a>> OpenBooster<'a> for $ty<'a, S> {
            type Options = $options;
            type ReturnTo = S;
            fn booster(&self) -> BoosterKind {
                self.info.booster
            }
        
            fn selections_left(&self) -> SelectionsLeft {
                self.info.selections_left
            }
        
            fn options(&self) -> &[Self::Options] {
                &self.info.options
            }
        
            async fn skip(self) -> Result<Self::ReturnTo, Error> {
                let response = self.connection.request(protocol::SkipBooster::<'a, Self> {
                    _r_marker: std::marker::PhantomData,
                }).await??;
                Ok(Self::ReturnTo::new(response, self.connection))
            }
        }
    };
}

macro_rules! impl_open_plain_booster {
    ($ty:ident, $options:ty) => {
        impl_open_booster!($ty, $options);

        impl<'a, S : Screen<'a>> OpenPlainBooster<'a> for $ty<'a, S> {
            async fn select(self, index: u32) -> Result<PlainSelectResult<'a, Self>, Error> {
                let response = self.connection.request(protocol::SelectBoosterOption::<'a, Self, protocol::BoosterSelectResult<'a, Self>> {
                    index,
                    _r_marker: std::marker::PhantomData,
                    _r_marker2: std::marker::PhantomData,
                }).await??;
                match response {
                    protocol::BoosterSelectResult::Again(info) => Ok(PlainSelectResult::Again(Self::new(info, self.connection))),
                    protocol::BoosterSelectResult::Done(result) => Ok(PlainSelectResult::Done(S::new(result, self.connection))),
                }
            }
        }
    };
}

macro_rules! impl_open_card_booster {
    ($ty:ident, $options:ty) => {
        impl_open_booster!($ty, $options);

        impl<'a, S : Screen<'a> + 'a> OpenCardBooster<'a> for $ty<'a, S> {
            async fn hand(&self) -> &[BoosterCard] {
                &self.info.hand
            }
            
            async fn click(self, indices: &[u32]) -> Result<Self, Error> {
                let response = self.connection.request(protocol::ClickCardBooster::<'a, Self> {
                    indices: indices.to_vec(),
                    _r_marker: std::marker::PhantomData,
                    _c_marker: std::marker::PhantomData,
                }).await??;
                Ok(Self::new(response, self.connection))
            }

            async fn select(self, index: u32) -> Result<HandSelectResult<'a, Self>, Error> {
                let response = self.connection.request(protocol::SelectBoosterOption::<'a, Self, protocol::CardBoosterSelectResult<'a, Self>> {
                    index,
                    _r_marker: std::marker::PhantomData,
                    _r_marker2: std::marker::PhantomData,
                }).await??;
                match response {
                    protocol::CardBoosterSelectResult::Again(info) => Ok(HandSelectResult::Again(Self::new(info, self.connection))),
                    protocol::CardBoosterSelectResult::Done(result) => Ok(HandSelectResult::Done(S::new(result, self.connection))),
                    protocol::CardBoosterSelectResult::GameOver => Ok(HandSelectResult::GameOver(GameOverview::new(self.connection))),
                }
            }
        }
    }
}


pub struct OpenTarot<'a, S : Screen<'a> + 'a> {
    info: protocol::OpenCardBoosterInfo<'a, Self>,
    connection: &'a mut Connection,
}

impl_open_card_booster!(OpenTarot, TarotOption);

impl <'a, S : Screen<'a>> Screen<'a> for OpenTarot<'a, S> {
    type Info = protocol::OpenCardBoosterInfo<'a, Self>;
    fn name() -> &'static str {
        "tarot"
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}

pub struct OpenBuffoon<'a, S : Screen<'a> + 'a> {
    info: protocol::OpenBoosterInfo<'a, Self>,
    connection: &'a mut Connection,
}

impl_open_plain_booster!(OpenBuffoon, Joker);

impl <'a, S : Screen<'a>> Screen<'a> for OpenBuffoon<'a, S> {
    type Info = protocol::OpenBoosterInfo<'a, Self>;
    fn name() -> &'static str {
        "buffoon"
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}

pub struct OpenCelestial<'a, S : Screen<'a> + 'a> {
    info: protocol::OpenBoosterInfo<'a, Self>,
    connection: &'a mut Connection,
}

impl_open_plain_booster!(OpenCelestial, PlanetOption);

impl <'a, S : Screen<'a>> Screen<'a> for OpenCelestial<'a, S> {
    type Info = protocol::OpenBoosterInfo<'a, Self>;
    fn name() -> &'static str {
        "celestial"
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}

pub struct OpenSpectral<'a, S : Screen<'a> + 'a> {
    info: protocol::OpenCardBoosterInfo<'a, Self>,
    connection: &'a mut Connection,
}

impl_open_card_booster!(OpenSpectral, SpectralOption);

impl <'a, S : Screen<'a>> Screen<'a> for OpenSpectral<'a, S> {
    type Info = protocol::OpenCardBoosterInfo<'a, Self>;
    fn name() -> &'static str {
        "spectral"
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}

pub struct OpenStandard<'a, S : Screen<'a> + 'a> {
    info: protocol::OpenBoosterInfo<'a, Self>,
    connection: &'a mut Connection,
}

impl_open_plain_booster!(OpenStandard, PlayingCard);

impl <'a, S : Screen<'a>> Screen<'a> for OpenStandard<'a, S> {
    type Info = protocol::OpenBoosterInfo<'a, Self>;
    fn name() -> &'static str {
        "standard"
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

pub enum PlainSelectResult<'a, O: OpenPlainBooster<'a>> {
    Again(O),
    Done(O::ReturnTo),
}

pub enum HandSelectResult<'a, O: OpenCardBooster<'a>> {
    Again(O),
    Done(O::ReturnTo),
    GameOver(GameOverview<'a>),
}

#[derive(Serialize, Deserialize)]
pub enum SpectralOption {
    Normal(SpectralKind),
    BlackHole,
    Soul
}

#[derive(Serialize, Deserialize)]
pub enum TarotOption {
    Normal(TarotKind),
    Spectral(SpectralOption),
    Soul
}

#[derive(Serialize, Deserialize)]
pub enum PlanetOption {
    Normal(PlanetKind),
    BlackHole
}

pub(crate) mod protocol {
    use serde::{Deserialize, Serialize};
    use crate::{balatro::{deck::PlayingCard, jokers::Joker}, net::protocol::{Packet, Request, Response}};
    use super::{BoosterKind, BoosterCard, OpenBooster, OpenCardBooster, PlanetOption, SelectionsLeft, SpectralOption, TarotOption};
    use crate::balatro::Screen;

    #[derive(Deserialize)]
    pub struct OpenBoosterInfo<'a, B: OpenBooster<'a>> {
        pub booster: BoosterKind,
        pub options: Vec<B::Options>,
        pub selections_left: SelectionsLeft
    }

    impl<'a, B: OpenBooster<'a>> Response for OpenBoosterInfo<'a, B> {}

    impl<'a, B: OpenBooster<'a>> Packet for OpenBoosterInfo<'a, B> {
        fn kind() -> String {
            format!("open_booster/{}/info", B::name())
        }
    }

    #[derive(Deserialize)]
    pub struct OpenCardBoosterInfo<'a, B: OpenCardBooster<'a>> {
        pub booster: BoosterKind,
        pub options: Vec<B::Options>,
        pub selections_left: SelectionsLeft,
        pub hand: Vec<BoosterCard>
    }

    impl<'a, B: OpenCardBooster<'a>> Response for OpenCardBoosterInfo<'a, B> {}

    impl<'a, B: OpenCardBooster<'a>> Packet for OpenCardBoosterInfo<'a, B> {
        fn kind() -> String {
            format!("open_booster/{}/info", B::name())
        }
    }

    #[derive(Serialize)]
    pub struct SelectBoosterOption<'a, B: OpenBooster<'a>, R: Response> {
        pub index: u32,
        pub _r_marker: std::marker::PhantomData<&'a B>,
        pub _r_marker2: std::marker::PhantomData<R>,
    }

    impl<'a, B: OpenBooster<'a>, R: Response> Request for SelectBoosterOption<'a, B, R> {
        type Expect = Result<R, String>;
    }

    impl<'a, B: OpenBooster<'a>, R: Response> Packet for SelectBoosterOption<'a, B, R> {
        fn kind() -> String {
            format!("open_booster/{}/select", B::name())
        }
    }
    
    #[derive(Deserialize)]
    pub enum BoosterSelectResult<'a, B: OpenBooster<'a>> {
        Again(B::Info),
        Done(<B::ReturnTo as Screen<'a>>::Info),
    }

    impl<'a, B: OpenBooster<'a>> Response for BoosterSelectResult<'a, B> {}

    impl<'a, B: OpenBooster<'a>> Packet for BoosterSelectResult<'a, B> {
        fn kind() -> String {
            format!("open_booster/{}/select", B::name())
        }
    }

    #[derive(Deserialize)]
    pub enum CardBoosterSelectResult<'a, B: OpenCardBooster<'a>> {
        Again(B::Info),
        Done(<B::ReturnTo as Screen<'a>>::Info),
        GameOver,
    }

    impl<'a, B: OpenCardBooster<'a>> Response for CardBoosterSelectResult<'a, B> {}

    impl<'a, B: OpenCardBooster<'a>> Packet for CardBoosterSelectResult<'a, B> {
        fn kind() -> String {
            format!("open_booster/{}/select", B::name())
        }
    }
    

    #[derive(Serialize)]
    pub struct SkipBooster<'a, B: OpenBooster<'a>> {
        pub _r_marker: std::marker::PhantomData<&'a B>,
    }

    impl<'a, B: OpenBooster<'a>> Request for SkipBooster<'a, B> {
        type Expect = Result<<B::ReturnTo as Screen<'a>>::Info, String>;
    }

    impl<'a, B: OpenBooster<'a>> Packet for SkipBooster<'a, B> {
        fn kind() -> String {
            format!("open_booster/{}/skip", B::name())
        }
    }

    #[derive(Serialize)]
    pub struct ClickCardBooster<'a, B: OpenCardBooster<'a>> {
        pub indices: Vec<u32>,
        pub _r_marker: std::marker::PhantomData<&'a B>,
        pub _c_marker: std::marker::PhantomData<&'a B>,
    }

    impl<'a, B: OpenCardBooster<'a>> Request for ClickCardBooster<'a, B> {
        type Expect = Result<OpenCardBoosterInfo<'a, B>, String>;
    }   

    impl<'a, B: OpenCardBooster<'a>> Packet for ClickCardBooster<'a, B> {
        fn kind() -> String {
            format!("open_booster/{}/click", B::name())
        }
    }
    
    
}