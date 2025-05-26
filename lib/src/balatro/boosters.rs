use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::{balatro_enum, net::Connection, balatro::Error};
use super::{consumables::{PlanetKind, SpectralKind, TarotKind}, deck::PlayingCard, jokers::Joker, overview::GameOverview, Screen};

balatro_enum!(BoosterPackKind {
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

#[allow(async_fn_in_trait)]
pub trait Open<'a>: Sized + Screen<'a> {
    type Options: for<'de> Deserialize<'de>;
    type ReturnTo: Screen<'a>;
    fn booster(&self) -> BoosterPackKind;
    fn selections_left(&self) -> SelectionsLeft;
    fn options(&self) -> &[Self::Options];
    async fn skip(self) -> Result<Self::ReturnTo, Error>;
}

#[allow(async_fn_in_trait)]
pub trait OpenBare<'a>: Sized + Open<'a> {
    async fn select(self, index: u32) -> Result<BareSelectResult<'a, Self>, Error>;
}

#[allow(async_fn_in_trait)]
pub trait OpenWithHand<'a>: Sized + Open<'a> {
    async fn hand(&self) -> &[BoosterCard];
    async fn click(self, indices: &[u32]) -> Result<Self, Error>;
    async fn select(self, index: u32) -> Result<HandSelectResult<'a, Self>, Error>;
}

#[derive(Serialize, Deserialize)]
pub struct BoosterCard {
    card: PlayingCard,
    selected: bool,
}

macro_rules! impl_open {
    ($ty:ident, $options:ty) => {
        impl<'a, S : Screen<'a>> Open<'a> for $ty<'a, S> {
            type Options = $options;
            type ReturnTo = S;
            fn booster(&self) -> BoosterPackKind {
                self.info.booster
            }
        
            fn selections_left(&self) -> SelectionsLeft {
                self.info.selections_left
            }
        
            fn options(&self) -> &[Self::Options] {
                &self.info.options
            }
        
            async fn skip(self) -> Result<Self::ReturnTo, Error> {
                let response = self.connection.request(protocol::BoosterPackSkip::<'a, Self> {
                    _r_marker: std::marker::PhantomData,
                }).await??;
                Ok(Self::ReturnTo::new(response, self.connection))
            }
        }
    };
}

macro_rules! impl_open_bare {
    ($ty:ident, $options:ty) => {
       impl_open!($ty, $options);

        impl<'a, S : Screen<'a>> OpenBare<'a> for $ty<'a, S> {
            async fn select(self, index: u32) -> Result<BareSelectResult<'a, Self>, Error> {
                let response = self.connection.request(protocol::OpenSelect::<'a, Self, protocol::BareBoosterPackSelectResult<'a, Self>> {
                    index,
                    _r_marker: std::marker::PhantomData,
                    _r_marker2: std::marker::PhantomData,
                }).await??;
                match response {
                    protocol::BareBoosterPackSelectResult::Again(info) => Ok(BareSelectResult::Again(Self::new(info, self.connection))),
                    protocol::BareBoosterPackSelectResult::Done(result) => Ok(BareSelectResult::Done(S::new(result, self.connection))),
                }
            }
        }
    };
}

macro_rules! impl_open_with_hand {
    ($ty:ident, $options:ty) => {
       impl_open!($ty, $options);

        impl<'a, S : Screen<'a> + 'a> OpenWithHand<'a> for $ty<'a, S> {
            async fn hand(&self) -> &[BoosterCard] {
                &self.info.hand
            }
            
            async fn click(self, indices: &[u32]) -> Result<Self, Error> {
                let response = self.connection.request(protocol::CardBoosterPackClick::<'a, Self> {
                    indices: indices.to_vec(),
                    _r_marker: std::marker::PhantomData,
                    _c_marker: std::marker::PhantomData,
                }).await??;
                Ok(Self::new(response, self.connection))
            }

            async fn select(self, index: u32) -> Result<HandSelectResult<'a, Self>, Error> {
                let response = self.connection.request(protocol::OpenSelect::<'a, Self, protocol::CardBoosterPackSelectResult<'a, Self>> {
                    index,
                    _r_marker: std::marker::PhantomData,
                    _r_marker2: std::marker::PhantomData,
                }).await??;
                match response {
                    protocol::CardBoosterPackSelectResult::Again(info) => Ok(HandSelectResult::Again(Self::new(info, self.connection))),
                    protocol::CardBoosterPackSelectResult::Done(result) => Ok(HandSelectResult::Done(S::new(result, self.connection))),
                    protocol::CardBoosterPackSelectResult::GameOver => Ok(HandSelectResult::GameOver(GameOverview::new(self.connection))),
                }
            }
        }
    }
}


