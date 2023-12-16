use crate::err::CustomError as Err;
use crate::service::{default_service, Service};
use dotenv::dotenv;
use lazy_static::lazy_static;
use teloxide::types::ParseMode::MarkdownV2;
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
    #[command(description = "create new event")]
    Event(String),
    #[command(description = "make new suggestion")]
    Suggest(String),
    #[command(description = "achieves active event")]
    Achieve,
    #[command(description = "picks a subject for active event")]
    Pick,
    #[command(description = "current event info")]
    Current,
    #[command(description = "turns insights on/off for current event")]
    Insights,
    #[command(description = "starts current event only if insights enabled (to get summary link)")]
    StartClub,
}

fn default_service_blocking() -> Service {
    let rt = Handle::current();
    tokio::task::block_in_place(|| rt.block_on(default_service()))
}

lazy_static! {
    static ref SERVICE: Service = default_service_blocking();
}

async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let mut message: String;

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .disable_notification(true)
                .await?
        }
        Command::Start => {
            message = "You're all set up! Now you can create event for your club".to_string();

            if let Err(err) = SERVICE.register_new_club(msg.chat.id.0).await {
                let db_err = err.downcast_ref::<tokio_postgres::Error>().unwrap();
                if db_err.code().unwrap() == &SqlState::UNIQUE_VIOLATION {
                    message = "You're already started a club".to_string();
                }
            }

            bot.send_message(msg.chat.id, message)
                .disable_notification(true)
                .await?
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

            match SERVICE.new_club_event(msg.chat.id.0, date.as_str()).await {
                Ok(date) => message = format!("New club event created on {}", date),
                Err(err) => {
                    let er = err.downcast_ref::<Err>().unwrap();
                    message = er.to_string()
                }
            }

            bot.send_message(msg.chat.id, message)
                .disable_notification(true)
                .await?
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

            message = format!("Got it. Your suggestion:\n{}", suggestion);

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

            bot.send_message(msg.chat.id, message)
                .disable_notification(true)
                .await?
        }
        Command::Insights => {
            match SERVICE.toggle_with_insights(msg.chat.id.0).await {
                Ok(text) => message = text,
                Err(err) => {
                    let er = err.downcast_ref::<Err>().unwrap();
                    message = er.to_string()
                }
            }

            bot.send_message(msg.chat.id, message)
                .disable_notification(true)
                .await?
        }
        Command::StartClub => {
            match SERVICE.start_active_event(msg.chat.id.0).await {
                Ok(text) => message = text,
                Err(err) => {
                    let er = err.downcast_ref::<Err>().unwrap();
                    message = er.to_string()
                }
            }

            bot.send_message(msg.chat.id, message)
                .parse_mode(MarkdownV2)
                .disable_notification(true)
                .await?
        }
        Command::Achieve => {
            match SERVICE.achieve_active_event(msg.chat.id.0).await {
                Ok(date) => message = format!("Ok, event on {} is achieved", date),
                Err(err) => {
                    let er = err.downcast_ref::<Err>().unwrap();
                    message = er.to_string()
                }
            }

            bot.send_message(msg.chat.id, message)
                .disable_notification(true)
                .await?
        }
        Command::Pick => {
            match SERVICE.pick_from_suggestions(msg.chat.id.0).await {
                Ok(text) => message = text,
                Err(err) => {
                    let er = err.downcast_ref::<Err>().unwrap();
                    message = er.to_string()
                }
            }

            bot.send_message(msg.chat.id, message)
                .disable_web_page_preview(true)
                .disable_notification(true)
                .parse_mode(MarkdownV2)
                .await?
        }
        Command::Current => {
            match SERVICE.get_current_event_info(msg.chat.id.0).await {
                Ok(text) => message = text,
                Err(err) => {
                    let er = err.downcast_ref::<Err>().unwrap();
                    message = er.to_string()
                }
            }

            bot.send_message(msg.chat.id, message)
                .parse_mode(MarkdownV2)
                .disable_notification(true)
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
