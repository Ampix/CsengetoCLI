use chrono::{Local, Timelike};

use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

use std::thread;
use std::time::Duration;

mod config;

#[derive(Debug, Clone)]
struct Hangok {
    id: i32,
    hour: i32,
    minute: i32,
    path: String,
    status: i8,
}
fn main() {
    let conn = Connection::open("./db.db3").unwrap();

    setup_db(&conn);

    let hangok = buble_sort(get_data(&conn));
    if hangok.clone().len() == 0 {
        panic!("Nincs hang!")
    }
    conn.close().expect("Valahogy nem sikerült bezárni");
    println!("Betöltött hangfájlok: ");
    for hang in &hangok {
        println!("{}, {} óra {} perc", hang.path, hang.hour, hang.minute);
    }
    loop {
        let hangok_copy = hangok.clone();
        let now = Local::now();

        for hangf in hangok_copy {
            if hangf.status == 1 {
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
    if hang.hour == now.hour().try_into().unwrap()
        && hang.minute == now.minute().try_into().unwrap()
    {
        println!("Szóljon a zene!");
        play_sound(&hang.path);
        println!("Zene vége!");
    } else {
        let next_in_seconds = hang.hour * 3600 + hang.minute * 60;
        let now_in_second: i32 = (now.hour() * 3600 + now.minute() * 60 + now.second())
            .try_into()
            .unwrap();

        let mut wait_time = next_in_seconds - now_in_second;

        if wait_time <= 0 {
            wait_time = (24 * 60 * 60) + wait_time;
        }

        println!("Várunk {} másodpercet", wait_time);
        thread::sleep(Duration::from_secs(wait_time.try_into().unwrap()));
        check(hang);
    }
}

fn setup_db(conn: &Connection) {
    conn.execute("CREATE TABLE IF NOT EXISTS csengo (id INTEGER PRIMARY KEY, time TEXT NOT NULL, path TEXT NOT NULL,status BOOLEAN DEFAULT 1 NOT NULL)", ()).expect("Nem sikerült létrehozni a táblát");
}

fn get_data(conn: &Connection) -> Vec<Hangok> {
    let mut datas = conn
        .prepare("SELECT * FROM csengo")
        .expect("Nem sikerült lekérni az adatokat");
    let hangok = datas
        .query_map([], |row| {
            let time: String = row.get(1)?;
            let mut time_vector: Vec<&str> = time.split(':').collect();

            if let Some(stripped) = time_vector[0].strip_prefix("0") {
                time_vector[0] = stripped;
            }

            if let Some(stripped) = time_vector[1].strip_prefix("0") {
                time_vector[1] = stripped;
            }
            Ok(Hangok {
                id: row.get(0)?,
                hour: time_vector[0].to_string().parse::<i32>().unwrap(),
                minute: time_vector[1].to_string().parse::<i32>().unwrap(),
                path: row.get(2)?,
                status: row.get(3)?,
            })
        })
        .unwrap();

    let mut hangok_lista = vec![];

    for hang in hangok {
        if let Ok(hang_value) = hang {
            if hang_value.status == 1 {
                hangok_lista.push(hang_value);
            }
        }
    }

    hangok_lista
}

fn play_sound(file_path: &String) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = File::open(String::from("sounds/") + file_path.as_str()).unwrap();
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
    bigger_hour: i32,
    bigger_minute: i32,
    smaller_hour: i32,
    smaller_minute: i32,
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
