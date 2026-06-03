use std::sync::atomic::{AtomicBool, Ordering};
use std::time;
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::geolocation_api::get_coords;
use crate::rppal_gpio::{self};

static STOP_THREAD: AtomicBool = AtomicBool::new(false);

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
    SetDelay(u64),
    #[command(description = "Start automaticcaly locate the device")]
    StartAutoLocate,
    #[command(description = "Stop auto locate if active")]
    StopAutoLocate,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let delay: u64 = 1;

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
                let _ = bot
                    .send_message(msg.chat.id, "Couldn't make the buzzer beep!")
                    .await;
            }
            bot.send_message(msg.chat.id, "Buzzer finished beeping.")
                .await?
        }
        Command::SetDelay(delay) => {
            bot.send_message(msg.chat.id, format!("Delay setted to {delay} minute(s)!"))
                .await?
        }
        Command::StartAutoLocate => {
            if STOP_THREAD.load(Ordering::Relaxed) {
                bot.send_message(msg.chat.id, format!("Auto location is already started!"))
                    .await?
            } else {
                STOP_THREAD.store(false, Ordering::Relaxed);
                let bot_clone = bot.clone();
                let msg_clone = msg.clone();

                tokio::spawn(async move {
                    if let Err(e) = auto_locate(delay, bot_clone, msg_clone).await {
                        println!("[AUTO LOCATE ERROR]: {e}");
                    }
                });
                bot.send_message(msg.chat.id, "Auto locate started!")
                    .await?
            }
        }
        Command::StopAutoLocate => {
            if STOP_THREAD.load(Ordering::Relaxed) {
                bot.send_message(msg.chat.id, format!("Auto location is alredy off!"))
                    .await?
            } else {
                STOP_THREAD.store(true, Ordering::Relaxed);
                bot.send_message(msg.chat.id, "Auto locating stopped!")
                    .await?
            }
        }
    };

    Ok(())
}

fn locate_msg() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let (lat, long, acc) = get_coords()?;
    let mut msg = String::from("Accuracy to high, it's not ideal to update location now...");
    if acc <= 100f32 {
        msg = format!(
            "Position: {lat}, {long}\nAccuracy: {acc} meter(s)\nhttps://www.google.com/maps/place/{lat},{long}"
        );
    }
    Ok(msg)
}

async fn auto_locate(
    delay: u64,
    bot: Bot,
    msg: Message,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    loop {
        if STOP_THREAD.load(Ordering::Relaxed) {
            STOP_THREAD.store(false, Ordering::Relaxed);
            return Ok(String::from("Auto locate stopped."));
        }
        let response = tokio::task::spawn_blocking(|| match locate_msg() {
            Ok(m) => m,
            Err(e) => format!("[ERROR] {e}"),
        })
        .await
        .unwrap_or_else(|e| format!("[ERROR] {e}"));

        bot.send_message(msg.chat.id, response).await?;

        tokio::time::sleep(time::Duration::from_mins(delay)).await;
    }
}
