use std::collections::HashMap;

use axum::http::HeaderMap;

pub fn headermap_to_hashmap(header: & HeaderMap) -> HashMap<String, String> {
    let mut hash_map =  HashMap::new();

    for (key, value) in header.iter() {
        hash_map.insert(key.to_string(), value.to_str().unwrap().to_string());
    }
    return hash_map;
}
