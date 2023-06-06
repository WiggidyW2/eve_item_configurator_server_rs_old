use crate::{
    typedef::TypeId,
    item_configurator_proto as pb,
};

use std::collections::HashMap;

// map of TypeId to a map of arbitrary key to Json index
#[derive(Debug, Clone, Default)]
pub struct Items(pub HashMap<TypeId, HashMap<String, u32>>);

impl Into<Vec<pb::Item>> for Items {
    fn into(self) -> Vec<pb::Item> {
        self.0
            .into_iter()
            .map(|(type_id, json_idx)| pb::Item {
                    type_id: type_id,
                    enabled: true,
                    json_idx: json_idx,
            })
            .collect()
    }
}
