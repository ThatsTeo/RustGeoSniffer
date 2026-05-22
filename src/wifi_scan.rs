// -=- GET MACs FROM NEARBY ACCESS POINT -=-
// use regex::Regex;
// use std::process::Command;

// fn main() {
//     let iwlist = Command::new("sh")
//         .arg("-c")
//         .arg("sudo iwlist wlan0 scanning")
//         .output()
//         .expect("failed to execute process");

//     let output = String::from_utf8_lossy(&iwlist.stdout);

//     // println!("iwlist output:\n{}", output);

//     let regex_mac = Regex::new(r"Address:\s+([0-9A-Fa-f:]{17})").unwrap();

//     for cap in regex_mac.captures_iter(&output) {
//         let mac_address = &cap[1];

//         println!("MAC: {mac_address}");
//     }
// }
