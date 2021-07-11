use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use std::io::Cursor;

pub struct BinaryResponse {
    pub content_type: ContentType,
    pub body: Vec<u8>,
}

impl<'r> Responder<'r, 'static> for BinaryResponse {
    fn respond_to(self, _: &Request) -> response::Result<'static> {
        Response::build()
            .sized_body(self.body.len(), Cursor::new(self.body))
            .header(self.content_type)
            .ok()
    }
}
