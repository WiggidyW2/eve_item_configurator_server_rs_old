mod validator;

mod service;

mod pb;

mod typedef;
pub use typedef::TypeId;

mod data;
pub use data::DivisionNames;

mod error;
pub use error::Error;

mod accessors;
pub use accessors::{
    Accessor,
    TypeIdGetter,
    NameGetter,
    MarketGroupGetter,
    GroupGetter,
    CategoryGetter,
    JsonGetter,
    JsonSetter,
    ItemGetter,
    ItemSetter,
    CharacterGetter,
    CharacterSetter,
};

pub async fn serve(
    accessor: impl Accessor,
    client_id: String,
    address: std::net::SocketAddr,
    server: &mut tonic::transport::Server,
) -> Result<(), Error> {
    server
        .add_service(pb::server::ItemConfiguratorServer::new(
            service::Service {
                accessor: accessors::AccessorWrapper(accessor),
                validator: validator::Validator::new(client_id).await?,
            }
        ))
        .serve(address)
        .await?;
    Ok(())
}
