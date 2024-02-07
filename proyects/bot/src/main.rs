use std::env;

extern crate serenity;
use dotenv::dotenv;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{
    help_commands, CommandResult, Configuration, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::channel::Reaction;
use serenity::model::gateway::Ready;
use serenity::{async_trait, prelude::*};

#[group]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        //let reaction_message = add_reaction
        //    .message(&ctx.http)
        //    .await
        //    .expect("Failed to get message");
        //let user = add_reaction.user(&ctx).await.expect("Failed to get user");
        //println!(
        //    "El usuario {} agrego una reaccion {} al mensaje {}",
        //    user.name, add_reaction.emoji, reaction_message.content
        //);
        //if let Err(why) = add_reaction
        //   .channel_id
        //   .say(
        //       &ctx.http, content: format!("{} left a reaction", add_reaction.user_id)
        //   )
        //   .await
        //{
        //    println!("error racting to a reaction: {:?}", why);
        //}
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "?ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong?").await {
                println!("Error giving messsage: {:?}", why);
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Error al cargar el fichero .evn");

    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix("~"));

    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error en el cliente");

    if let Err(msg) = client.start().await {
        println!("Error: {:?}", msg);
    }
}
