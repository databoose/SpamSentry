use std::fs::OpenOptions;
use std::io::Read;
use toml::from_str;

mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = match OpenOptions::new().read(true).open("config.toml") {
       Ok(file) => file,
       Err(NotFound) => {
            config::write_config_defaults()?;
            OpenOptions::new().read(true).open("config.toml")?
        },
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("contents: {}", contents);
    let config: config::Config = from_str(&contents)?;

    let per_message_tag_limit = config.limits.per_message_tag_limit;
    println!("per_message_tag_limit: {:?}", per_message_tag_limit);

    Ok(())
}