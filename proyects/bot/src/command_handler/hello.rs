use serenity::model::channel::Message;
use serenity::{prelude::*, Result};

pub async fn handle_hello(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
    if msg.content == "!hola" {
        let user_name = msg.author.name.clone();
        if let Err(why) = msg
            .channel_id
            .say(
                &ctx.http,
                format!("Hola!, {} como te sientes hoy?", user_name),
            )
            .await
        {
            println!("Error al enviar el mensaje: {:?}", why);
            //return Err(why);
        }
    }

    Ok(())
}
