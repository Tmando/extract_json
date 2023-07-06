use serde_json::{ Value };
use extendr_api::prelude::*;

fn extract_json_value(value: &Value) -> Robj {
    if value.is_object() {
        let cur_entry = value.as_object().unwrap();
        let keys = cur_entry.keys();
        let mut map: std::collections::HashMap<&str, Robj> = std::collections::HashMap::new();
        for cur_key in keys {
            let entry = cur_entry[cur_key].clone();
            map.insert(cur_key, Robj::from(extract_json_value(&entry)));
        }
        let list = List::from_hashmap(map).unwrap();
        return Robj::from(list);
    }
    if value.is_array() {
        let cur_entry = value.as_array().unwrap();
        let mut key_map: Vec<Robj> = Vec::new();
        for i in cur_entry.iter() {
            let entry = i;
            key_map.push(Robj::from(extract_json_value(&entry)));
        }
        return Robj::from(key_map);
    }
    if value.is_boolean() {
        let cur_entry = value.as_bool().unwrap();
        return Robj::from(cur_entry);
    }
    if value.is_i64() {
        let cur_entry = value.as_i64().unwrap();
        return Robj::from(cur_entry);
    }

    if value.is_f64() {
        let cur_entry = value.as_f64().unwrap();
        return Robj::from(cur_entry);
    }

    if value.is_string() {
        let cur_entry = value.as_str().unwrap();
        return Robj::from(cur_entry);
    }

    if value.is_null() {
        let cur_entry = value.as_null().unwrap();
        return Robj::from(cur_entry);
    }
    return Robj::from("");
}

#[extendr]
fn from_json(input_str: String) -> Robj {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&input_str).unwrap();
    return extract_json_value(&v);
}

extendr_module! {
    mod extract_json;
    fn from_json;
 }
