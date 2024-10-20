use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub limits: LimitsConfig
}

#[derive(Serialize, Deserialize)]
pub struct LimitsConfig {
    pub per_message_tag_limit: Option<i32>
}


// defaults
#[derive(Serialize, Deserialize, Debug)]
struct LimitsDefaults {
    limits: LimitsFields,
}

#[derive(Serialize, Deserialize, Debug)]
struct LimitsFields {
    per_message_tag_limit: Option<i32>,
}

impl Default for LimitsFields {
    fn default() -> Self {
        LimitsFields {
            per_message_tag_limit: Some(4),
        }
    }
}

impl Default for LimitsDefaults {
    fn default() -> Self {
        LimitsDefaults {
            limits: LimitsFields::default(),
        }
    }
}

pub fn write_config_defaults() -> std::io::Result<File> {
    let config = LimitsDefaults::default();
    let toml = toml::to_string(&config).unwrap();
    
    let mut file = File::create("config.toml")?;
    file.write_all(toml.as_bytes())?;
    Ok(file)
}