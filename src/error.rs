pub enum Error {
    AccessorError(crate::accessors::Error),
    ServiceError(crate::service::Error),
}

impl From<crate::accessors::Error> for Error {
    fn from(e: crate::accessors::Error) -> Self {
        Self::AccessorError(e)
    }
}

impl From<crate::service::Error> for Error {
    fn from(e: crate::service::Error) -> Self {
        Self::ServiceError(e)
    }
}
