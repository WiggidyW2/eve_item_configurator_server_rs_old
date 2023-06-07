use super::{
    service::Service,
    error::JsonError,
};

use crate::{
    accessors::Accessor,
    error::Error,
    pb,
};

use std::collections::{HashMap, HashSet};

use serde_json::Value as JsonValue;

impl<A: Accessor> Service<A> {
    fn update_items_authorized_abort(
        &self,
        new_token: String,
    ) -> pb::UpdateRep {
        pb::UpdateRep {
            refresh_token: new_token,
            authorized: true,
        }
    }
    
    pub fn update_items_unauthorized(
        &self,
        new_token: String,
    ) -> pb::UpdateRep {
        pb::UpdateRep {
            refresh_token: new_token,
            authorized: false,
        }
    }

    pub async fn update_items_authorized(
        &self,
        req: pb::UpdateReq,
        new_token: String,
    ) -> Result<pb::UpdateRep, Error> {
        let new_json_strings = req.json;
        let new_items = req.items;

        // Abort early if there are no new JSON strings or Items
        if new_json_strings.len() == 0 && new_items.len() == 0 {
            return Ok(self.update_items_authorized_abort(new_token));
        }

        // Retrieve the existing JSON strings and Items
        let json_strings_fut = match new_json_strings.len() {
            0 => None,
            _ => Some(self.accessor.get_json(&req.name)),
        };
        let mut items = match new_items.len() {
            0 => HashMap::new(),
            _ => self.accessor.get_items(&req.name).await?,
        };
        let mut json_strings = match json_strings_fut {
            Some(fut) => fut.await?,
            None => Vec::new(),
        };

        // Track the indexes of the JSON strings that are removed to later
        // remove dangling ones.
        let mut removed_indexes = HashSet::with_capacity(json_strings.len());

        // Convert the JSON strings to JSON values
        let mut json_values = json_strings_to_values(&json_strings)
            .map_err(|e| JsonError::Old(e))?; // Storage JSON
        let new_json_values = json_strings_to_values(&new_json_strings)
            .map_err(|e| JsonError::New(e))?; // New JSON from request

        // We will be mapping the new Item JSON indexes to different indexes
        // for the purposes of deduplication.
        let mut json_indexes = Vec::with_capacity(new_json_strings.len());

        // Check if the new JSON value already exists. If so, point its index
        // to the existing index. Otherwise, append the String and point the
        // index to the last element.
        'outer: for new_json_value in new_json_values {
            let mut i = 0;
            for json_value in &json_values {
                if &new_json_value == json_value {
                    json_indexes.push(i);
                    continue 'outer;
                }
                i += 1;
            }
            // Index of last element
            json_indexes.push(i + 1);
            // Reserialize to be consistent and possibly smaller.
            json_strings.push(serde_json::to_string(&new_json_value).unwrap());
            // Push the new JSON value onto the existing JSON values,
            // in case the request contains duplicate new JSON values.
            json_values.push(new_json_value);
        }

        // Update the existing items with the new JSON keys / indexes, adding
        // new items as necessary, and removing items that are disabled.
        for new_item in new_items {

            // If enabled, update the existing item.
            if new_item.enabled {
                // Get the existing item or create a new one.
                let item = items.entry(new_item.type_id).or_insert_with(|| {
                    HashMap::with_capacity(new_item.json_idx.len())
                });
                for (key, json_index) in new_item.json_idx {
                    let new_index = json_indexes[json_index as usize];
                    let removed_index = match &json_values[new_index] {
                        // If the JSON value is null, remove the entry.
                        JsonValue::Null => item.remove(&key),
                        // Otherwise, insert the new index.
                        _ => item.insert(key, new_index.try_into().unwrap()),
                    };
                    // Track the removed index to avoid dangling JSON strings.
                    if let Some(removed_index) = removed_index {
                        removed_indexes.insert(removed_index);
                    }
                }

            // If disabled, remove the existing item.
            } else {
                let removed_item = items.remove(&new_item.type_id);
                // Track the removed indexes to avoid dangling JSON strings.
                if let Some(removed_item) = removed_item {
                    for (_, removed_index) in removed_item {
                        removed_indexes.insert(removed_index);
                    }
                }
            }
        }

        // Remove dangling JSON strings
        remove_dangling_json(removed_indexes, &mut items, &mut json_strings);

        // Insert the new JSON strings and items (unless there are none)
        let insert_json_fut = match json_strings.len() {
            0 => None,
            _ => Some(self.accessor.set_json(&req.name, json_strings)),
        };
        if items.len() > 0 {
            self.accessor.set_items(&req.name, items).await?;
        }
        if let Some(fut) = insert_json_fut {
            fut.await?;
        }
        
