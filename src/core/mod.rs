mod resize;

pub use resize::resize;

pub enum ImageSize {
    Medium,
    Small,
    Thumbnail,
}
