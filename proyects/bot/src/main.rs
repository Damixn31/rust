use std::env;

mod handlers {
    pub mod discord_handler;
    pub mod message_handler;
}

mod citas {
    pub mod citas_handler;
    pub mod citas_impl;
}

use citas::citas_handler::Citas;
use citas::citas_impl::Quotes;

extern crate serenity;
use handlers::discord_handler::DiscordHandler;
use handlers::message_handler::MessageHandler;

use serenity::framework::standard::macros::group;
use serenity::framework::standard::{Configuration, StandardFramework};

use serenity::prelude::*;

#[group]
struct General;

#[tokio::main]
async fn main() {
    let quotes = Quotes::new();
    let citas = Citas::new(quotes.clone());
    dotenv::dotenv().expect("Error al cargar el fichero .evn");

    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix("~"));

    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(DiscordHandler)
        //.event_handler(Citas { quotes })
        .event_handler(citas)
        .framework(framework)
        .await
        .expect("Error en el cliente");

    if let Err(msg) = client.start().await {
        println!("Error: {:?}", msg);
    }
}
