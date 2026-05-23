use std::fs;

pub fn get_api() -> String {
    let contents =
        fs::read_to_string("config.txt").expect("[API DEBUG] File 'config.txt' not found!");
    let mut gcg_api: &str = "";

    for line in contents.lines() {
        let line = line.trim();

        if line.starts_with("GCGEOLOCATION_API=") {
            if let Some((_api, value)) = line.split_once('=') {
                println!("[API DEBUG] Geolocation API found! Returning API...");
                return value.to_string();
            }
        }
    }

    println!("[API DEBUG] API not found! Check again file `config.txt`...");
    String::new()
}
