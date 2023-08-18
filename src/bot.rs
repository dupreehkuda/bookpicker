use crate::err::CustomError as Err;
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
    #[command(description = "starts club", parse_with = "split")]
    Start,
    #[command(description = "new event")]
    Event(String),
    #[command(description = "new suggestion")]
    Suggest(String),
    #[command(description = "achieves active event")]
    Achieve,
    #[command(description = "picks a subject for event")]
    Pick,
    #[command(description = "current event info")]
    Current,
}

fn default_service_blocking() -> Service {
    let rt = Handle::current();
    tokio::task::block_in_place(|| rt.block_on(default_service()))
}

lazy_static! {
    static ref SERVICE: Service = default_service_blocking();
}

async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Start => {
            let mut message =
                "You're all set up! Now you can create event for your club".to_string();

            if let Err(err) = SERVICE.register_new_club(msg.chat.id.0).await {
                let db_err = err.downcast_ref::<tokio_postgres::Error>().unwrap();
                if db_err.code().unwrap() == &SqlState::UNIQUE_VIOLATION {
                    message = "You're already started a bookclub".to_string();
                }
            }

            bot.send_message(msg.chat.id, message).await?
        }
        Command::Event(date) => {
            if date.is_empty() {
                bot.send_message(
                    msg.chat.id,
                    "Please write a date in format -\n/event 2023.07.16 15:00".to_string(),
                )
                .await?;

                return Ok(());
            }

            let mut message = String::new();

            match SERVICE.new_club_event(msg.chat.id.0, date.as_str()).await {
                Ok(date) => message = format!("New club event created on {}", date),
                Err(err) => {
                    let er = err.downcast_ref::<Err>().unwrap();
                    message = er.to_string()
                }
            }

            bot.send_message(msg.chat.id, message).await?
        }
        Command::Suggest(suggestion) => {
            if suggestion.is_empty() {
                bot.send_message(
                    msg.chat.id,
                    "Your suggestion is empty ;(\nFormat - /suggest smth".to_string(),
                )
                .await?;

                return Ok(());
            }

            let mut message = format!("Got it. Your suggestion:\n{}", suggestion);

            if let Err(err) = SERVICE
                .new_member_suggestion(
                    msg.chat.id.0,
                    msg.from().unwrap().id.0 as u32,
                    suggestion.as_str(),
                )
                .await
            {
                let er = err.downcast_ref::<Err>().unwrap();
                message = er.to_string()
            }

            bot.send_message(msg.chat.id, message).await?
        }
        Command::Achieve => {
            let message: String;

            match SERVICE.achieve_active_event(msg.chat.id.0).await {
                Ok(date) => message = format!("Ok, event on {} achieved", date),
                Err(err) => {
                    let er = err.downcast_ref::<Err>().unwrap();
                    message = er.to_string()
                }
            }

            bot.send_message(msg.chat.id, message).await?
        }
        Command::Pick => {
            let message: String;

            match SERVICE.pick_from_suggestions(msg.chat.id.0).await {
                Ok(subject) => message = format!("Randomly picked\n{}", subject),
                Err(err) => {
                    let er = err.downcast_ref::<Err>().unwrap();
                    message = er.to_string()
                }
            }

            bot.send_message(msg.chat.id, message).await?
        }
        Command::Current => {
            let message: String;

            match SERVICE.get_current_event_info(msg.chat.id.0).await {
                Ok(text) => message = text,
                Err(err) => {
                    let er = err.downcast_ref::<Err>().unwrap();
                    message = er.to_string()
                }
            }

            bot.send_message(msg.chat.id, message).await?
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
