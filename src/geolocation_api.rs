use reqwest;
use std::fs;

use crate::wifi_scan::iwlist_output_parsed;

fn get_api() -> String {
    let contents =
        fs::read_to_string("config.txt").expect("[API DEBUG] File 'config.txt' not found!");

    for line in contents.lines() {
        let line = line.trim();

        if line.starts_with("GOOGLE_API_KEY=") {
            if let Some((_api, value)) = line.split_once('=') {
                println!("[API DEBUG] Geolocation API found!");
                return value.to_string();
            }
        }
    }

    println!("[API DEBUG] API not found! Check again file `config.txt`...");
    String::new()
}

pub fn get_coords() -> Result<(f64, f64, f32), Box<dyn std::error::Error + Send + Sync>> {
    let url = "https://www.googleapis.com/geolocation/v1/geolocate?key=";
    let gcg_api = get_api();

    // init payload
    let mut payload = serde_json::json!({
        "wifiAccessPoints": []
    });

    // parse JSON
    let networks = iwlist_output_parsed();
    for (mac, rssi) in &networks {
        if let Some(array) = payload["wifiAccessPoints"].as_array_mut() {
            array.push(serde_json::json!({
                "macAddress": mac,
                "signalStrength": rssi
            }));
        }
    }

    let res = reqwest::blocking::Client::new()
        .post(format!("{url}{gcg_api}"))
        .json(&payload)
        .send()?
        .json::<serde_json::Value>()?;

    let lat = res["location"]["lat"].as_f64().unwrap_or(0.0) as f64;
    let lng = res["location"]["lng"].as_f64().unwrap_or(0.0) as f64;
    let accuracy = res["accuracy"].as_f64().unwrap_or(0.0) as f32;

    Ok((lat, lng, accuracy))
}
