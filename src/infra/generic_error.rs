use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct GenericError {
    message: String,
}

impl Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl GenericError {
    fn from_error<T: Error>(inner: T) -> Self {
        Self::from_string(inner.to_string())
    }

    pub fn from_string(message: String) -> Self {
        Self { message }
    }
}

impl<T: Error> From<T> for GenericError {
    fn from(inner: T) -> Self {
        Self::from_error(inner)
    }
}

pub trait ToGenericError {
    fn to_generic_error(self) -> GenericError;
}

impl ToGenericError for String {
    fn to_generic_error(self) -> GenericError {
        GenericError::from_string(self)
    }
}

impl ToGenericError for &str {
    fn to_generic_error(self) -> GenericError {
        self.to_string().to_generic_error()
    }
}
