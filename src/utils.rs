use std::collections::HashMap;

pub fn get(hashmap: &HashMap<String, Option<String>>, key: &str) -> String {
    if !hashmap.contains_key(key) {
        panic!("There's no value for {}", key)
    } else {
        let value = hashmap[key].clone().expect(&*format!("Incorrect value for {}", key));
        if value.is_empty() {
            panic!("There's no value for {}", key)
        }
        value
    }
}

pub fn parse(hashmap: &HashMap<String, Option<String>>, key: &str) -> usize {
    if !hashmap.contains_key(key) {
        panic!("There's no value for {}", key)
    } else {
        hashmap[key].clone().expect(&*format!("Incorrect value for {}", key))
            .parse().expect(&*format!("Incorrect value for {}", key))
    }
}
