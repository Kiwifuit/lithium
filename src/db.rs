use std::process::exit;

use crate::err::DatabaseError;
use rusqlite::Connection;

#[derive(Debug)]
pub struct Login {
    username: String,
    password: Option<String>,
}

impl Login {
    pub fn new(username: String) -> Self {
        Self {
            username,
            password: None,
        }
    }

    pub fn update_password(&mut self, password: String) {
        self.password = Some(password)
    }

    pub fn get_username(&self) -> String {
        self.username.to_owned()
    }
}

pub fn query_all_usernames(conn: &Connection) -> Result<Vec<Login>, DatabaseError> {
    let mut query = match conn.prepare("SELECT username_value FROM logins") {
        Ok(q) => q,
        Err(_) => return Err(DatabaseError::UsernameQueryPrepareError),
    };

    let res = query.query_map([], |row| {
        let username = match row.get(0) {
            Ok(u) => u,
            Err(e) => {
                eprintln!("An error occurred while querying the database: {}", e);
                exit(-1);
            }
        };

        Ok(Login::new(username))
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
    let mut query = match conn.prepare("SELECT password_value FROM logins WHERE username_value = ?")
    {
        Ok(q) => q,
        Err(_) => return Err(DatabaseError::UsernameQueryPrepareError),
    };

    match query.query([login.get_username()]) {
        Ok(mut passwd) => {
            // Everything inside counts as a  "Password Parse Error"
            // because making a separate variant is a pain and I don't
            // wanna do that ;-;
            let passwd = passwd.next();

            // Apparently this worked first try :D
            // .....hopefully
            if let Ok(Some(passwd)) = passwd {
                let passwd = match passwd.get(0) {
                    Ok(p) => p,
                    Err(e) => {
                        return Err(DatabaseError::PasswordParseError(
                            login.get_username(),
                            e.to_string(),
                        ))
                    }
                };
                login.update_password(passwd)
            }
        }
        Err(e) => {
            return Err(DatabaseError::PasswordParseError(
                login.get_username(),
                e.to_string(),
            ))
        }
    };

    Ok(())
}
