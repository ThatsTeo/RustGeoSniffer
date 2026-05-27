// -=- BEEP BUZZER 3 TIMES -=-
use rppal::gpio::Gpio;
use std::thread;
use std::time::Duration;

// Define PIN for Buzzer
const BUZZ_PIN: u8 = 17;

pub fn beep_buzzer() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[GPIO DEBUG] Starting GPIO...");

    // Initialize the GPIO Interface
    let gpio = Gpio::new()?;

    let mut buzzer_output = gpio.get(BUZZ_PIN)?.into_output();

    println!("[GPIO DEBUG] Buzzer ready on PIN {}", BUZZ_PIN);

    for _ in 0..4 {
        buzzer_output.set_high();
        thread::sleep(Duration::from_millis(200));
        buzzer_output.set_low();
        thread::sleep(Duration::from_millis(100));
    }

    println!("[GPIO DEBUG] Beep ended...");
    Ok(())
}
