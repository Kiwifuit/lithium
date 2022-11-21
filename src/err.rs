use std::fmt::Display;

#[derive(Debug)]
pub enum DatabaseError {
    UnknownUsernameError,
    UsernameQueryPrepareError(String),
    UsernameParseError(String),
    PasswordReadError(String),
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
                    "An error occurred while preparing to query the database: {}",
                    err
                ),
                Self::PasswordReadError(err) =>
                    format!("An error occurred while reading the password: {}", err),
            }
        )
    }
}

#[derive(Debug)]
pub enum PathError {
    GlobError(String),
    SearchError(String),
}

impl Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::GlobError(err) =>
                    format!("An error occurred while creating the globber: {}", err),
                Self::SearchError(err) =>
                    format!("An error occurred while searching for profiles: {}", err),
            }
        )
    }
}
