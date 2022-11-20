use std::fmt::Display;

pub enum DatabaseError {
    UsernameReadError,
    PasswordReadError,
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "An unknown error occurred while reading the {} value in the database",
            match self {
                DatabaseError::PasswordReadError => "password",
                DatabaseError::UsernameReadError => "username",
            }
        )
    }
}
