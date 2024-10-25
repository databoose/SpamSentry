use std::fs::OpenOptions;
use std::io::Read;
use toml::from_str;

mod config;

use matrix_sdk::{
    Client, config::SyncSettings,
    ruma::{user_id, events::room::message::SyncRoomMessageEvent}
};

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
    
    //println!("contents: {}", contents);
    let tables: config::Tables = from_str(&contents)?; //accessible struct for limits values
    //println!("{:?}", tables.limits.per_message_tag_limit);
    //println!("{:?}", tables.login.username);

    let alice = user_id!("@alice:example.org");
    let client = Client::builder().server_name(alice.server_name()).build().await?;

    // First we need to log in.
    client.matrix_auth().login_username(alice, "password").send().await?;

    client.add_event_handler(|ev: SyncRoomMessageEvent| async move {
        println!("Received a message {:?}", ev);
    });

    // Syncing is important to synchronize the client state with the server.
    // This method will never return unless there is an error.
    client.sync(SyncSettings::default()).await?;
    
    Ok(())
}