pub struct OpenArcanaPack<'a, S : Screen<'a> + 'a> {
    info: protocol::OpenWithHandInfo<'a, Self>,
    connection: &'a mut Connection,
}

impl_open_with_hand!(OpenArcanaPack, TarotOption);

impl <'a, S : Screen<'a>> Screen<'a> for OpenArcanaPack<'a, S> {
    type Info = protocol::OpenWithHandInfo<'a, Self>;
    fn name() -> &'static str {
        "arcana"
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}

pub struct OpenBuffoonPack<'a, S : Screen<'a> + 'a> {
    info: protocol::OpenInfo<'a, Self>,
    connection: &'a mut Connection,
}

impl_open_bare!(OpenBuffoonPack, Joker);

impl <'a, S : Screen<'a>> Screen<'a> for OpenBuffoonPack<'a, S> {
    type Info = protocol::OpenInfo<'a, Self>;
    fn name() -> &'static str {
        "buffoon"
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}

pub struct OpenCelestialPack<'a, S : Screen<'a> + 'a> {
    info: protocol::OpenInfo<'a, Self>,
    connection: &'a mut Connection,
}

impl_open_bare!(OpenCelestialPack, PlanetOption);

impl <'a, S : Screen<'a>> Screen<'a> for OpenCelestialPack<'a, S> {
    type Info = protocol::OpenInfo<'a, Self>;
    fn name() -> &'static str {
        "celestial"
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}

pub struct OpenSpectralPack<'a, S : Screen<'a> + 'a> {
    info: protocol::OpenWithHandInfo<'a, Self>,
    connection: &'a mut Connection,
}

impl_open_with_hand!(OpenSpectralPack, SpectralOption);

impl <'a, S : Screen<'a>> Screen<'a> for OpenSpectralPack<'a, S> {
    type Info = protocol::OpenWithHandInfo<'a, Self>;
    fn name() -> &'static str {
        "spectral"
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}

pub struct OpenStandardPack<'a, S : Screen<'a> + 'a> {
    info: protocol::OpenInfo<'a, Self>,
    connection: &'a mut Connection,
}

impl_open_bare!(OpenStandardPack, PlayingCard);

