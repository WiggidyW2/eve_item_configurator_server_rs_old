pub enum Error {
    ProtoFieldError(ProtoFieldError),
}

impl From<ProtoFieldError> for Error {
    fn from(e: ProtoFieldError) -> Self {
        Self::ProtoFieldError(e)
    }
}

pub enum ProtoFieldError {
    InvalidQuery{ name: &'static str, value: i32 },
}
