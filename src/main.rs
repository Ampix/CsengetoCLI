use checker::watch_config_file;
use chrono::{Local, Timelike};

use config::{check_config, load_config, MainConfig};
use lazy_static::lazy_static;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

use std::sync::RwLock;
use std::thread;
use std::time::Duration;

lazy_static! {
    pub static ref MAIN_CONFIG: RwLock<MainConfig> = RwLock::new(load_config());
}

mod checker;
mod config;

#[derive(Debug, Clone)]
struct Hangok {
    id: i8,
    hour: u32,
    minute: u32,
    sound_id: i8,
    display: String,
    group: i8,
}
fn main() {
    check_config();
    watch_config_file();
    let hangok = buble_sort(get_data());
    if hangok.clone().len() == 0 {
        panic!("Nincs hang!")
    }
    println!("Betöltött idők: ");
    for hang in &hangok {
        println!(
            "{}, {} óra {} perc, szól: {}",
            hang.display,
            hang.hour,
            hang.minute,
            group_is_enabled(hang.group)
        );
    }
    loop {
        let hangok_copy = hangok.clone();
        let now = Local::now();
        for hangf in hangok_copy {
            if group_is_enabled(hangf.group) {
                if compare_hour_time(
                    now.hour().try_into().unwrap(),
                    now.minute().try_into().unwrap(),
                    hangf.hour,
                    hangf.minute,
                ) {
                    let index = hangok
                        .clone()
                        .iter()
                        .position(|x| &x.id == &hangf.id)
                        .unwrap();
                    println!("Index: {}", index);
                    if index == hangok.len() - 1 {
                        check(hangok.clone()[0].clone());
                    }
                } else {
                    check(hangf);
                }
            }
        }
        println!("Lejátszási lista vége, 60 sec pihi!");
        thread::sleep(Duration::from_secs(60));
    }
}

fn check(hang: Hangok) {
    let now = Local::now();
    println!("Most {:?} óra {:?} perc", now.hour(), now.minute());
    println!("Következő {:?} óra {:?} perc", hang.hour, hang.minute);
    if hang.hour == now.hour() && hang.minute == now.minute() {
        println!("Szóljon a zene!");
        play_sound(&get_sound_by_id(hang.sound_id));
        println!("Zene vége!");
    } else {
        let next_in_seconds = hang.hour * 3600 + hang.minute * 60;
        let now_in_second: i32 = (now.hour() * 3600 + now.minute() * 60 + now.second())
            .try_into()
            .unwrap();

        let mut wait_time: i32 =
            <u32 as TryInto<i32>>::try_into(next_in_seconds).unwrap() - now_in_second;

        if wait_time <= 0 {
            wait_time = (24 * 60 * 60) + wait_time;
        }

        println!("Várunk {} másodpercet", wait_time);
        thread::sleep(Duration::from_secs(wait_time.try_into().unwrap()));
        check(hang);
    }
}

fn get_sound_by_id(id: i8) -> String {
    let config = MAIN_CONFIG.read().unwrap();
    let sound = config.sounds.iter().find(|p| p.id == id);
    return sound.unwrap().path.clone();
}

fn group_is_enabled(id: i8) -> bool {
    let config = MAIN_CONFIG.read().unwrap();
    let config = config.groups.iter().find(|p| p.id == id);
    return config.unwrap().enabled;
}

fn get_data() -> Vec<Hangok> {
    let config = MAIN_CONFIG.read().unwrap();
    let hangs = config
        .times
        .iter()
        .map(|hang| -> Hangok {
            Hangok {
                id: hang.id,
                hour: hang.hour,
                minute: hang.minute,
                sound_id: hang.sound_id,
                group: hang.group_id,
                display: hang.display.clone(),
            }
        })
        .collect();
    return hangs;
}

fn play_sound(file_path: &String) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = File::open(file_path.as_str()).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);

    sink.sleep_until_end();
    sink.clear();
}

fn buble_sort(input: Vec<Hangok>) -> Vec<Hangok> {
    let mut output = input.clone();
    let length = output.len();

    for i in 0..length {
        let mut swapped = false;

        for j in 0..length - i - 1 {
            if compare_hour_time(
                output[j].hour,
                output[j].minute,
                output[j + 1].hour,
                output[j + 1].minute,
            ) {
                swapped = true;

                let temp = output[j].clone();
                output[j] = output[j + 1].clone();
                output[j + 1] = temp.clone();
            }
        }
        if swapped == false {
            break;
        }
    }
    output
}

fn compare_hour_time(
    bigger_hour: u32,
    bigger_minute: u32,
    smaller_hour: u32,
    smaller_minute: u32,
) -> bool {
    if bigger_hour > smaller_hour {
        true
    } else if smaller_hour == bigger_hour {
        if bigger_minute > smaller_minute {
            true
        } else {
            false
        }
    } else {
        false
    }
}
