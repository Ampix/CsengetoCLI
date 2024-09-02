use rusqlite::Connection;

fn main() {
    let conn= Connection::open("./db.db3").unwrap();
    
    setup_db(conn);

    println!("Hello, világ!");
}

fn setup_db(conn:Connection){

    conn.execute("CREATE TABLE IF NOT EXISTS csengo (id INTEGER PRIMARY KEY, time TEXT NOT NULL, path TEXT NOT NULL)", ()).expect("Nem sikerült létrehozni a táblát");

}


