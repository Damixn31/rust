use serenity::all::{ChannelId, Member};
use serenity::{async_trait, model::channel::Message, prelude::*};

use serenity::model::gateway::Ready;

use crate::MessageHandler;

pub struct DiscordHandler;

#[async_trait]
impl EventHandler for DiscordHandler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        println!("Nuevo miembro agregado al servidor: {:?}", new_member);
        let channel_id = ChannelId::new(1204850870945063073);

        if let Err(why) = MessageHandler::send_welcome_message(&ctx, &new_member, &channel_id).await
        {
            println!("Error al enviar el mensaje de bienvenida! {:?}", why);
        }
    }
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
            "bien" => {
                if let Err(why) = MessageHandler::handle_good(&ctx, &msg).await {
                    println!("Error al manejar bien: {:?}", why);
                }
            }
            "chau" => {
                if let Err(why) = MessageHandler::handle_bye(&ctx, &msg).await {
                    println!("Error al enviar chau: {:?}", why);
                }
            }
            _ => {
                let _ = msg
                    .channel_id
                    .say(&ctx.http, "Lo siento no entendi eso.")
                    .await;

                println!("Mensaje no reconocido: {}", msg.content);
            }
        }
        //if msg.content.contains("!ping") {
        //    if let Err(why) = MessageHandler::handle_ping(&ctx, &msg).await {
        //        println!("Error al manejar ping: {:?}", why);
        //    }
        //}
        //
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}
