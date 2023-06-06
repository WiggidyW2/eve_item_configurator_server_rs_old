use std::collections::HashMap;

use crate::{
    typedef::TypeId,
    data::{
        stable::DivisionNames,
    },
    accessors::accessors::*,
    error::Error,
};

// use super::*;

use tonic::async_trait;

#[async_trait]
pub trait Accessor: Send + Sync + 'static {
    async fn get_type_ids(&self) -> Result<Vec<TypeId>, Error>;
    async fn get_names(
        &self,
        language: &str,
    ) -> Result<Vec<String>, Error>;
    async fn get_market_groups(
        &self,
        language: &str,
    ) -> Result<DivisionNames, Error>;
    async fn get_groups(
        &self,
        language: &str,
    ) -> Result<DivisionNames, Error>;
    async fn get_categories(
        &self,
        language: &str,
    ) -> Result<DivisionNames, Error>;
    async fn get_json(
        &self,
        config_name: &str,
    ) -> Result<Vec<String>, Error>;
    async fn set_json(
        &self,
        config_name: &str,
        json: Vec<String>,
    ) -> Result<(), Error>;
    async fn get_items(
        &self,
        config_name: &str,
    ) -> Result<HashMap<TypeId, HashMap<String, u32>>, Error>;
    async fn set_items(
        &self,
        config_name: &str,
        items: HashMap<TypeId, HashMap<String, u32>>,
    ) -> Result<(), Error>;
    async fn get_characters(
        &self,
        config_name: &str,
    ) -> Result<Vec<String>, Error>;
    async fn set_characters(
        &self,
        config_name: &str,
        characters: Vec<String>
    ) -> Result<(), Error>;
}

pub struct AccessorImpl<
    A: TypeIdGetter,
    B: NameGetter,
    C: MarketGroupGetter,
    D: GroupGetter,
    E: CategoryGetter,
    F: JsonGetter,
    G: JsonSetter,
    H: ItemGetter,
    I: ItemSetter,
    J: CharacterGetter,
    K: CharacterSetter,
> {
    type_id_getter: A,
    name_getter: B,
    market_group_getter: C,
    group_getter: D,
    category_getter: E,
    json_getter: F,
    json_setter: G,
    item_getter: H,
    item_setter: I,
    character_getter: J,
    character_setter: K,
}

#[async_trait]
impl<
    A: TypeIdGetter,
    B: NameGetter,
    C: MarketGroupGetter,
    D: GroupGetter,
    E: CategoryGetter,
    F: JsonGetter,
    G: JsonSetter,
    H: ItemGetter,
    I: ItemSetter,
    J: CharacterGetter,
    K: CharacterSetter,
> Accessor for AccessorImpl<A, B, C, D, E, F, G, H, I, J, K> {
    async fn get_type_ids(&self) -> Result<Vec<TypeId>, Error> {
        self.type_id_getter
            .get_type_ids()
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::TypeIdGetterError(Box::new(e))
            ))
    }
    async fn get_names(
        &self,
        language: &str,
    ) -> Result<Vec<String>, Error> {
        self.name_getter
            .get_names(language)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::NameGetterError(Box::new(e))
            ))
    }
    async fn get_market_groups(
        &self,
        language: &str,
    ) -> Result<DivisionNames, Error> {
        self.market_group_getter
            .get_market_groups(language)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::MarketGroupGetterError(Box::new(e))
            ))
    }
    async fn get_groups(
        &self,
        language: &str,
    ) -> Result<DivisionNames, Error> {
        self.group_getter
            .get_groups(language)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::GroupGetterError(Box::new(e))
            ))
    }
    async fn get_categories(
        &self,
        language: &str,
    ) -> Result<DivisionNames, Error> {
        self.category_getter
            .get_categories(language)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::CategoryGetterError(Box::new(e))
            ))
    }
    async fn get_json(
        &self,
        config_name: &str,
    ) -> Result<Vec<String>, Error> {
        self.json_getter
            .get_json(config_name)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::JsonGetterError(Box::new(e))
            ))
    }
    async fn set_json(
        &self,
        config_name: &str,
        json: Vec<String>,
    ) -> Result<(), Error> {
        self.json_setter
            .set_json(config_name, json)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::JsonSetterError(Box::new(e))
            ))
    }
    async fn get_items(
        &self,
        config_name: &str,
    ) -> Result<HashMap<TypeId, HashMap<String, u32>>, Error> {
        self.item_getter
            .get_items(config_name)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::ItemGetterError(Box::new(e))
            ))
    }
    async fn set_items(
        &self,
        config_name: &str,
        items: HashMap<TypeId, HashMap<String, u32>>,
    ) -> Result<(), Error> {
        self.item_setter
            .set_items(config_name, items)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::ItemSetterError(Box::new(e))
            ))
    }
    async fn get_characters(
        &self,
        config_name: &str,
    ) -> Result<Vec<String>, Error> {
        self.character_getter
            .get_characters(config_name)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::CharacterGetterError(Box::new(e))
            ))
    }
    async fn set_characters(
        &self,
        config_name: &str,
        characters: Vec<String>
    ) -> Result<(), Error> {
        self.character_setter
            .set_characters(config_name, characters)
            .await
            .map_err(|e| Error::AccessorError(
                super::Error::CharacterSetterError(Box::new(e))
            ))
    }
}

