use chrono::NaiveDateTime;
use rocket::form::{self, DataField, FromFormField, ValueField};

#[derive(Debug)]
pub struct NaiveDateTimeRocket(NaiveDateTime);

#[rocket::async_trait]
impl<'r> FromFormField<'r> for NaiveDateTimeRocket {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        print!("parse {}", field.value);
        let result = NaiveDateTime::parse_from_str(field.value, "%Y-%m-%dT%H:%M:%S");
        match result {
            Ok(val) => Ok(NaiveDateTimeRocket(val)),
            Err(_) => Err(form::Error::validation(
                "not a valid date time (expected format %Y-%m-%dT%H:%M:%S)",
            ))?,
        }
    }

    async fn from_data(_field: DataField<'r, '_>) -> form::Result<'r, Self> {
        todo!("parse from a value or use default impl")
    }
}

impl From<NaiveDateTimeRocket> for NaiveDateTime {
    fn from(item: NaiveDateTimeRocket) -> Self {
        item.0
    }
}
