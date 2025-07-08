use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::{balatro_enum, net::{protocol::Response, Connection}, balatro::Error};
use super::{consumables::{PlanetKind, SpectralKind, TarotKind}, deck::PlayingCard, jokers::Joker, Screen};

macro_rules! impl_hud_generic {
    ($($t:ident),*) => {
        $(
            impl<'a, R : Response + 'a> crate::balatro::hud::Hud<'a> for $t<'a, R> {
                fn hands(&self) -> u32 {
                    self.info.hud.hands
                }

                fn discards(&self) -> u32 {
                    self.info.hud.discards
                }

                fn round(&self) -> u32 {
                    self.info.hud.round
                }

                fn ante(&self) -> u32 {
                    self.info.hud.ante
                }

                fn money(&self) -> u32 {
                    self.info.hud.money
                }

                fn jokers(&self) -> &[crate::balatro::jokers::Joker] {
                    &self.info.hud.jokers
                }

                fn tags(&self) -> &[crate::balatro::blinds::Tag] {
                    &self.info.hud.tags
                }

                fn run_info(&self) -> &crate::balatro::hud::RunInfo {
                    &self.info.hud.run_info
                }

                async fn move_joker(self, from: u32, to: u32) -> Result<Self, crate::balatro::Error> {
                    let new_info = self
                        .connection
                        .request(crate::balatro::hud::protocol::MoveJoker { from, to, _marker: std::marker::PhantomData::<&$t<'a, R>> })
                        .await??;
                    Ok(Self::new(new_info, self.connection))
                }

                async fn sell_joker(self, index: u32) -> Result<Self, crate::balatro::Error> {
                    let new_info = self
                        .connection
                        .request(crate::balatro::hud::protocol::SellJoker { index, _marker: std::marker::PhantomData::<&$t<'a, R>> })
                        .await??;
                    Ok(Self::new(new_info, self.connection))
                }

                fn consumables(&self) -> &[crate::balatro::consumables::Consumable] {
                    &self.info.hud.consumables
                }

                async fn move_consumable(self, from: u32, to: u32) -> Result<Self, crate::balatro::Error> {
                    let new_info = self
                        .connection
                        .request(crate::balatro::hud::protocol::MoveConsumable { from, to, _marker: std::marker::PhantomData::<&$t<'a, R>> })
                        .await??;
                    Ok(Self::new(new_info, self.connection))
                }

                async fn use_consumable(self, index: u32) -> Result<Self, crate::balatro::Error> {
                    let new_info = self
                        .connection
                        .request(crate::balatro::hud::protocol::UseConsumable { index, _marker: std::marker::PhantomData::<&$t<'a, R>> })
                        .await??;
                    Ok(Self::new(new_info, self.connection))
                }

                async fn sell_consumable(self, index: u32) -> Result<Self, crate::balatro::Error> {
                    let new_info = self
                        .connection
                        .request(crate::balatro::hud::protocol::SellConsumable { index, _marker: std::marker::PhantomData::<&$t<'a, R>> })
                        .await??;
                    Ok(Self::new(new_info, self.connection))
                }
            }
        )*
    };
}

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
    type ReturnTo: Response + 'a;
    fn booster(&self) -> BoosterPackKind;
    fn selections_left(&self) -> SelectionsLeft;
    fn options(&self) -> &[Self::Options];
    async fn select(self, index: u32) -> Result<SelectResult<'a, Self>, Error>;
    async fn skip(self) -> Result<Self::ReturnTo, Error>;
}

#[allow(async_fn_in_trait)]
pub trait OpenWithHand<'a>: Sized + Open<'a> {
    async fn hand(&self) -> &[BoosterCard];
    async fn click(self, indices: &[u32]) -> Result<Self, Error>;
}

#[derive(Serialize, Deserialize)]
pub struct BoosterCard {
    pub card: PlayingCard,
    pub selected: bool,
}

macro_rules! impl_open {
    ($ty:ident, $options:ty) => {
        impl<'a, R : Response + 'a> Open<'a> for $ty<'a, R> {
            type Options = $options;
            type ReturnTo = R;
            fn booster(&self) -> BoosterPackKind {
                self.info.booster
            }
        
            fn selections_left(&self) -> SelectionsLeft {
                self.info.selections_left
            }
        
            fn options(&self) -> &[Self::Options] {
                &self.info.options
            }

            async fn select(self, index: u32) -> Result<SelectResult<'a, Self>, Error> {
                let response = self.connection.request(protocol::OpenSelect::<'a, Self> {
                    index,
                    _r_marker: std::marker::PhantomData,
                }).await??;
                match response {
                    protocol::SelectResult::Again(info) => Ok(SelectResult::Again(Self::new(info, self.connection))),
                    protocol::SelectResult::Done(result) => Ok(SelectResult::Done(result)),
                }
            }
        
            async fn skip(self) -> Result<Self::ReturnTo, Error> {
                let response = self.connection.request(protocol::BoosterPackSkip::<'a, Self> {
                    _r_marker: std::marker::PhantomData,
                }).await??;
                Ok(response)
            }
        }
    };
}

macro_rules! impl_open_with_hand {
    ($ty:ident, $options:ty) => {
       impl_open!($ty, $options);

        impl<'a, R : Response + 'a> OpenWithHand<'a> for $ty<'a, R> {
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
        }
    }
}

pub enum OpenBoosterPack<'a, R : Response + 'a> {
    Arcana(OpenArcanaPack<'a, R>),
    Buffoon(OpenBuffoonPack<'a, R>),
    Celestial(OpenCelestialPack<'a, R>),
    Spectral(OpenSpectralPack<'a, R>),
    Standard(OpenStandardPack<'a, R>),
}

pub struct OpenArcanaPack<'a, R : Response + 'a> {
    info: protocol::OpenWithHandInfo<'a, Self>,
    connection: &'a mut Connection,
}

impl_open_with_hand!(OpenArcanaPack, TarotOption);

impl <'a, R : Response + 'a> Screen<'a> for OpenArcanaPack<'a, R> {
    type Info = protocol::OpenWithHandInfo<'a, Self>;
    fn name() -> String {
        format!("{}/open/arcana", R::kind())
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}


pub struct OpenBuffoonPack<'a, R : Response + 'a> {
    info: protocol::OpenInfo<'a, Self>,
    connection: &'a mut Connection,
}

impl_open!(OpenBuffoonPack, Joker);

impl <'a, R : Response + 'a> Screen<'a> for OpenBuffoonPack<'a, R> {
    type Info = protocol::OpenInfo<'a, Self>;
    fn name() -> String {
        format!("{}/open/buffoon", R::kind())
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}


pub struct OpenCelestialPack<'a, R : Response + 'a> {
    info: protocol::OpenInfo<'a, Self>,
    connection: &'a mut Connection,
}

impl_open!(OpenCelestialPack, PlanetOption);

impl <'a, R : Response + 'a> Screen<'a> for OpenCelestialPack<'a, R> {
    type Info = protocol::OpenInfo<'a, Self>;
    fn name() -> String {
        format!("{}/open/celestial", R::kind())
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}


pub struct OpenSpectralPack<'a, R : Response + 'a> {
    info: protocol::OpenWithHandInfo<'a, Self>,
    connection: &'a mut Connection,
}

impl_open_with_hand!(OpenSpectralPack, SpectralOption);

impl <'a, R : Response + 'a> Screen<'a> for OpenSpectralPack<'a, R> {
    type Info = protocol::OpenWithHandInfo<'a, Self>;
    fn name() -> String {
        format!("{}/open/spectral", R::kind())
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}


pub struct OpenStandardPack<'a, R : Response + 'a> {
    info: protocol::OpenInfo<'a, Self>,
    connection: &'a mut Connection,
}

impl_open!(OpenStandardPack, PlayingCard);

impl <'a, R : Response + 'a> Screen<'a> for OpenStandardPack<'a, R> {
    type Info = protocol::OpenInfo<'a, Self>;
    fn name() -> String {
        format!("{}/open/standard", R::kind())
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}

impl_hud_generic!(OpenArcanaPack, OpenBuffoonPack, OpenCelestialPack, OpenSpectralPack, OpenStandardPack);

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionsLeft {
    One = 1,
    Two = 2,
}

pub enum SelectResult<'a, O: Open<'a>> {
    Again(O),
    Done(O::ReturnTo),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SpectralOption {
    Spectral(SpectralKind),
    BlackHole,
    Soul
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TarotOption {
    Tarot(TarotKind),
    Spectral(SpectralOption),
    Soul
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PlanetOption {
    Planet(PlanetKind),
    BlackHole
}

pub(crate) mod protocol {
    use serde::{Deserialize, Serialize};
    use crate::net::protocol::{Packet, Request, Response};
    use super::{BoosterPackKind, BoosterCard, Open, OpenWithHand, SelectionsLeft};
    use crate::balatro::hud::protocol::HudInfo;

    #[derive(Deserialize)]
    pub struct OpenInfo<'a, B: Open<'a>> {
        pub hud: HudInfo,
        pub booster: BoosterPackKind,
        pub options: Vec<B::Options>,
        pub selections_left: SelectionsLeft
    }

    impl<'a, B: Open<'a>> Response for OpenInfo<'a, B> {}

    impl<'a, B: Open<'a>> Packet for OpenInfo<'a, B> {
        fn kind() -> String {
            format!("{}/info", B::name())
        }
    }

    #[derive(Deserialize)]
    pub struct OpenWithHandInfo<'a, B: OpenWithHand<'a>> {
        pub hud: HudInfo,
        pub booster: BoosterPackKind,
        pub options: Vec<B::Options>,
        pub selections_left: SelectionsLeft,
        pub hand: Vec<BoosterCard>
    }

    impl<'a, B: OpenWithHand<'a>> Response for OpenWithHandInfo<'a, B> {}

    impl<'a, B: OpenWithHand<'a>> Packet for OpenWithHandInfo<'a, B> {
        fn kind() -> String {
            format!("{}/info", B::name())
        }
    }

    #[derive(Serialize)]
    pub struct OpenSelect<'a, B: Open<'a>> {
        pub index: u32,
        pub _r_marker: std::marker::PhantomData<&'a B>,
    }

    impl<'a, B: Open<'a>> Request for OpenSelect<'a, B> {
        type Expect = Result<SelectResult<'a, B>, String>;
    }

    impl<'a, B: Open<'a>> Packet for OpenSelect<'a, B> {
        fn kind() -> String {
            format!("{}/select", B::name())
        }
    }
    
    #[derive(Deserialize)]
    pub enum SelectResult<'a, B: Open<'a>> {
        Again(B::Info),
        Done(B::ReturnTo),
    }

    impl<'a, B: Open<'a>> Response for SelectResult<'a, B> {}

    impl<'a, B: Open<'a>> Packet for SelectResult<'a, B> {
        fn kind() -> String {
            format!("{}/select", B::name())
        }
    }
    

    #[derive(Serialize)]
    pub struct BoosterPackSkip<'a, B: Open<'a>> {
        pub _r_marker: std::marker::PhantomData<&'a B>,
    }

    impl<'a, B: Open<'a>> Request for BoosterPackSkip<'a, B> {
        type Expect = Result<B::ReturnTo, String>;
    }

    impl<'a, B: Open<'a>> Packet for BoosterPackSkip<'a, B> {
        fn kind() -> String {
            format!("{}/skip", B::name())
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
            format!("{}/click", B::name())
        }
    }
    
    
}
