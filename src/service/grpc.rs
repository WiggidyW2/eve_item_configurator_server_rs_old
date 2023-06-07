use super::{
    service::Service,
};

use crate::{
    accessors::Accessor,
    pb,
};

use tonic::async_trait;

#[async_trait]
impl<A: Accessor> pb::server::ItemConfigurator for Service<A> {
    async fn update(
        &self,
        request: tonic::Request<pb::UpdateReq>,
    ) -> Result<tonic::Response<pb::UpdateRep>, tonic::Status> {
        match self.update_items(request.into_inner()).await {
            Ok(rep) => Ok(tonic::Response::new(rep)),
            Err(e) => Err(e.into()),
        }
    }
    async fn list(
        &self,
        request: tonic::Request<pb::ListReq>,
    ) -> Result<tonic::Response<pb::ListRep>, tonic::Status> {
        match self.list_items(request.into_inner()).await {
            Ok(rep) => Ok(tonic::Response::new(rep)),
            Err(e) => Err(e.into()),
        }
    }
    async fn list_characters(
        &self,
        request: tonic::Request<pb::ListCharactersReq>,
    ) -> Result<tonic::Response<pb::ListCharactersRep>, tonic::Status> {
        match self.list_characters(request.into_inner()).await {
            Ok(rep) => Ok(tonic::Response::new(rep)),
            Err(e) => Err(e.into()),
        }
    }
    async fn add_characters(
        &self,
        request: tonic::Request<pb::AddCharactersReq>,
    ) -> Result<tonic::Response<pb::AddCharactersRep>, tonic::Status> {
        match self.add_characters(request.into_inner()).await {
            Ok(rep) => Ok(tonic::Response::new(rep)),
            Err(e) => Err(e.into()),
        }
    }
    async fn del_characters(
        &self,
        request: tonic::Request<pb::DelCharactersReq>,
    ) -> Result<tonic::Response<pb::DelCharactersRep>, tonic::Status> {
        match self.del_characters(request.into_inner()).await {
            Ok(rep) => Ok(tonic::Response::new(rep)),
            Err(e) => Err(e.into()),
        }
    }
}
