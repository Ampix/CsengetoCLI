use rusqlite::Connection;
use std::io;

fn main() {
    let conn= Connection::open("./db.db3").unwrap();
    
    setup_db(conn);
    let mut condition :bool = true;
    while condition {
        println!("Hello, világ!");
        let mut input_string  = String::new();
        io::stdin().read_line(&mut input_string).expect("Failed to read line");
        println!("Input {}",input_string);

        condition = false;
    }
    println!("Hello, világ!");

}

fn setup_db(conn:Connection){

    conn.execute("CREATE TABLE IF NOT EXISTS csengo (id INTEGER PRIMARY KEY, time TEXT NOT NULL, path TEXT NOT NULL)", ()).expect("Nem sikerült létrehozni a táblát");

}


