mod geolocation_api;
mod rppal_gpio;
mod tg_bot;
mod wifi_scan;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[DEBUG] Program test...");
    // let reti = wifi_scan::iwlist_output_parsed();
    // print!("{:?}", reti);

    let (lat, long, acc) = geolocation_api::get_coords()?;
    println!("{}, {}, {}", lat, long, acc);
    println!("[DEBUG] Test ended!");
    Ok(())
}
