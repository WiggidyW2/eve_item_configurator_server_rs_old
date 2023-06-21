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
    Accessor, CategoryGetter, CharacterGetter, CharacterSetter, GroupGetter, ItemGetter,
    ItemSetter, JsonGetter, JsonSetter, MarketGroupGetter, NameGetter, TypeIdGetter,
};

// re-export for convenience to accessor implementors
pub use tonic::async_trait;

use tonic_web;

pub async fn serve(
    accessor: impl Accessor,
    client_id: String,
    address: std::net::SocketAddr,
    server: &mut tonic::transport::Server,
    http1: bool,
) -> Result<(), Error> {
    let service = pb::server::ItemConfiguratorServer::new(service::Service {
        accessor: accessors::AccessorWrapper(accessor),
        validator: validator::Validator::new(client_id).await?,
    });
    let router = match http1 {
        true => server.add_service(tonic_web::enable(service)),
        false => server.add_service(service),
    };
    router.serve(address).await?;
    Ok(())
}
