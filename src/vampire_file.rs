use std::collections::HashMap;
use configparser::ini::Ini;

pub struct VaMpireFile {
    username: String,
    password: String,

    cpu: usize,
    ram: usize,
    disk: usize,
}

impl VaMpireFile {
    pub fn to_string(&self) -> String {
        format!(
            "username: {}\ncpu: {}\nram: {}GB\ndisk: {}GB",
            &self.username, &self.cpu, &self.ram, &self.disk
        )
    }

    pub fn get(file_path: &str) -> VaMpireFile {
        let raw_content = Ini::new().load(file_path)
            .unwrap_or_else(|error| panic!("{}", error));

        let content = raw_content.get("default")
            .expect("Vampirefile is empty!");

        VaMpireFile {
            username: get(content, "username"),
            password: get(content, "password"),
            cpu: get(content, "cpu").parse().unwrap_or_default(),
            ram: get(content, "ram").parse().unwrap_or_default(),
            disk: get(content, "disk").parse().unwrap_or_default(),
        }
    }
}

fn get(hashmap: &HashMap<String, Option<String>>, key: &str) -> String {
    if !hashmap.contains_key(key) {
        "".to_string()
    } else {
        hashmap[key].clone().unwrap_or_default()
    }
}
