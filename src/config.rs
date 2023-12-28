use serde::{Serialize, Deserialize, ser::SerializeMap};
use toml::{to_string_pretty, from_str};
use std::{fs::{read_to_string, create_dir_all}, path::PathBuf, fmt};

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    name: String,
    email: String
}

impl Account {
    pub fn new(name: String, email: String) -> Account {
        Account {
            name,
            email
        }
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

pub fn get_config() -> PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push("gitmanager");
    create_dir_all(&path).expect("Cannot create configuration directory");
    path.push("config.toml");
    path
}

pub fn get_accounts(path: &PathBuf) -> Vec<Account> {
    if path.exists() {
        let file = read_to_string(path).expect("unable to read accounts from config file.");
        from_str(&file).unwrap()
    } else {
        Vec::new()
    }
}

pub fn update_acounts(accounts: Vec<Account>, path: &PathBuf) {
    let str = to_string_pretty(&accounts).unwrap();
    std::fs::write(path, &str).expect("unable to write accounts to config file");
}
