use serenity::framework::standard::*;
use serenity::model::channel;
use serenity::prelude::*;

#[macros::command]
async fn echo(context: &Context, msg: &channel::Message, args: Args) -> CommandResult {
    match args.rest() {
        "" => {
            msg.reply(context, "No arguments provided to send.")
                .await
                .expect("Unable to send messsage.");
            return Ok(());
        }
        _ => {
            msg.channel_id
                .say(context, args.rest())
                .await
                .expect("Unable to send message.");
            return Ok(());
        }
    }
}
