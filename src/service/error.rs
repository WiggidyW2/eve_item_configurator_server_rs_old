#[derive(Debug)]
pub enum ProtoFieldError {
    InvalidQuery{ name: &'static str, value: i32 },
}

impl std::fmt::Display for ProtoFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtoFieldError::InvalidQuery{ name, value } => {
                write!(f, "Field had invalid variant for Query: {}={}", name, value)
            },
        }
    }
}

impl std::error::Error for ProtoFieldError {}

impl Into<tonic::Status> for ProtoFieldError {
    fn into(self) -> tonic::Status {
        tonic::Status::invalid_argument(self.to_string())
    }
}

#[derive(Debug)]
pub enum JsonError {
    Old(serde_json::Error),
    New(serde_json::Error),
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Old(e) => {
                write!(f, "Error parsing stored JSON: {}", e)
            },
            Self::New(e) => {
                write!(f, "Error parsing new JSON: {}", e)
            },
        }
    }
}

impl std::error::Error for JsonError {}

impl Into<tonic::Status> for JsonError {
    fn into(self) -> tonic::Status {
        match self {
            Self::Old(e) => tonic::Status::internal(e.to_string()),
            Self::New(e) => tonic::Status::invalid_argument(e.to_string()),
        }
    }
}
