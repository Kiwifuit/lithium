mod db;
mod err;
mod path;

use rusqlite::Connection;

fn main() {
    let conn = Connection::open("./data/Profile 1/Login Data").unwrap();

    for mut username in db::query_all_usernames(&conn).unwrap() {
        if let Err(e) = db::query_password_for(&conn, &mut username) {
            println!("An error occurred while querying the database:\n\t{}", e);
            continue;
        }

        println!("{:?}", username);
    }
}
