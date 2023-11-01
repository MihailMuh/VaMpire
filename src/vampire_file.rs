use configparser::ini::Ini;

use crate::utils::{get, parse};

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
            "   username: {}\n   password: ***\n   cpu: {}\n   ram: {}GB\n   disk: {}GB",
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
            cpu: parse(content, "cpu"),
            ram: parse(content, "ram"),
            disk: parse(content, "disk"),
        }
    }
}
