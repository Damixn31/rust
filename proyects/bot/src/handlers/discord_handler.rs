//use serenity::all::{ChannelId, Member};
use serenity::{async_trait, model::channel::Message, prelude::*};

use serenity::model::gateway::Ready;

//use crate::citas::citas_handler::MessageCitas;
use crate::citas::citas_impl::Quotes;
use crate::command_handler::bye::handle_bye;
use crate::command_handler::good::handle_good;
use crate::command_handler::hello::handle_hello;
//use crate::MessageHandler;

//pub struct Citas {
//    pub quotes: Quotes,
//}

//impl Citas {
//    pub fn new(quotes: Quotes) -> Self {
//        Citas { quotes }
//    }
//}

pub struct DiscordHandler {
    pub quotes: Quotes,
}

#[async_trait]
impl EventHandler for DiscordHandler {
    //async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
    //    println!("Nuevo miembro agregado al servidor: {:?}", new_member);
    //    let channel_id = ChannelId::new(1204850870945063073);

    //    if let Err(why) = MessageHandler::send_welcome_message(&ctx, &new_member, &channel_id).await
    //    {
    //        println!("Error al enviar el mensaje de bienvenida! {:?}", why);
    //    }
    // }
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        match msg.content.as_str() {
            "!hola" => {
                if let Err(why) = handle_hello(&ctx, &msg).await {
                    println!("Error al manejar hola: {:?}", why);
                }
            }
            "bien" => {
                if let Err(why) = handle_good(&ctx, &msg).await {
                    println!("Error al manejar bien: {:?}", why);
                }
            }
            "chau" => {
                if let Err(why) = handle_bye(&ctx, &msg).await {
                    println!("Error al enviar chau: {:?}", why);
                }
            }
            "!motivacion" => {
                if let Some(quote) = self.quotes.random_quote("motivacion") {
                    if let Err(why) = msg.channel_id.say(&ctx.http, quote).await {
                        println!("Error al enviar la cita: {:?}", why);
                    }
                }
            }
            "!inspiracion" => {
                if let Some(quote) = self.quotes.random_quote("inspiracion") {
                    if let Err(why) = msg.channel_id.say(&ctx.http, quote).await {
                        println!("Error al enviar la cita: {:?}", why);
                    }
                }
            }
            _ => {
                let _ = msg
                    .channel_id
                    .say(&ctx.http, "Lo siento, no puedo entender eso.")
                    .await;
                println!("Mensaje no reconocido!: {}", msg.content);
                //return;
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}
