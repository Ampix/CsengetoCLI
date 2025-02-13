use std::{fs::File, io::Write, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_string_pretty};

#[derive(Debug, Serialize, Deserialize)]
pub struct MainConfig {
    pub enabled: bool,
    pub sounds: Vec<SoundsConfig>,
    pub times: Vec<TimesConfig>,
}

impl Default for MainConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sounds: vec![],
            times: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoundsConfig {
    pub id: i8,
    pub relative_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimesConfig {
    pub id: i8,
    pub enabled: bool,
    pub sound_id: i8,
    pub hour: i8,
    pub minute: i8,
}

fn load_config() -> MainConfig {
    let config_path = Path::new("./config.json");
    if !config_path.exists() {
        let mut file = File::create(config_path).unwrap();
        let config = MainConfig::default();
        let json = to_string_pretty(&config).unwrap();
        let buf = json.as_bytes();
        file.write(buf).unwrap();
    }
    let config = File::open(config_path).unwrap();
    let config = from_reader(config);
    if config.is_err() {
        panic!("Config mismatch!")
    }
    return config.unwrap();
}

fn check_config(config: &MainConfig) {
    for sound in config.sounds.iter() {
        let path = Path::new(&sound.relative_path);
        if !path.exists() {
            panic!("{} is not found", sound.relative_path)
        }
    }
    for time in config.times.iter() {
        if time.hour > 24 || time.minute > 60 || time.hour < 1 || time.minute < 1 {
            panic!("{}:{} invalid time", time.hour, time.minute)
        }
    }
    println!("Config check completed")
}

pub fn load_and_check_config() -> MainConfig {
    let config = load_config();
    check_config(&config);
    config
}
