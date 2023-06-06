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
