use std::fmt::Display;

#[derive(Debug)]
pub enum DatabaseError {
    UnknownUsernameError,
    UsernameQueryPrepareError(String),
    UsernameParseError(String),
    PasswordQueryPrepareError(String),
    PasswordParseError(String, String),
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UnknownUsernameError =>
                    String::from("An unknown error occurred while querying the database"),
                Self::UsernameParseError(err) =>
                    format!("An error occurred while parsing the username: {}", err),
                Self::UsernameQueryPrepareError(err) => format!(
                    "An error occurred while preparing to query the database (username): {}",
                    err
                ),
                Self::PasswordQueryPrepareError(err) => format!(
                    "An error occurred while preparing to query the database (password): {}",
                    err
                ),
                Self::PasswordParseError(usr, err) => format!(
                    "An error occurred while parsing the password for {:?}: {}",
                    usr, err
                ),
            }
        )
    }
}
