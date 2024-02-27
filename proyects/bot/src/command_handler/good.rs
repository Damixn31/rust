use serenity::model::channel::Message;
use serenity::{prelude::*, Result};

pub async fn handle_good(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
    let user_name = msg.author.name.clone();
    if let Err(why) = msg
        .channel_id
        .say(&ctx.http, format!("En que puedo ayudarte {}", user_name))
        .await
    {
        println!("Error al enviar el mensaje: {:?}", why);
    }
    Ok(())
}