impl <'a, S : Screen<'a>> Screen<'a> for OpenStandardPack<'a, S> {
    type Info = protocol::OpenInfo<'a, Self>;
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

pub enum BareSelectResult<'a, O: OpenBare<'a>> {
    Again(O),
    Done(O::ReturnTo),
}

pub enum HandSelectResult<'a, O: OpenWithHand<'a>> {
    Again(O),
    Done(O::ReturnTo),
    GameOver(GameOverview<'a>),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SpectralOption {
    Normal(SpectralKind),
    BlackHole,
    Soul
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TarotOption {
    Normal(TarotKind),
    Spectral(SpectralOption),
    Soul
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PlanetOption {
    Normal(PlanetKind),
    BlackHole
}

pub(crate) mod protocol {
    use serde::{Deserialize, Serialize};
    use crate::net::protocol::{Packet, Request, Response};
    use super::{BoosterPackKind, BoosterCard, Open, OpenWithHand, SelectionsLeft};
    use crate::balatro::Screen;

    #[derive(Deserialize)]
    pub struct OpenInfo<'a, B: Open<'a>> {
        pub booster: BoosterPackKind,
        pub options: Vec<B::Options>,
        pub selections_left: SelectionsLeft
    }

    impl<'a, B: Open<'a>> Response for OpenInfo<'a, B> {}

    impl<'a, B: Open<'a>> Packet for OpenInfo<'a, B> {
        fn kind() -> String {
            format!("open/{}/info", B::name())
        }
    }

    #[derive(Deserialize)]
    pub struct OpenWithHandInfo<'a, B: OpenWithHand<'a>> {
        pub booster: BoosterPackKind,
        pub options: Vec<B::Options>,
        pub selections_left: SelectionsLeft,
        pub hand: Vec<BoosterCard>
    }

    impl<'a, B: OpenWithHand<'a>> Response for OpenWithHandInfo<'a, B> {}

    impl<'a, B: OpenWithHand<'a>> Packet for OpenWithHandInfo<'a, B> {
        fn kind() -> String {
            format!("open/{}/info", B::name())
        }
    }

    #[derive(Serialize)]
    pub struct OpenSelect<'a, B: Open<'a>, R: Response> {
        pub index: u32,
        pub _r_marker: std::marker::PhantomData<&'a B>,
        pub _r_marker2: std::marker::PhantomData<R>,
    }

    impl<'a, B: Open<'a>, R: Response> Request for OpenSelect<'a, B, R> {
        type Expect = Result<R, String>;
    }

    impl<'a, B: Open<'a>, R: Response> Packet for OpenSelect<'a, B, R> {
        fn kind() -> String {
            format!("open/{}/select", B::name())
        }
    }
    
    #[derive(Deserialize)]
    pub enum BareBoosterPackSelectResult<'a, B: Open<'a>> {
        Again(B::Info),
        Done(<B::ReturnTo as Screen<'a>>::Info),
    }

    impl<'a, B: Open<'a>> Response for BareBoosterPackSelectResult<'a, B> {}

    impl<'a, B: Open<'a>> Packet for BareBoosterPackSelectResult<'a, B> {
        fn kind() -> String {
            format!("open/{}/select", B::name())
        }
    }

    #[derive(Deserialize)]
    pub enum CardBoosterPackSelectResult<'a, B: OpenWithHand<'a>> {
        Again(B::Info),
        Done(<B::ReturnTo as Screen<'a>>::Info),
        GameOver,
    }

    impl<'a, B: OpenWithHand<'a>> Response for CardBoosterPackSelectResult<'a, B> {}

    impl<'a, B: OpenWithHand<'a>> Packet for CardBoosterPackSelectResult<'a, B> {
        fn kind() -> String {
            format!("open/{}/select", B::name())
        }
    }
    

    #[derive(Serialize)]
    pub struct BoosterPackSkip<'a, B: Open<'a>> {
        pub _r_marker: std::marker::PhantomData<&'a B>,
    }

    impl<'a, B: Open<'a>> Request for BoosterPackSkip<'a, B> {
        type Expect = Result<<B::ReturnTo as Screen<'a>>::Info, String>;
    }

    impl<'a, B: Open<'a>> Packet for BoosterPackSkip<'a, B> {
        fn kind() -> String {
            format!("open/{}/skip", B::name())
        }
    }

    #[derive(Serialize)]
    pub struct CardBoosterPackClick<'a, B: OpenWithHand<'a>> {
        pub indices: Vec<u32>,
        pub _r_marker: std::marker::PhantomData<&'a B>,
        pub _c_marker: std::marker::PhantomData<&'a B>,
    }

    impl<'a, B: OpenWithHand<'a>> Request for CardBoosterPackClick<'a, B> {
        type Expect = Result<OpenWithHandInfo<'a, B>, String>;
    }   

    impl<'a, B: OpenWithHand<'a>> Packet for CardBoosterPackClick<'a, B> {
        fn kind() -> String {
            format!("open/{}/click", B::name())
        }
    }
    
    
}