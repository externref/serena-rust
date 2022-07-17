use serenity::async_trait;
use serenity::framework::standard::*;
use serenity::model::channel;
use serenity::model::gateway;
use serenity::prelude::*;
use serenity::Client;

#[macros::group]
#[commands(ping, echo)]

struct GeneralCommands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, event: gateway::Ready) {
        println!(
            "Bot is online.\nUser: {}\nUser ID: {}",
            event.user.name, event.user.id
        );
    }
}
#[tokio::main]
async fn main() {
    let mut framework = StandardFramework::new()
        .configure(|c| c.prefix("!").case_insensitivity(true).with_whitespace(true));
    framework.group_add(&GENERALCOMMANDS_GROUP);
    let intents = GatewayIntents::all();
    let token = std::env::var("TOKEN").expect("Error while getting token");
    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error in creating client");
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {}", why);
    }
}

#[macros::command]
async fn ping(context: &Context, msg: &channel::Message) -> CommandResult {
    
    msg.reply(context, "pong!").await?;

    Ok(())
}

#[macros::command]
async fn echo(context: &Context, msg: &channel::Message, args: Args) -> CommandResult {
    match args.rest() {
        "" => {
            msg.reply(context, "No arguments provided to send.").await;
            return Ok(());
        }
        _ => {
            msg.channel_id.say(context, args.rest()).await;
            return Ok(());
        }
    }
}
