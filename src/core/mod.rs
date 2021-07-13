mod resize;
pub mod image_event_handler;

pub use resize::resize;

pub enum ImageSize {
    Medium,
    Small,
    Thumbnail,
}
