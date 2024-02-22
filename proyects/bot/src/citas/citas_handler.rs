use serenity::{async_trait, model::channel::Message, prelude::*};

use super::citas_impl::Quotes;

pub struct Citas {
    pub quotes: Quotes,
}

impl Citas {
    pub fn new(quotes: Quotes) -> Self {
        Citas { quotes }
    }
}
#[async_trait]
impl EventHandler for Citas {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("Mensaje recibido: {}", msg.content);
        if msg.content.starts_with("!cita") {
            let args: Vec<&str> = msg.content.split_whitespace().collect();
            if args.len() > 1 {
                let category = args[1].to_lowercase();
                //let mut args = msg.content.splitn(2, ' ');
                //args.next();
                //if let Some(category) = args.next() {
                //let category = category.trim().to_lowercase();
                println!("Categoria espicificada por el usuario: {}", category);
                if let Some(quote) = self.quotes.random_quote(&category) {
                    println!("Cita obtenida: {}", quote);
                    if let Err(why) = msg.channel_id.say(&ctx.http, quote).await {
                        println!("Error al enviar el mensaje: {:?}", why);
                    }
                    return;
                }
                //}
            }
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "Formato incorrecto. Uso: !cita <categoria>")
                .await
            {
                println!("Error al enviar el mensaje: {:?}", why);
            }
        }

        //if msg.content == "!cita" {
        //    let quote = self.quotes.random();
        //    if let Err(why) = msg.channel_id.say(&ctx.http, quote).await {
        //        println!("Error al enviar el mensaje: {:?}", why);
        //    }
        //}
    }
}
