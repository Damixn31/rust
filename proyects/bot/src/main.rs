use std::collections::HashSet;
use std::env;

mod handlers {
    pub mod discord_handler;
    pub mod message_handler;
}

extern crate serenity;
use dotenv::dotenv;
use handlers::discord_handler::DiscordHandler;
use handlers::message_handler::MessageHandler;

use serenity::all::{ChannelId, MessageId, UserId};
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{
    help_commands, CommandResult, Configuration, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::channel::Reaction;
use serenity::model::gateway::Ready;
use serenity::{async_trait, prelude::*};

#[group]
struct General;

//struct Handler;

//#[async_trait]
//impl EventHandler for Handler {
//    async fn message(&self, ctx: Context, msg: Message) {
//        if msg.content == "hola" {
//            let user_name = msg.author.name.clone();
//            if let Err(why) = msg
//                .channel_id
//                .say(&ctx.http, format!("Hola, {}!", user_name))
//                .await
//           {
//               println!("Error al enviar el mensaje: {:?}", why);
//           }
//       }
//   }
//    async fn ready(&self, _: Context, ready: Ready) {
//        println!("{} is ready", ready.user.name);
//    }
//}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Error al cargar el fichero .evn");

    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix("~"));

    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(DiscordHandler)
        .framework(framework)
        .await
        .expect("Error en el cliente");

    if let Err(msg) = client.start().await {
        println!("Error: {:?}", msg);
    }
}
