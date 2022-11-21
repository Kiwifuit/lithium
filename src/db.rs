use std::process::exit;

use crate::err::DatabaseError;
use rusqlite::{Connection, DatabaseName};
use std::io::prelude::*;

const PASSWORD_BUFFER_LIMIT: usize = 64;

#[derive(Debug)]
pub struct Login {
    username: String,
    password: Option<Vec<u8>>,
    rowid: i64,
}

impl Login {
    pub fn new(username: String, rowid: i64) -> Self {
        Self {
            username,
            password: None,
            rowid,
        }
    }

    pub fn update_password(&mut self, password: Vec<u8>) {
        self.password = Some(password)
    }

    pub fn get_username(&self) -> String {
        self.username.to_owned()
    }
}

pub fn query_all_usernames(conn: &Connection) -> Result<Vec<Login>, DatabaseError> {
    let mut query = match conn.prepare("SELECT username_value FROM logins") {
        Ok(q) => q,
        Err(e) => return Err(DatabaseError::UsernameQueryPrepareError(e.to_string())),
    };

    let mut row_id = -1;
    let res = query.query_map([], |row| {
        let username = match row.get(0) {
            Ok(u) => u,
            Err(e) => {
                eprintln!("An error occurred while querying the database: {}", e);
                exit(-1);
            }
        };

        row_id += 1;

        while test_row_id(&conn, &row_id).is_err() {
            row_id += 1;
        }

        Ok(Login::new(username, row_id))
    });

    if let Ok(rows) = res {
        let mut res = vec![];

        for row in rows {
            if let Ok(login) = row {
                res.push(login)
            } else {
                return Err(DatabaseError::UsernameParseError(
                    row.unwrap_err().to_string(),
                ));
            }
        }

        return Ok(res);
    } else {
        return Err(DatabaseError::UnknownUsernameError);
    }
}

pub fn query_password_for(conn: &Connection, login: &mut Login) -> Result<(), DatabaseError> {
    let mut buffer = vec![0u8; PASSWORD_BUFFER_LIMIT];
    let mut raw = match conn.blob_open(
        DatabaseName::Main,
        "logins",
        "password_value",
        login.rowid,
        true,
    ) {
        Ok(p) => p,
        Err(e) => return Err(DatabaseError::PasswordReadError(e.to_string())),
    };

    if let Err(err) = raw.read(&mut buffer) {
        return Err(DatabaseError::PasswordReadError(err.to_string()));
    }

    login.update_password(buffer);

    Ok(())
}

fn test_row_id(conn: &Connection, id: &i64) -> Result<(), ()> {
    match conn.blob_open(DatabaseName::Main, "logins", "password_value", *id, true) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}
