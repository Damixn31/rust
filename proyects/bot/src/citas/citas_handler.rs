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

        if msg.content == "!categorias" {
            let categories: Vec<&str> = self
                .quotes
                .categories
                .iter()
                .map(|c| c.name.as_str())
                .collect();
            if categories.is_empty() {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "No hay categorias disponibles.")
                    .await
                {
                    println!("Error al enviar el mensaje: {:?}", why);
                }
            } else {
                let response = format!("Categorias disponibles:\n{}", categories.join("\n"));
                if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                    println!("Error al enviar el mensaje: {:?}", why);
                }
            }
        }
        match msg.content.as_str() {
            s if msg.content.starts_with("!cita") => {
                let args: Vec<&str> = s.split_whitespace().collect();
                if args.len() > 1 {
                    let category = args[1].to_lowercase();
                    println!("Categoria espicificada por el usuario: {}", category);
                    if let Some(quote) = self.quotes.random_quote(&category) {
                        print!("Cita obtenida: {}", quote);
                        if let Err(why) = msg.channel_id.say(&ctx.http, quote).await {
                            println!("Error al enviar el mensaje: {:?}", why);
                        }
                        return;
                    }
                }
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "Formato incorrecto. Uso: !cita <categoria>")
                    .await
                {
                    println!("Error al enviar el mensaje: {:?}", why);
                }
            }
            _ => {}
        }
    }
}
