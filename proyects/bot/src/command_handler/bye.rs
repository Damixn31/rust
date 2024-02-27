use serenity::model::channel::Message;
use serenity::{prelude::*, Result};

pub async fn handle_bye(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
    if msg.content == "chau" {
        let user_name = msg.author.name.clone();
        if let Err(why) = msg
            .channel_id
            .say(&ctx.http, format!("Hasta luego, {}", user_name))
            .await
        {
            println!("Error al enviar el mensaje: {:?}", why);
            //return Err(why);
        }
    }
    Ok(())
}
