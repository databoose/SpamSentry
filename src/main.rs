use std::fs::OpenOptions;
use std::io::Read;
use std::env;
use std::process;

use url::Url;
use toml::from_str;
use log_x::{loggers::{ global_logger::DefaultLoggerTrait, log_levels::LogLevel }, 
                       timestamp, log_info, log_error, log_warn, log_debug, Logger};

mod config;
use matrix_sdk::{
    config::SyncSettings,
    ruma::{
        events::room::message::{MessageType, SyncRoomMessageEvent},
    },
    Client, Room, RoomState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::set_log_level(LogLevel::Trace);
    let path = env::current_dir()?;
    let mut file = match OpenOptions::new().read(true).open("config.toml") {
       Ok(file) => file,
       Err(NotFound) => {
            log_info!("Configuration file not found, creating one in directory of executable...");
            config::write_config_defaults()?;
            log_info!("Configuration file created in : {}", path.display());
            print!("");
            OpenOptions::new().read(true).open("config.toml")?
        },
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let tables: config::Tables = from_str(&contents)?; //accessible struct for limits values
    //println!("{:?}", tables.limits.per_message_tag_limit);
    //println!("{:?}", tables.login.username);

    if tables.login.username == "@example-change-me:matrix.org" && 
       tables.login.password == "PASSWORD-HERE" {
            log_error!("Login credentials in the configuration file {}config.toml are not set, please configure them.", path.display());
            process::exit(0x0100);
    }

    let homeserver_url = Url::parse(&format!("https://{}", tables.login.username.split(':').nth(1).unwrap()))?;
    let client = Client::new(homeserver_url).await?;

    match client.matrix_auth().login_username(&tables.login.username, &tables.login.password).send().await {
        Ok(_) => {
            log_info!("Logged in as {}", &tables.login.username);
        }
        
        Err(error) => {
            log_error!("{}", error);
        }
    }

    client.add_event_handler(|ev: SyncRoomMessageEvent| async move {
        println!("Received a message {:?}", ev);
    });
    client.sync(SyncSettings::default()).await?;
    
    Ok(())
}