        // Return the response
        Ok(pb::UpdateRep {
            refresh_token: new_token,
            authorized: true,
        })
    }
}

fn json_strings_to_values(
    json_strings: &[String],
) -> Result<Vec<JsonValue>, serde_json::Error> {
    let mut json_values = Vec::with_capacity(json_strings.len());
    for json_string in json_strings {
        let json_value = serde_json::from_str(json_string)?;
        json_values.push(json_value);
    }
    Ok(json_values)
}

fn remove_dangling_json(
    mut removed_indexes: HashSet<u32>, // contains at least every un-used index
    items: &mut HashMap<u32, HashMap<String, u32>>,
    json_strings: &mut Vec<String>,
) {
    if removed_indexes.len() == 0 { return; }

    // Remove the indexes from removed_indexes that are still being used
    for indexes in items.values() {
        for index in indexes.values() {
            removed_indexes.remove(index);
        }
    }

    if removed_indexes.len() == 0 { return; }

    // Remove the unused JSON strings
    let mut i = 0;
    json_strings.retain(|_| {
        let retain = !removed_indexes.contains(&i);
        i += 1;
        retain
    });

    // Convert the removed_indexes to a sorted vector
    let removed_indexes = to_sorted_vec(removed_indexes);
    
    // Update the Items' JSON indexes to account for the shifts
    let max_shift = removed_indexes.len() as u32;
    let first_index = removed_indexes[0];
    let last_index = removed_indexes[removed_indexes.len() - 1];
    let mid_slice = &removed_indexes[1..&removed_indexes.len() - 1];
    for indexes in items.values_mut() {
        for index in indexes.values_mut() {
            // No shifting required
            if *index < first_index {
                continue;
            // No searching required
            } else if *index > last_index {
                *index -= max_shift;
            // Binary search for the partition point
            } else {
                let shift = 1 + mid_slice
                    .partition_point(|i| *index > *i)
                    as u32;
                *index -= shift;
            }
        }
    }
}

fn to_sorted_vec(set: HashSet<u32>) -> Vec<u32> {
    let mut vec = Vec::with_capacity(set.len());
    for item in set {
        vec.push(item);
    }
    vec.sort_unstable();
    vec
}

#[cfg(test)]
mod tests {
    use super::{
        remove_dangling_json,
    };

    use std::collections::{HashMap, HashSet};

    #[test]
    fn remove_dangling_json_test() {
        let mut removed_indexes = HashSet::with_capacity(3);
        removed_indexes.insert(1);
        removed_indexes.insert(3);
        removed_indexes.insert(7);

        let mut items = HashMap::new();
        items.insert(0, {
            let mut item = HashMap::with_capacity(3);
            item.insert("0".to_string(), 0);
            item.insert("2".to_string(), 2);
            item.insert("4".to_string(), 4);
            item
        });
        items.insert(1, {
            let mut item = HashMap::with_capacity(6);
            item.insert("0".to_string(), 0);
            item.insert("2".to_string(), 2);
            item.insert("2 again".to_string(), 2);
            item.insert("5".to_string(), 5);
            item.insert("6".to_string(), 6);
            item.insert("6 again".to_string(), 6);
            item
        });
        items.insert(2, {
            let mut item = HashMap::with_capacity(3);
            item.insert("2".to_string(), 2);
            item.insert("5".to_string(), 5);
            item.insert("6".to_string(), 6);
            item
        });
        
        let mut json_strings = vec![
            "0".to_string(),
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
        ];

        remove_dangling_json(removed_indexes, &mut items, &mut json_strings);

        assert_eq!(json_strings, vec![
            "0".to_string(),
            "2".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
        ]);

        assert_eq!(items[&0], {
            let mut item = HashMap::with_capacity(3);
            item.insert("0".to_string(), 0);
            item.insert("2".to_string(), 1);
            item.insert("4".to_string(), 2);
            item
        });

        assert_eq!(items[&1], {
            let mut item = HashMap::with_capacity(6);
            item.insert("0".to_string(), 0);
            item.insert("2".to_string(), 1);
            item.insert("2 again".to_string(), 1);
            item.insert("5".to_string(), 3);
            item.insert("6".to_string(), 4);
            item.insert("6 again".to_string(), 4);
            item
        });
        
        assert_eq!(items[&2], {
            let mut item = HashMap::with_capacity(3);
            item.insert("2".to_string(), 1);
            item.insert("5".to_string(), 3);
            item.insert("6".to_string(), 4);
            item
        });
    }
}
