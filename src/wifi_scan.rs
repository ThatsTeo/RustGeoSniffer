// -=- GET MACs FROM NEARBY ACCESS POINT -=-

use regex::Regex;
use std::collections::HashMap;
use std::process::Command;

pub fn iwlist_output_parsed() -> HashMap<String, i8> {
    let iwlist = Command::new("sh")
        .arg("-c")
        .arg("sudo iwlist wlan0 scanning")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&iwlist.stdout);

    // println!("iwlist output:\n{}", output);

    // let regex_essid = Regex::new(r#"ESSID:"([^"]+)""#).unwrap();
    let regex_mac = Regex::new(r"Address:\s+([0-9A-Fa-f:]{17})").unwrap();
    let regex_signal = Regex::new(r"Signal level=(-[0-9]+) dBm").unwrap();

    let mut networks: HashMap<String, i8> = HashMap::new();

    // MACs
    let mut vec_macs: Vec<String> = Vec::new();
    for cap in regex_mac.captures_iter(&output) {
        let mac_address = &cap[1];
        vec_macs.push(String::from(mac_address));
        // println!("MAC: {mac_address}");
    }
    // Signal Strenght
    let mut vec_sign: Vec<i8> = Vec::new();
    for cap in regex_signal.captures_iter(&output) {
        let sign_strenght = &cap[1];
        vec_sign.push(sign_strenght.parse().unwrap());
    }

    // Create HashMap for Google Cloud API
    for i in 0..vec_macs.len() {
        networks.insert(vec_macs[i].clone(), vec_sign[i]);
    }

    // C++ (end func with return var;) = Rust (last line wihout ; is the return)
    // return networks; = networks
    networks
}
