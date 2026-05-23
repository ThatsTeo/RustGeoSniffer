# GeoSniffBot 🛜

![License](https://img.shields.io/badge/license-GPL--3.0-green)
![Language](https://img.shields.io/badge/language-Rust-orange)
![Platform](https://img.shields.io/badge/platform-Raspberry%20Pi%205-red)

> A fast and smart way to locate a Raspberry Pi 5 without GPS

## 📑 Overview

**GeoSniffBot** started as a school project all developed by me in C++. This is a Rust port from the original repository: https://github.com/ThatsTeo/GeoSniffer
 
- To start, the main code relies on the `sudo iwlist wlan0 scanning` Linux command, which is used to obtain the MAC Address (physical network card address) from all near Access Points (aka Routers).
- After collecting the address, a function performs a POST to Google Cloud Geolocation API to triangulate the position of the Raspberry (works best with at least 2 nearby routers!).
- The output is then displayed through a Telegram Bot created to send the location directly to the users phone, with also a manual request via a `/locate` command.

### Demo on Telegram
| Command                     | Result                    |
|-----------------------------|---------------------------|
| `/locate` (good accuracy)   | Location found            |
| `/locate` (accuracy > 100m) | Too imprecise             |
| `/beep`                     | Buzzer activated          |
| `/help`                     | Display possible commands |


<img src="assets/core_demo.png" width="250"/>

---

## How it works ⚙️

```
┌────────────────────────────────────┐
│     sudo iwlist wlan0 scanning     │  ← Obtain MACs
│        Parse Wi-Fi output          │
└─────────────────┬──────────────────┘
                  │ 
┌─────────────────▼───────────────────┐
│      POST to Google Cloud API       │  ← Geolocation
│    Parse lat/lng/accuracy output    │
└─────────────────┬───────────────────┘
                  │ 
┌─────────────────▼───────────────────┐
│        Telegram BOT message         │  ← Output
└─────────────────────────────────────┘
```

## Used libs 📚

### Standard used: Cargo 1.95.0 (f2d3ce0bd 2026-03-21)

| Use               | Library                      |
|-------------------|------------------------------|
| Async runtime     | tokio                        |
| Activating buzzer | rppal                        |
| HTTP POST         | reqwest                      |
| JSON              | serde_json                   |
| Telegram Bot      | teloxide                     |
| Geolocation       | Google Cloud Geolocation API |

## Project Structure 🗂️

```
GeoSniff/                   # Root folder
├── src/
│   ├── geolocation_api.rs         # Parse output & obtain MACs
│   ├── main.rs                    # Curl POST & geolocalization
│   ├── tg_bot.rs                  # Buzzer alarm with GPIO PINs
│   └── wifi_scan.rs               # All Telegram Bot functions
├── .gitignore
├── Cargo.lock              # Needed for Cargo.toml
├── Cargo.toml              # For project dependency and settings
├── LICENSE                 # GNU GPL3 License
├── config.txt     <-----   # Where you will place your API 
└── README.md               # This file
```

---
<!--
## How to compile and build 🏁

**First** you will ***need*** your Telegram Bot token and Google Cloud Geolocation API, then create a file named `config.txt`
in the root folder as showed in the structure above.

Once you have the file, copy this **template**:

```
GOOGLE_API_KEY=abcd
TELEGRAM_TOKEN=1234
```
then save the file and proced to the compile step.

To **compile** the executable, open terminal and got in the **root folder** (`GeoSniff/`) and run: 
- `mkdir build/` -> Used to create the build directory
- `cd build/` -> Enter the newly created folder
- `cmake ..` -> Generates the build files from CMakeLists.txt
- `make -j4` -> use to compile all files

### And you are done! Now to **execute** just run `./geo_sniff` and the program should start-->
