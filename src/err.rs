use std::fmt::Display;

pub enum DatabaseError {
    UsernameQueryPrepareError,
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UsernameQueryPrepareError =>
                    "An unknown error occurred while preparing to query the database",
            }
        )
    }
}
