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
    ruma::events::room::message::{MessageType, SyncRoomMessageEvent},
    Client, Room,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::set_log_level(LogLevel::Trace);
    let path = env::current_dir()?;
    let mut file = match OpenOptions::new().read(true).open("config.toml") {
         Ok(file) => file,
         Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
             log_info!("Configuration file not found, creating one in directory of executable...");
             config::write_config_defaults()?;
             log_info!("Default configuration file created in : {} , you will have to configure it.", path.display());
             print!("");
             process::exit(0x0100);
         },
         Err(e) => return Err(e.into()),
     };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let tables: config::Tables = from_str(&contents)?; //accessible struct for values
    println!("{:?}", tables.filters.message_filters);

    if tables.login.username == "@example-change-me:matrix.org" && 
       tables.login.password == "PASSWORD-HERE" {
            log_error!("Login credentials in the configuration file {}/config.toml are not set, please configure them.", path.display());
            process::exit(0x0100);
    }

    if tables.info.room_id == "example-room-id:matrix.org" {
        log_error!("Room ID for automatic moderation is not set in the configuration file {}/config.toml, please configure it.", path.display());
    }
    
    let homeserver_url = Url::parse(&format!("https://{}", tables.login.username.split(':').nth(1).unwrap()))?;
    let client = Client::new(homeserver_url).await?;
    match client.matrix_auth().login_username(&tables.login.username, &tables.login.password).device_id("SPAMSENTRY00").send().await {
        Ok(_) => {
            log_info!("Logged in as {}", &tables.login.username);
        }
            
        Err(error) => {
            log_error!("{}", error);
        }
    }
    
    client.add_event_handler(|ev: SyncRoomMessageEvent, room: Room| async move {
        println!("---");
        if let SyncRoomMessageEvent::Original(orig) = ev { // gets the Original from the SyncRoomMessageEvent
            if let MessageType::Text(text_content) = orig.content.msgtype {
                if tables.info.room_id == room.room_id().to_string() {
                    println!("---");
                    log_debug!("Message received\n");
                    log_debug!("Room name : {:?}", room.name().unwrap_or("None (Direct Message)".to_string()));
                    log_debug!("Room ID : {:?},", room.room_id());
                    
                    log_debug!("Sender: {}", orig.sender.to_string());
                    log_debug!("Body: {}", text_content.body);
                    println!("\n");
                }

                if tables.filters.message_filters.iter().any(|s| text_content.body.contains(s)) {
                    println!("Banning user {}", orig.sender.to_string());
                    match room.ban_user(&orig.sender, None).await {
                        Ok(_) => {
                            log_info!("User {} banned", orig.sender.to_string());
                        }
                        Err(error) => {
                            log_error!("{}", error);
                        }
                    }
                }
            }
            else {
                log_error!("SyncRoomMessageEvent::Original doesn't contain a MessageType::Text");
            }
        }
        else {
            log_error!("message event doesn't have an original, this should never happen.");
        }
    });

    client.sync(SyncSettings::default()).await?;
    Ok(())
}
