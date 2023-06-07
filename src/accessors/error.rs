#[derive(Debug)]
pub enum Error {
    TypeIdGetterError(Box<dyn std::error::Error>),
    NameGetterError(Box<dyn std::error::Error>),
    MarketGroupGetterError(Box<dyn std::error::Error>),
    GroupGetterError(Box<dyn std::error::Error>),
    CategoryGetterError(Box<dyn std::error::Error>),
    JsonGetterError(Box<dyn std::error::Error>),
    JsonSetterError(Box<dyn std::error::Error>),
    ItemGetterError(Box<dyn std::error::Error>),
    ItemSetterError(Box<dyn std::error::Error>),
    CharacterGetterError(Box<dyn std::error::Error>),
    CharacterSetterError(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TypeIdGetterError(e) => {
                write!(f, "TypeIdGetterError: {}", e)
            },
            Self::NameGetterError(e) => {
                write!(f, "NameGetterError: {}", e)
            },
            Self::MarketGroupGetterError(e) => {
                write!(f, "MarketGroupGetterError: {}", e)
            },
            Self::GroupGetterError(e) => {
                write!(f, "GroupGetterError: {}", e)
            },
            Self::CategoryGetterError(e) => {
                write!(f, "CategoryGetterError: {}", e)
            },
            Self::JsonGetterError(e) => {
                write!(f, "JsonGetterError: {}", e)
            },
            Self::JsonSetterError(e) => {
                write!(f, "JsonSetterError: {}", e)
            },
            Self::ItemGetterError(e) => {
                write!(f, "ItemGetterError: {}", e)
            },
            Self::ItemSetterError(e) => {
                write!(f, "ItemSetterError: {}", e)
            },
            Self::CharacterGetterError(e) => {
                write!(f, "CharacterGetterError: {}", e)
            },
            Self::CharacterSetterError(e) => {
                write!(f, "CharacterSetterError: {}", e)
            },
        }
    }
}

impl std::error::Error for Error {}

impl Into<tonic::Status> for Error {
    fn into(self) -> tonic::Status {
        match self {
            Self::TypeIdGetterError(e) => tonic::Status::internal(e.to_string()),
            Self::NameGetterError(e) => tonic::Status::internal(e.to_string()),
            Self::MarketGroupGetterError(e) => tonic::Status::internal(e.to_string()),
            Self::GroupGetterError(e) => tonic::Status::internal(e.to_string()),
            Self::CategoryGetterError(e) => tonic::Status::internal(e.to_string()),
            Self::JsonGetterError(e) => tonic::Status::internal(e.to_string()),
            Self::JsonSetterError(e) => tonic::Status::internal(e.to_string()),
            Self::ItemGetterError(e) => tonic::Status::internal(e.to_string()),
            Self::ItemSetterError(e) => tonic::Status::internal(e.to_string()),
            Self::CharacterGetterError(e) => tonic::Status::internal(e.to_string()),
            Self::CharacterSetterError(e) => tonic::Status::internal(e.to_string()),
        }
    }
}
