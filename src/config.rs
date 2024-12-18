use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct Tables { // all config tables
    pub filters: FilterFields,
    pub login: LoginFields,
    pub info: InfoFields,
}

#[derive(Serialize, Deserialize)]
pub struct InfoFields {
    pub room_id: String,   
}

#[derive(Serialize, Deserialize)]
pub struct FilterFields {
    pub message_filters: Vec<String>,
    pub per_message_tag_limit: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginFields {
    pub username: String,
    pub password: String,
}

impl Default for Tables {
    fn default() -> Self {
        Tables {
            filters: FilterFields {
                message_filters: vec!["".to_string()],
                per_message_tag_limit: Some(4),
            },
            login: LoginFields {
                username: String::from("@example-change-me:matrix.org"),
                password: String::from("PASSWORD-HERE"),
            },
            info: InfoFields {
                room_id: String::from("example-room-id:matrix.org"),
            }
        }
    }
}

pub fn write_config_defaults() -> std::io::Result<File> {
    let config = Tables::default();
    let toml = toml::to_string(&config).unwrap();
    
    let mut file = File::create("config.toml")?;
    file.write_all(toml.as_bytes())?;
    Ok(file)
}