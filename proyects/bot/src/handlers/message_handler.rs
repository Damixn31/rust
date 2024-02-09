use serenity::model::channel::Message;
use serenity::{prelude::*, Result};

pub struct MessageHandler;

impl MessageHandler {
    pub async fn handle_ping(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
            println!("Error al enviar el mensaje: {:?}", why);
            return Err(why);
        }
        Ok(())
    }
    pub async fn handle_hello(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
        if msg.content == "hola" {
            let user_name = msg.author.name.clone();
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, format!("Hola, {}!", user_name))
                .await
            {
                println!("Error al enviar el mensaje: {:?}", why);
                return Err(why);
            }
        }

        Ok(())
    }
}
