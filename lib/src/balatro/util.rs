#[macro_export]
macro_rules! balatro_enum {
    ($name:ident { $($item:ident = $identifier:literal),* $(,)? }) => {
        #[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $(
                #[serde(rename = $identifier)]
                $item,
            )*
        }
    };
}
