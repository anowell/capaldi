use chrono::NaiveDate;
use chrono::ParseError;
use rocket::form::{self, DataField, FromFormField, ValueField};
use rocket::request::FromParam;

// https://github.com/SergioBenitez/Rocket/issues/602#issuecomment-380497269
pub struct NaiveDateForm(pub NaiveDate);

impl<'a> FromParam<'a> for NaiveDateForm {
    type Error = ParseError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match NaiveDate::parse_from_str(&param, "%Y-%m-%d") {
            Ok(date) => Ok(NaiveDateForm(date)),
            Err(e) => Err(e),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for NaiveDateForm {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match NaiveDate::parse_from_str(field.value, "%Y-%m-%d") {
            Ok(date) => Ok(NaiveDateForm(date)),
            Err(_) => Err(form::Error::validation("invalid date format"))?,
        }
    }

    async fn from_data(_field: DataField<'r, '_>) -> form::Result<'r, Self> {
        unimplemented!("did not implement parsing data field as data")
    }
}
