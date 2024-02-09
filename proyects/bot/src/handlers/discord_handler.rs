use serenity::{async_trait, model::channel::Message, prelude::*};

use serenity::model::gateway::Ready;

use crate::MessageHandler;

pub struct DiscordHandler;

#[async_trait]
impl EventHandler for DiscordHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            "!ping" => {
                if let Err(why) = MessageHandler::handle_ping(&ctx, &msg).await {
                    println!("Error al enviar ping: {:?}", why);
                }
            }
            "hola" => {
                if let Err(why) = MessageHandler::handle_hello(&ctx, &msg).await {
                    println!("Error al manejar hola: {:?}", why);
                }
            }
            _ => {
                println!("Mensaje no reconocido: {}", msg.content);
            }
        }
        if msg.content.contains("!ping") {
            if let Err(why) = MessageHandler::handle_ping(&ctx, &msg).await {
                println!("Error al manejar ping: {:?}", why);
            }
        }
    }
    //async fn message(&self, ctx: Context, msg: Message) {
    //    if msg.content.contains("hola") {
    //        if let Err(why) = MessageHandler::handle_hello(&ctx, &msg).await {
    //            println!("Error al enviar saludo: {:?}", why)
    //        }
    //    }
    //}
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}
