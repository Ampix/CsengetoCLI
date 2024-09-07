use chrono::{Local, Timelike};

use rodio::{Decoder, OutputStream, Sink};
use rusqlite::Connection;
use std::fs::File;
use std::io::BufReader;

use std::ops::Index;
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

    let mut hangok = get_data(&conn);
    conn.close().expect("Valahogy nem sikerült bezárni");
    for hang in &hangok {
        println!("óra{} perc{} ", hang.hour, hang.minute,)
    }
    hangok = buble_sort(hangok.clone());
    for hang in &hangok {
        println!("óra{} perc{}", hang.hour, hang.minute)
    }
    let condition = true;

    while condition {
        let now = Local::now().time();

        for hang in hangok.clone() {
            println!("Mostan {} óra {} perc van", now.hour(), now.minute());
            println!("Zene {} óra {} perckor lesz ", hang.hour, hang.minute);

            if hang.hour == now.hour().try_into().unwrap()
                && hang.minute == now.minute().try_into().unwrap()
                && hang.status == 1
            {
                println!("idő van");

                play_mp3(&hang.path)
            }
        }
        println!("Ellenőrizve");

        thread::sleep(Duration::from_secs(10));
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
    for hang in &output {
        println!("{}", hang.hour);
    }
    for i in 0..length {
        let mut swapped = false;

        for j in 0..length - i - 1 {
            if compare_hangok(&output[j], &output[j + 1]) {
                swapped = true;
                println!("swapped");
                let temp = output[j].clone();
                output[j] = output[j + 1].clone();
                output[j + 1] = temp.clone();
                for hang in &output {
                    println!("{}", hang.hour);
                }
            }
        }
        if swapped == false {
            break;
        }
        for hang in &input {
            println!("{}", hang.hour);
        }
    }
    output
}
fn compare_hangok(hang_1: &Hangok, hang_2: &Hangok) -> bool {
    if hang_1.hour > hang_2.hour {
        true
    } else if hang_2.hour == hang_1.hour {
        if hang_1.minute > hang_2.minute {
            true
        } else {
            false
        }
    } else {
        false
    }
}
