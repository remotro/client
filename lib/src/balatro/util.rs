#[macro_export]
macro_rules! balatro_enum {
    ($name:ident { $($item:ident = $identifier:literal),* $(,)? }) => {
        #[derive(Serialize, Deserialize, Clone, Debug)]
        enum $name {
            $(
                #[serde(rename = $identifier)]
                $item,
            )*
        }
    };
}
