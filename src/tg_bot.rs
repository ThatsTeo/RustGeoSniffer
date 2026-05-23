use teloxide::{dispatching::dialogue::GetChatId, prelude::*, utils::command::BotCommands};

use crate::geolocation_api::get_coords;
use crate::rppal_gpio::{self, beep_buzzer};

#[tokio::main]
pub async fn run_bot() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Send the current location.")]
    Locate,
    #[command(description = "Beep the buzzer 3 times", parse_with = "split")]
    Beep,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Locate => {
            bot.send_message(
                msg.chat.id,
                "[Rust]\nCalculating the location. This process usually takes a few seconds...",
            )
            .await?;
            let response = tokio::task::spawn_blocking(|| match locate_msg() {
                Ok(m) => m,
                Err(e) => format!("[ERROR] {e}"),
            })
            .await
            .unwrap_or_else(|e| format!("[ERROR] {e}"));

            bot.send_message(msg.chat.id, response).await?
        }
        Command::Beep => {
            bot.send_message(msg.chat.id, "[Rust]\nBeeping the buzzer 3 times...")
                .await?;
            rppal_gpio::beep_buzzer();
            bot.send_message(msg.chat.id, "[Rust]\nBuzzer finished beeping.")
                .await?
        }
    };

    Ok(())
}

fn locate_msg() -> Result<(String), Box<dyn std::error::Error>> {
    let (lat, long, acc) = get_coords()?;
    let mut msg = String::from("Accuracy to high, it's not ideal to update location now...");
    if acc <= 100f32 {
        msg = format!(
            "[Rust]\nPosition: {lat}, {long}\nAccuracy: {acc}\nhttps://www.google.com/maps/place/{lat},{long}"
        );
    }
    Ok(msg)
}
