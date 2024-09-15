use chrono::{Local, Timelike};

use rodio::{Decoder, OutputStream, Sink};
use rusqlite::Connection;
use std::fs::File;
use std::io::BufReader;

use std::thread;
use std::time::Duration;
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
    conn.close().expect("Valahogy nem sikerült bezárni");
    for hang in &hangok {
        println!("{}", hang.hour);
    }
    let mut condition = true;

    let now = Local::now().time();

    let mut next_hang = hangok[0].clone();
    let mut next_hang_index = 0;
    for hang_index in 0..hangok.len() {
        if compare_hour_time(
            hangok[hang_index].hour,
            hangok[hang_index].minute,
            now.hour().try_into().unwrap(),
            now.minute().try_into().unwrap(),
        ) && compare_hour_time(
            now.hour().try_into().unwrap(),
            now.minute().try_into().unwrap(),
            next_hang.hour,
            next_hang.minute,
        ) {
            next_hang_index = hang_index;
            next_hang = hangok[next_hang_index].clone();
        }
    }

    println!("hour {} minute {}", next_hang.hour, next_hang.minute);
    let mut wait_time = get_time_difference(
        next_hang.hour,
        next_hang.minute,
        0,
        now.hour().try_into().unwrap(),
        now.minute().try_into().unwrap(),
        now.second().try_into().unwrap(),
    ) - 10;

    println!("{}", wait_time);
    thread::sleep(Duration::from_secs(wait_time.try_into().unwrap()));
    while condition {
        let now = Local::now().time();

        println!("Mostan {} óra {} perc van", now.hour(), now.minute());
        println!(
            "Zene {} óra {} perckor lesz ",
            next_hang.hour, next_hang.minute
        );

        if next_hang.hour == now.hour().try_into().unwrap()
            && next_hang.minute == now.minute().try_into().unwrap()
            && next_hang.status == 1
        {
            println!("idő van");
            play_mp3(&next_hang.path);
            next_hang_index += 1;
            next_hang = hangok[next_hang_index].clone();
            wait_time = get_time_difference(
                next_hang.hour,
                next_hang.minute,
                0,
                now.hour().try_into().unwrap(),
                now.minute().try_into().unwrap(),
                now.second().try_into().unwrap(),
            ) - 10;
            thread::sleep(Duration::from_secs(wait_time.try_into().unwrap()));
        }

        println!("Ellenőrizve");

        thread::sleep(Duration::from_secs(1));
    }
}

fn setup_db(conn: &Connection) {
    conn.execute("CREATE TABLE IF NOT EXISTS csengo (id INTEGER PRIMARY KEY, time TEXT NOT NULL, path TEXT NOT NULL,status BOOLEAN DEFAULT 1 NOT NULL   )", ()).expect("Nem sikerült létrehozni a táblát");
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
        hangok_lista.push(hang.unwrap());
    }

    hangok_lista
}

fn play_mp3(file_path: &String) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = File::open(file_path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);

    sink.sleep_until_end();
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

fn get_time_difference(
    bigger_hour: i32,
    bigger_minute: i32,
    bigger_second: i32,
    smaller_hour: i32,
    smaller_minute: i32,
    smaller_second: i32,
) -> i32 {
    let mut hour_difference = bigger_hour - smaller_hour;
    let mut minute_difference = bigger_minute - smaller_minute;
    let second_difference = bigger_second - smaller_second;
    if second_difference == 0 {
        minute_difference = -1;
    }
    if minute_difference == 0 {
        hour_difference = -1;
    }
    return ((hour_difference * 60 + minute_difference) * 60) + second_difference;
}
