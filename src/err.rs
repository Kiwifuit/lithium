use std::fmt::Display;

pub enum DatabaseError {
    UsernameQueryPrepareError,
    UnknownUsernameError,
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
                Self::UsernameParseError(err) =>
                    format!("An error occurred while parsing the username: {}", err),
                Self::UnknownUsernameError =>
                    String::from("An unknown error occurred while querying the database"),
                Self::UsernameQueryPrepareError =>
                    String::from("An unknown error occurred while preparing to query the database"),
                Self::PasswordQueryPrepareError(err) => format!(
                    "An error occurred while preparing to query the database: {} (password)",
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
