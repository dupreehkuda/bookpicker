use crate::service::{default_service, Service};
use dotenv::dotenv;
use lazy_static::lazy_static;
use teloxide::{prelude::*, types::Message, utils::command::BotCommands};
use tokio::runtime::Handle;
use tokio_postgres::error::SqlState;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Starts bookpicker", parse_with = "split")]
    Start,
    #[command(description = "handle new event creation")]
    NewEvent(String),
}

fn default_service_blocking() -> Service {
    let rt = Handle::current();

    tokio::task::block_in_place(|| rt.block_on(default_service()))
}

lazy_static! {
    static ref SERVICE: Service = default_service_blocking();
}

async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    println!("debug 6");
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Start => {
            let result = SERVICE.register_new_bookclub(msg.chat.id.0).await;
            if let Err(err) = result {
                if let Some(db_err) = err.as_db_error() {
                    if db_err.code() == &SqlState::UNIQUE_VIOLATION {
                        bot.send_message(
                            msg.chat.id,
                            "You're already started a bookclub".to_string(),
                        )
                        .await?;
                        return Ok(());
                    }
                }
            }

            bot.send_message(
                msg.chat.id,
                "You're all set up! Now you can create event for your bookclub".to_string(),
            )
            .await?
        }
        Command::NewEvent(date) => {
            SERVICE
                .new_book_club_event(msg.chat.id.0, date.as_str())
                .await
                .unwrap();
            bot.send_message(
                msg.chat.id,
                format!("New bookclub event created on {}", date),
            )
            .await?
        }
    };

    Ok(())
}

pub async fn run() {
    dotenv().ok();
    pretty_env_logger::init();

    let bot = Bot::from_env();

    bot.set_my_commands(Command::bot_commands())
        .await
        .expect("Failed to set bot commands");

    let handler = dptree::entry().branch(
        Update::filter_message()
            .filter_command::<Command>()
            .endpoint(command_handler),
    );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
