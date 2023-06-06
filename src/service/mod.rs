use std::collections::HashMap;
use std::cmp::max;

use crate::{
    typedef::TypeId,
    item_configurator_proto as pb,
    accessors::Accessor,
    // error::Error,
    data::{
        stable::DivisionNames,
    },
};

mod error;
pub use error::Error;

mod procedure;

pub struct Service<A> {
    accessor: A,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IterateQuery {
    None,
    Items,
    Types,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReturnQuery {
    None,
    InTypes,
    Configured,
    NotConfigured,
    NotInItems,
    InItems,
}

use procedure::{Procedure, Iterate, Keep};

impl<A: Accessor> Service<A> {
    pub async fn list(&self, req: pb::ListReq) -> Result<pb::ListRep, crate::error::Error> {
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
        let mut names = match names_fut {
            Some(fut) => fut.await?,
            None => Vec::new(),
        };
        let mut market_groups = match market_groups_fut {
            Some(fut) => fut.await?,
            None => DivisionNames::with_names(vec!["".to_string()]),
        };
        let mut groups = match groups_fut {
            Some(fut) => fut.await?,
            None => DivisionNames::with_names(vec!["".to_string()]),
        };
        let mut categories = match categories_fut {
            Some(fut) => fut.await?,
            None => DivisionNames::with_names(vec!["".to_string()]),
        };
        let json = match json_fut {
            Some(fut) => fut.await?,
            None => Vec::new(),
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
                    let (json_idx, enabled) = match items.remove(&type_id) {
                        Some(json_idx) => (json_idx, true),
                        None => (HashMap::new(), false),
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
        })

        // unimplemented!()
        
        // if procedure.iterate == Iterate::Items {
        //     for (type_id, json_idx) in items {

        //         match procedure.keep {
        //             Keep::Configured => if json_idx.len() == 0 {
        //                 continue;
        //             },
        //             Keep::NotConfigured => if json_idx.len() != 0 {
        //                 continue;
        //             },
        //             _ => (),
        //         };

        //         rep_items.push(pb::ListItem {
        //             type_id: type_id,
        //             enabled: true,
        //             json_idx: json_idx,
        //             name: "".to_string(),
        //             market_group_idx: 0,
        //             group_idx: 0,
        //             category_idx: 0,
        //         });
        //     }
        // }

        // else if procedure.iterate == Iterate::Types {
        //     let get_item = match (req.include_json, procedure.keep) {
        //         (false, Keep::InTypes) => false,
        //         _ => true,
        //     };
        //     // We reverse so that we can pop names / division names
        //     for type_id in types.into_iter().rev() {

        //         // None means item is disabled and not configured
        //         // Some(len > 0) means item is enabled and configured
        //         // Some(len = 0) means item is enabled and not configured
        //         let (json_idx, enabled) = match get_item {
        //             true => match items.remove(&type_id) {
        //                 Some(json_idx) => (json_idx, true),
        //                 None => (HashMap::new(), false),
        //             },
        //             false => (HashMap::new(), true),
        //         };

        //         match procedure.keep {
        //             Keep::NotInItems => if enabled {
        //                 continue;
        //             },
        //             Keep::InItems => if !enabled {
        //                 continue;
        //             },
        //             Keep::Configured => if json_idx.len() == 0 {
        //                 continue;
        //             },
        //             Keep::NotConfigured => if json_idx.len() != 0 {
        //                 continue;
        //             },
        //             _ => (),
        //         };

        //         rep_items.push(pb::ListItem {
        //             type_id: type_id,
        //             enabled: enabled,
        //             json_idx: json_idx,
        //             name: match req.include_name {
        //                 true => names.pop().unwrap(),
        //                 false => "".to_string(),
        //             },
        //             market_group_idx: match req.include_market_group {
        //                 true => market_groups.indexes.pop().unwrap(),
        //                 false => 0,
        //             },
        //             group_idx: match req.include_group {
        //                 true => groups.indexes.pop().unwrap(),
        //                 false => 0,
        //             },
        //             category_idx: match req.include_category {
        //                 true => categories.indexes.pop().unwrap(),
        //                 false => 0,
        //             },
        //         });
        //     }
        // }
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
