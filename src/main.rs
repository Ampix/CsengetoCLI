use rusqlite::Connection;

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

    for hang in hangok {
        println!(
            "id:{} time:{} path:{} status:{}",
            hang.id, hang.time, hang.path, hang.status
        )
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
