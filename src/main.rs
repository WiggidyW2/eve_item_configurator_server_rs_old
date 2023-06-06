mod typedef;

mod accessors;

mod item_configurator_proto;

mod data_types;

mod data;

mod service;

mod error;
pub use error::Error;

fn main() {
    let mut json_obj_test_1 = serde_json::Value::Object(serde_json::Map::new());
    let mut json_obj_test_2 = serde_json::Value::Object(serde_json::Map::new());

    let mut inner_1 = json_obj_test_1.as_object_mut().unwrap();
    let mut inner_2 = json_obj_test_2.as_object_mut().unwrap();

    inner_1.insert("test1".to_string(), serde_json::Value::String("A".to_string()));
    inner_1.insert("test2".to_string(), serde_json::Value::String("B".to_string()));
    inner_1.insert("test3".to_string(), serde_json::Value::String("C".to_string()));

    inner_2.insert("test2".to_string(), serde_json::Value::String("B".to_string()));
    inner_2.insert("test3".to_string(), serde_json::Value::String("C".to_string()));
    inner_2.insert("test1".to_string(), serde_json::Value::String("A".to_string()));

    println!("{}", json_obj_test_1 == json_obj_test_2);


    println!("Hello, world!");
}
