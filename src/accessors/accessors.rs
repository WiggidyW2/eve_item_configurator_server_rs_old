use std::collections::HashMap;

use crate::{
    typedef::TypeId,
    data::{
        stable::DivisionNames,
    },
};

use tonic::async_trait;

// Static Data

#[async_trait]
pub trait TypeIdGetter: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;
    async fn get_type_ids(&self) -> Result<Vec<TypeId>, Self::Error>;
}

#[async_trait]
pub trait NameGetter: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;
    async fn get_names(
        &self,
        language: &str,
    ) -> Result<Vec<String>, Self::Error>;
}

#[async_trait]
pub trait MarketGroupGetter: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;
    async fn get_market_groups(
        &self,
        language: &str,
    ) -> Result<DivisionNames, Self::Error>;
}

#[async_trait]
pub trait GroupGetter: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;
    async fn get_groups(
        &self,
        language: &str,
    ) -> Result<DivisionNames, Self::Error>;
}

#[async_trait]
pub trait CategoryGetter: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;
    async fn get_categories(
        &self,
        language: &str,
    ) -> Result<DivisionNames, Self::Error>;
}

// Dynamic Data

#[async_trait]
pub trait JsonGetter: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;
    async fn get_json(
        &self,
        config_name: &str,
    ) -> Result<Vec<String>, Self::Error>;
}

#[async_trait]
pub trait JsonSetter: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;
    async fn set_json(
        &self,
        config_name: &str,
        json: Vec<String>,
    ) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait ItemGetter: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;
    async fn get_items(
        &self,
        config_name: &str,
    ) -> Result<HashMap<TypeId, HashMap<String, u32>>, Self::Error>;
}

#[async_trait]
pub trait ItemSetter: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;
    async fn set_items(
        &self,
        config_name: &str,
        items: HashMap<TypeId, HashMap<String, u32>>,
    ) -> Result<(), Self::Error>;
}

// Character List that is used for authentication

#[async_trait]
pub trait CharacterGetter: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;
    async fn get_characters(
        &self,
        config_name: &str,
    ) -> Result<Vec<String>, Self::Error>;
}

#[async_trait]
pub trait CharacterSetter: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;
    async fn set_characters(
        &self,
        config_name: &str,
        characters: Vec<String>
    ) -> Result<(), Self::Error>;
}
