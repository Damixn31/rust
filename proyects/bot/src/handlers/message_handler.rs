//use serenity::all::{ChannelId, Member};
//use serenity::model::channel::Message;
//use serenity::{prelude::*, Result};

//pub struct MessageHandler;

//impl MessageHandler {
//pub async fn send_welcome_message(
//    ctx: &Context,
//    new_member: &Member,
//    channel_id: &ChannelId,
//) -> Result<(), serenity::Error> {
//    let welcome_message = format!("Bienvenido a nuestro servidor, {}!", new_member.user.name);

//    if let Err(why) = channel_id.say(&ctx.http, &welcome_message).await {
//        println!("Error al enviar mensaje de Bienvenida: {:?}", why);
//return Err(why);
//    }
//    Ok(())
//}
//pub async fn handle_ping(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
//    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
//        println!("Error al enviar el mensaje: {:?}", why);
//return Err(why);
//    }
//    Ok(())
//}
//    pub async fn handle_hello(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
//        if msg.content == "!hola" {
//            let user_name = msg.author.name.clone();
//            if let Err(why) = msg
//                .channel_id
//                .say(
//                    &ctx.http,
//                    format!("Hola!, {} como te sientes hoy?", user_name),
//                )
//                .await
//            {
//                println!("Error al enviar el mensaje: {:?}", why);
//                //return Err(why);
//            }
//        }
//
//        Ok(())
//    }

//pub async fn handle_unknown_message(
//    ctx: &Context,
//    msg: &Message,
//) -> Result<(), serenity::Error> {
//    let user_name = msg.author.name.clone();
//    if let Err(why) = msg
//        .channel_id
//        .say(
//            &ctx.http,
//            format!("Lo siento no entendi eso. {}", user_name),
//        )
//        .await
//    {
//        println!("Error al enviar el mensaje: {:?}", why);
//    }
//    Ok(())
//}
//}
