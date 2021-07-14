pub mod image_event_handler;
pub mod modules;
mod resize;

pub use resize::resize;

pub enum ImageSize {
    Medium,
    Small,
    Thumbnail,
}
