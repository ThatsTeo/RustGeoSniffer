mod geolocation_api;
mod rppal_gpio;
mod tg_bot;
mod wifi_scan;

fn main() {
    // println!("[DEBUG] Program test...");
    // let (lat, long, acc) = geolocation_api::get_coords()?;
    // println!("{}, {}, {}", lat, long, acc);
    // println!("[DEBUG] Test ended!");

    println!("[BOT DEBUG] Started bot..");
    std::thread::spawn(|| tg_bot::run_bot());
    loop {
        println!("Loop in progres...");
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
