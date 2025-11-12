#[doc(hidden)]
#[macro_export]
macro_rules! balatro_enum {
    // Handle mixed variants
    ($name:ident {
        $(
            $variant:ident $(
                { $($field:ident: $field_type:ty),* $(,)? }
            )? = $identifier:literal
        ),*
        $(,)?
    }) => {
        #[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
        pub enum $name {
            $(
                #[serde(rename = $identifier)]
                $variant $({ $($field: $field_type),* })?,
            )*
        }
    };
}
