use std::{fs::File, io::Write, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_string_pretty};

use crate::MAIN_CONFIG;

#[derive(Debug, Serialize, Deserialize)]
pub struct MainConfig {
    pub enabled: bool,
    pub sounds: Vec<SoundsConfig>,
    pub times: Vec<TimesConfig>,
    pub groups: Vec<GroupsConfig>,
}

impl Default for MainConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sounds: vec![],
            times: vec![],
            groups: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoundsConfig {
    pub id: i8,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimesConfig {
    pub id: i8,
    pub display: String,
    pub groups_id: Vec<i8>,
    pub sound_id: i8,
    pub hour: u32,
    pub minute: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupsConfig {
    pub id: i8,
    pub display: String,
    pub enabled: bool,
}

pub fn load_config() -> MainConfig {
    let config_path = Path::new("./config.json");
    if !config_path.exists() {
        let mut file = File::create(config_path).unwrap();
        let config = MainConfig::default();
        let json = to_string_pretty(&config).unwrap();
        let buf = json.as_bytes();
        file.write(buf).unwrap();
    };
    let config = File::open(config_path).unwrap();
    let config = from_reader(config);
    if config.is_err() {
        panic!("Config mismatch!")
    };
    return config.unwrap();
}

pub fn check_config() {
    let config = MAIN_CONFIG.read().unwrap();
    let mut ids = vec![];
    for sound in config.sounds.iter() {
        let path = Path::new(&sound.path);
        if ids.contains(&sound.id) {
            panic!("sound {} is double ID-d!", sound.id)
        }
        ids.push(sound.id);
        if !path.exists() {
            panic!("{} is not found", sound.path)
        }
    }
    let mut ids = vec![];
    for time in config.times.iter() {
        if ids.contains(&time.id) {
            panic!("time {} is double ID-d!", time.id)
        }
        ids.push(time.id);
        if time.hour > 24 || time.minute > 60 {
            panic!("{}:{} invalid time", time.hour, time.minute)
        }
    }
    let mut ids = vec![];
    for group in config.groups.iter() {
        if ids.contains(&group.id) {
            panic!("group {} is double ID-d!", group.id)
        }
        ids.push(group.id);
    }
    println!("Config check completed")
}
