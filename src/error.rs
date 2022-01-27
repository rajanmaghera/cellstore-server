
use std::fmt;
use std::error::Error as StdError;
use rocket::{Request, response};
use rocket::http::Status;
use rocket::response::Responder;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotFound,
    InternalServerError,
    Implement,
    IOError,
    SettingsNotFound,
    Debug(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotFound => f.write_str("NotFound"),
            Error::InternalServerError => f.write_str("InternalServerError"),
            Error::Implement => f.write_str("Implement"),
            Error::Debug(_) => f.write_str("Debug/custom error"),
            Error::SettingsNotFound => f.write_str("SettingsNotFound"),
            Error::IOError => f.write_str("IOError"),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match &*self {
            Error::NotFound => "Record not found",
            Error::InternalServerError => "Internal server error",
            Error::Implement => "To be implemented",
            Error::SettingsNotFound => "No settings configuration found",
            Error::Debug(message) => &message,
            Error::IOError => "Input/output error",
        }
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'static> {
        match self {
            Error::NotFound => Err(Status::NotFound),
            Error::Implement => Err(Status::InternalServerError),
            Error::SettingsNotFound => Err(Status::NotFound),
            _ => Err(Status::Locked),
        }
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(e: mongodb::error::Error) -> Self {

        let m = e.kind.as_ref();
        match &*m {
            mongodb::error::ErrorKind::ServerSelection {..} => Error::NotFound,
            _ => Error::InternalServerError,
        }
    }
}

impl From<mongodb::bson::ser::Error> for Error {
    fn from(e: mongodb::bson::ser::Error) -> Self {
        match e {
            mongodb::bson::ser::Error::Io(_) => Error::Implement,
            mongodb::bson::ser::Error::InvalidDocumentKey(_) => Error::Implement,
            mongodb::bson::ser::Error::InvalidCString(_) => Error::Implement,
            mongodb::bson::ser::Error::SerializationError {..} => Error::Implement,
            mongodb::bson::ser::Error::UnsignedIntegerExceededRange(_) => Error::Implement,
            _ => Error::Implement
        }
    }
}

impl From<mongodb::bson::de::Error> for Error {
    fn from (e: mongodb::bson::de::Error) -> Self {
        match e {
            mongodb::bson::de::Error::Io(_) => Error::Implement,
            mongodb::bson::de::Error::InvalidUtf8String(_) => Error::Implement,
            mongodb::bson::de::Error::UnrecognizedDocumentElementType { ..} => Error::Implement,
            mongodb::bson::de::Error::EndOfStream => Error::Implement,
            mongodb::bson::de::Error::DeserializationError { .. } => Error::Implement,
            _ => Error::InternalServerError,
        }
    }
}

impl From<mongodb::bson::document::ValueAccessError> for Error {
    fn from (e: mongodb::bson::document::ValueAccessError) -> Self {
        match e {
            mongodb::bson::document::ValueAccessError::NotPresent => Error::Implement,
            mongodb::bson::document::ValueAccessError::UnexpectedType => Error::Implement,
            _ => Error::InternalServerError,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            _ => Error::IOError,
        }
    }
}