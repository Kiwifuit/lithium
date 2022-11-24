use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum DatabaseError {
    UnknownUsernameError,
    UsernameQueryPrepareError(String),
    UsernameParseError(String),
    PasswordReadError(String),
    ConnectionError(String, String),
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
                Self::ConnectionError(prof, err) => format!(
                    "An error occurred while connecting to the database on {:?}: {}",
                    prof, err
                ),
            }
        )
    }
}

#[derive(Debug)]
pub enum PathError {
    GlobError(String),
    SearchError(String),
    LocalStateDoesNotExist(PathBuf),
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
                Self::LocalStateDoesNotExist(path) => format!(
                    "Local State file does not exist on {}",
                    path.as_path().display()
                ),
            }
        )
    }
}
