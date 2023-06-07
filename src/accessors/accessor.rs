use std::collections::HashMap;

use crate::{
    typedef::TypeId,
    data::DivisionNames,
    accessors::accessors::*,
    error::Error,
};

// use super::*;

pub trait Accessor:
    TypeIdGetter
    + NameGetter
    + MarketGroupGetter
    + GroupGetter
    + CategoryGetter
    + JsonGetter
    + JsonSetter
    + ItemGetter
    + ItemSetter
    + CharacterGetter
    + CharacterSetter
    + Send
    + Sync
    + 'static
{}

pub struct AccessorWrapper<A>(pub A);

impl<A: Accessor> AccessorWrapper<A> {
    pub async fn get_type_ids(&self) -> Result<Vec<TypeId>, Error> {
        self.0
            .get_type_ids()
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::TypeIdGetterError(Box::new(e))
            ))
    }
    pub async fn get_names(
        &self,
        language: &str,
    ) -> Result<Vec<String>, Error> {
        self.0
            .get_names(language)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::NameGetterError(Box::new(e))
            ))
    }
    pub async fn get_market_groups(
        &self,
        language: &str,
    ) -> Result<DivisionNames, Error> {
        self.0
            .get_market_groups(language)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::MarketGroupGetterError(Box::new(e))
            ))
    }
    pub async fn get_groups(
        &self,
        language: &str,
    ) -> Result<DivisionNames, Error> {
        self.0
            .get_groups(language)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::GroupGetterError(Box::new(e))
            ))
    }
    pub async fn get_categories(
        &self,
        language: &str,
    ) -> Result<DivisionNames, Error> {
        self.0
            .get_categories(language)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::CategoryGetterError(Box::new(e))
            ))
    }
    pub async fn get_json(
        &self,
        config_name: &str,
    ) -> Result<Vec<String>, Error> {
        self.0
            .get_json(config_name)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::JsonGetterError(Box::new(e))
            ))
    }
    pub async fn set_json(
        &self,
        config_name: &str,
        json: Vec<String>,
    ) -> Result<(), Error> {
        self.0
            .set_json(config_name, json)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::JsonSetterError(Box::new(e))
            ))
    }
    pub async fn get_items(
        &self,
        config_name: &str,
    ) -> Result<HashMap<TypeId, HashMap<String, u32>>, Error> {
        self.0
            .get_items(config_name)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::ItemGetterError(Box::new(e))
            ))
    }
    pub async fn set_items(
        &self,
        config_name: &str,
        items: HashMap<TypeId, HashMap<String, u32>>,
    ) -> Result<(), Error> {
        self.0
            .set_items(config_name, items)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::ItemSetterError(Box::new(e))
            ))
    }
    pub async fn get_characters(
        &self,
        config_name: &str,
    ) -> Result<Vec<String>, Error> {
        self.0
            .get_characters(config_name)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::CharacterGetterError(Box::new(e))
            ))
    }
    pub async fn set_characters(
        &self,
        config_name: &str,
        characters: Vec<String>
    ) -> Result<(), Error> {
        self.0
            .set_characters(config_name, characters)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::CharacterSetterError(Box::new(e))
            ))
    }
}

