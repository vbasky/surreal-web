use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

use derive_more::Display;
#[derive(Debug, Display)]
pub enum UserError {
    NoUserFound = 0,
    UserCreationFailure = 1,
    NoSuchUserFound = 2,
}
