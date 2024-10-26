use std::fs::OpenOptions;
use std::io::Read;
use toml::from_str;

mod config;

use matrix_sdk::{
    config::SyncSettings,
    ruma::{
        api::client::session::get_login_types::v3::{IdentityProvider, LoginType},
        events::room::message::{MessageType, SyncRoomMessageEvent},
    },
    Client, Room, RoomState,
};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = match OpenOptions::new().read(true).open("config.toml") {
       Ok(file) => file,
       Err(NotFound) => {
            println!("Configuration file not found, creating one in directory of executable...");
            config::write_config_defaults()?;
            OpenOptions::new().read(true).open("config.toml")?
        },
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let tables: config::Tables = from_str(&contents)?; //accessible struct for limits values
    //println!("{:?}", tables.limits.per_message_tag_limit);
    //println!("{:?}", tables.login.username);

    let homeserver_url = Url::parse(&format!("https://{}", tables.login.username.split(':').nth(1).unwrap()))?;
    let client = Client::new(homeserver_url).await?;
    client.matrix_auth().login_username(&tables.login.username, &tables.login.password).send().await?;

    client.add_event_handler(|ev: SyncRoomMessageEvent| async move {
        println!("Received a message {:?}", ev);
    });
    client.sync(SyncSettings::default()).await?;
    
    Ok(())
}
