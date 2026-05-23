mod geolocation_api;
mod rppal_gpio;
mod wifi_scan;

fn main() {
    println!("Hello World!");
    // let reti = wifi_scan::iwlist_output_parsed();
    // print!("{:?}", reti);

    println!("{}", geolocation_api::get_api());
}
