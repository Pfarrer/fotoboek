use strum_macros::{EnumString, ToString};

#[derive(PartialEq, EnumString, ToString)]
pub enum ImageSize {
    #[strum(serialize = "large")]
    Large,
    #[strum(serialize = "small")]
    Small,
}