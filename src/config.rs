use ron::{from_str, to_string};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::{create_dir_all, read_to_string},
    path::PathBuf,
};
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub name: String,
    pub email: String,
}

impl Account {
    pub fn new(name: String, email: String) -> Account {
        Account { name, email }
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

pub fn get_config() -> PathBuf {
    let mut path = dirs::config_dir().expect("Where tf is your config dir?");
    path.push("gitmanager");
    create_dir_all(&path).expect("Cannot create configuration directory");
    path.push("config.ron");
    path
}

pub fn get_accounts(path: &PathBuf) -> Vec<Account> {
    if path.exists() {
        let file = read_to_string(path).expect("unable to read accounts from config file.");
        from_str(&file).expect("unable to deserialize RON data")
    } else {
        Vec::new()
    }
}

pub fn update_acounts(accounts: Vec<Account>, path: &PathBuf) {
    let str = to_string(&accounts).expect("unable to serialize data");
    std::fs::write(path, str).expect("unable to write accounts to config file");
}
