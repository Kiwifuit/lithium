use std::process::exit;

mod db;
mod err;
mod path;

fn main() {
    let encryption_key = match path::get_encryption_key() {
        Ok(k) => k,
        Err(e) => {
            eprintln!("An error occurred while getting the encryption key: {}", e);
            exit(10)
        }
    };

    let profiles = match path::get_all_profiles() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("An error occurred while getting profiles: {}", e);
            exit(20)
        }
    };

    for database_location in profiles {
        let db = match db::connect_to(database_location.join("/Login Data")) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                exit(22);
            }
        };

        let logins = match db::query_all_usernames(&db) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{}", e);
                exit(22);
            }
        };

        for mut login in logins {
            match db::query_password_for(&db, &mut login) {
                Err(e) => eprintln!("{}", e),
                Ok(()) => (),
            };
        }
    }
}
