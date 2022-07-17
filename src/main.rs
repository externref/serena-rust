use serenity::async_trait;
use serenity::framework::standard::*;
use serenity::model::gateway;
use serenity::prelude::*;
use serenity::Client;

mod commands;

use crate::commands::echo::*;
use crate::commands::ping::*;

#[macros::group]
#[commands(ping, echo)]
struct GeneralCommands;

struct Handler {
    pool: sqlx::SqlitePool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, event: gateway::Ready) {
        println!(
            "Bot is online.\nUser: {}\nUser ID: {}",
            event.user.name, event.user.id
        );
        sqlx::query!(
            "
            CREATE TABLE IF NOT EXISTS prefixes (
                guild_id BIGINT,
                prefix VARCHAR(10)
            );
            "
        )
        .execute(&self.pool)
        .await
        .expect("Error in creating database.");
    }
}

#[tokio::main]
async fn main() {
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("prefixes.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Unable to connect to DB.");
    /*sqlx::migrate!("./migrations")
    .run(&sqlite_pool)
    .await
    .expect("Couldn't run database migrations");*/
    let handler = Handler { pool: sqlite_pool };
    let mut framework = StandardFramework::new()
        .configure(|c| c.prefix("!").case_insensitivity(true).with_whitespace(true));
    framework.group_add(&GENERALCOMMANDS_GROUP);
    let intents = GatewayIntents::all();
    let token = std::env::var("TOKEN").expect("Error while getting token");
    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(handler)
        .await
        .expect("Error in creating client");
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {}", why);
    }
}
