use super::{
    list_items_procedure::{Procedure, Iterate, Keep},
    service::Service,
};

use crate::{
    data::DivisionNames,
    accessors::Accessor,
    typedef::TypeId,
    error::Error,
    pb,
};

use std::collections::HashMap;
use std::cmp::max;

impl<A: Accessor> Service<A> {
    pub fn list_items_unauthorized(
        &self,
        new_token: String,
    ) -> pb::ListRep {
        pb::ListRep {
            items: Vec::new(),
            json: Vec::new(),
            market_groups: Vec::new(),
            groups: Vec::new(),
            categories: Vec::new(),
            refresh_token: new_token,
            authorized: false,
        }
    }

    pub async fn list_items_authorized(
        &self,
        req: pb::ListReq,
        new_token: String,
    ) -> Result<pb::ListRep, Error> {
        let procedure = Procedure::try_from(&req)?;

        let json_fut = match req.include_json {
            true => Some(self.accessor.get_json(&req.name)),
            false => None,
        };
        let items_fut = match procedure.get_items {
            true => Some(self.accessor.get_items(&req.name)),
            false => None,
        };
        let types_fut = match procedure.get_types {
            true => Some(self.accessor.get_type_ids()),
            false => None,
        };
        let names_fut = match req.include_name {
            true => Some(self.accessor.get_names(&req.language)),
            false => None,
        };
        let market_groups_fut = match req.include_market_group {
            true => Some(self.accessor.get_market_groups(&req.language)),
            false => None,
        };
        let groups_fut = match req.include_group {
            true => Some(self.accessor.get_groups(&req.language)),
            false => None,
        };
        let categories_fut = match req.include_category {
            true => Some(self.accessor.get_categories(&req.language)),
            false => None,
        };

        let types = match types_fut {
            Some(fut) => fut.await?,
            None => Vec::new(),
        };
        let mut items = match items_fut {
            Some(fut) => fut.await?,
            None => HashMap::new(),
        };
        let names = match names_fut {
            Some(fut) => fut.await?,
            None => Vec::new(),
        };
        let json = match json_fut {
            Some(fut) => fut.await?,
            None => Vec::new(),
        };

        // The default names vector includes a single empty string, which the
        // item indexes will point to in the case that division names are not
        // requested.
        let market_groups = match market_groups_fut {
            Some(fut) => fut.await?,
            None => DivisionNames::with_names(vec!["".to_string()]),
        };
        let groups = match groups_fut {
            Some(fut) => fut.await?,
            None => DivisionNames::with_names(vec!["".to_string()]),
        };
        let categories = match categories_fut {
            Some(fut) => fut.await?,
            None => DivisionNames::with_names(vec!["".to_string()]),
        };

        let mut list_items = Vec::with_capacity(max(items.len(), types.len()));
        
        match procedure.iterate {
            Iterate::None => set_list_items(
                &mut list_items,
                procedure.keep,
                market_groups.indexes,
                categories.indexes,
                groups.indexes,
                names,
                std::iter::empty(),
            ),
            Iterate::Items => set_list_items(
                &mut list_items,
                procedure.keep,
                market_groups.indexes,
                categories.indexes,
                groups.indexes,
                names,
                items.into_iter().map(|(type_id, json_idx)| {
                    (type_id, true, json_idx)
                }),
            ),
            Iterate::Types => set_list_items(
                &mut list_items,
                procedure.keep,
                market_groups.indexes,
                categories.indexes,
                groups.indexes,
                names,
                types.into_iter().rev().map(|type_id| {
                    let (json_idx, enabled) = match (
                        items.remove(&type_id),
                        req.include_json,
                     ) {
                        // There is an item and we don't need json.
                        (Some(_), false) => (HashMap::new(), true),
                        // There is an item and we need json.
                        (Some(json_idx), true) => (json_idx, true),
                        // There is no item.
                        (None, _) => (HashMap::new(), false),
                    };
                    (type_id, enabled, json_idx)
                }),
            ),
        };
        
        Ok(pb::ListRep {
            items: list_items,
            json: json,
            market_groups: market_groups.names,
            groups: groups.names,
            categories: categories.names,
            refresh_token: new_token,
            authorized: true,
        })
    }

}

fn set_list_items(
    items: &mut Vec<pb::ListItem>,
    keep: Keep,
    mut market_group_indexes: Vec<u32>,
    mut category_indexes: Vec<u32>,
    mut group_indexes: Vec<u32>,
    mut names: Vec<String>,
    iter: impl Iterator<Item = (
        TypeId, // type ID
        bool, // enabled
        HashMap<String, u32>, // json indexes
    )>,
) {
    for (type_id, enabled, json_idx) in iter {
        let market_group_index = market_group_indexes.pop();
        let category_index = category_indexes.pop();
        let group_index = group_indexes.pop();
        let name = names.pop();
        match keep {
            Keep::NotInItems => if enabled {
                continue;
            },
            Keep::InItems => if !enabled {
                continue;
            },
            Keep::Configured => if json_idx.len() == 0 {
                continue;
            },
            Keep::NotConfigured => if json_idx.len() != 0 {
                continue;
            },
            _ => (),
        }
        items.push(pb::ListItem {
            type_id: type_id,
            enabled: enabled,
            json_idx: json_idx,
            name: name.unwrap_or(String::new()),
            market_group_idx: market_group_index.unwrap_or(0),
            category_idx: category_index.unwrap_or(0),
            group_idx: group_index.unwrap_or(0),
        });
    }
}
