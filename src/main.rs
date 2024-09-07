use chrono::{Local, Timelike};
use rodio::{Decoder, OutputStream, Sink};
use rusqlite::Connection;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Hangok {
    id: i32,
    time: String,
    path: String,
    status: i8,
}
fn main() {
    let conn = Connection::open("./db.db3").unwrap();

    setup_db(&conn);

    let hangok = get_data(&conn);
    conn.close().expect("Valahogy nem sikerült bezárni");

    let mut condition = true;
    while condition {
        let now = Local::now().time();

        for hang in &hangok {
            let mut time_vector: Vec<&str> = hang.time.split(":").collect();

            if let Some(stripped) = time_vector[0].strip_prefix("0") {
                time_vector[0] = stripped;
            }

            if let Some(stripped) = time_vector[1].strip_prefix("0") {
                time_vector[1] = stripped;
            }

            println!("Mostan {} óra {} perc van", now.hour(), now.minute());
            println!(
                "Zene {} óra {} perckor lesz ",
                time_vector[0], time_vector[1]
            );

            if time_vector[0] == format!("{}", now.hour())
                && time_vector[1] == format!("{}", now.minute())
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
            Ok(Hangok {
                id: row.get(0)?,
                time: row.get(1)?,
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
