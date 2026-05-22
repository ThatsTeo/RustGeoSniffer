use rppal::gpio::Gpio;
use std::thread;
use std::time::Duration;

// Definiamo i numeri dei PIN usando la numerazione BCM (quella standard del Broadcom)
const LED_PIN: u4 = 18;
const BUTTON_PIN: u4 = 24;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Inizializzazione GPIO...");

    // Inizializza il gestore della GPIO del Raspberry Pi
    // Il punto di domanda (?) serve a gestire l'errore se la GPIO non è accessibile (es. permessi insufficienti)
    let gpio = Gpio::new()?;

    // Configura il PIN del LED come OUTPUT
    let mut led_output = gpio.get(LED_PIN)?.into_output();

    // Configura il PIN del pulsante come INPUT con resistenza di Pull-Up interna attiva
    let button_input = gpio.get(BUTTON_PIN)?.into_input_pullup();

    println!(
        "Sistema pronto! Premi il pulsante sul PIN BCM {} per far lampeggiare il LED sul PIN BCM {}",
        BUTTON_PIN, LED_PIN
    );

    // Ciclo principale (equivalente al tuo ciclo di sniffing o di controllo)
    for _ in 0..10 {
        // Legge lo stato del pulsante (is_low significa che è premuto se usiamo il Pull-Up)
        if button_input.is_low() {
            println!("Pulsante premuto! Accendo il LED.");

            // Accende il LED
            led_output.set_high();
            thread::sleep(Duration::from_millis(500));

            // Spegne il LED
            led_output.set_low();
            thread::sleep(Duration::from_millis(500));
        } else {
            // Se il pulsante non è premuto, aspetta un attimo prima di controllare di nuovo
            thread::sleep(Duration::from_millis(100));
        }
    }

    println!("Esecuzione terminata. I PIN verranno resettati automaticamente.");
    Ok(())
}
