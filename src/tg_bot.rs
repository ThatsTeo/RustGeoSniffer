use teloxide::{prelude::*, utils::command::BotCommands};

use crate::geolocation_api::get_coords;
use crate::rppal_gpio::{self};

#[tokio::main]
pub async fn run_bot() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, |bot: Bot, msg: Message, cmd: Command| async move {
        answer(bot, msg, cmd).await
    })
    .await;
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
    #[command(description = "Beep the buzzer 3 times")]
    Beep,
    #[command(description = "Set a delay between scans (used for /startautolocate)")]
    SetDelay,
    #[command(description = "Start automaticcaly locate the device")]
    StartAutoLocate,
    #[command(description = "Stop auto locate if active")]
    StopAutoLocate,
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
                "Calculating the location. This process usually takes a few seconds...",
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
            bot.send_message(msg.chat.id, "Beeping the buzzer 3 times...")
                .await?;

            // Beep the buzzer
            if let Err(e) = rppal_gpio::beep_buzzer() {
                let err_msg = e.to_string();
                println!("[BOT ERROR]: {}", err_msg);
                bot.send_message(msg.chat.id, "Couldn't make the buzzer beep!")
                    .await;
            }
            bot.send_message(msg.chat.id, "Buzzer finished beeping.")
                .await?
        }
        Command::SetDelay => bot.send_message(msg.chat.id, "Delay").await?,
        Command::StartAutoLocate => bot.send_message(msg.chat.id, "Start").await?,
        Command::StopAutoLocate => bot.send_message(msg.chat.id, "Stop").await?,
    };

    Ok(())
}

fn locate_msg() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let (lat, long, acc) = get_coords()?;
    let mut msg = String::from("Accuracy to high, it's not ideal to update location now...");
    if acc <= 100f32 {
        msg = format!(
            "Position: {lat}, {long}\nAccuracy: {acc}\nhttps://www.google.com/maps/place/{lat},{long}"
        );
    }
    Ok(msg)
}
