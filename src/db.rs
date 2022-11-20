pub struct Login {
    username: String,
    password: Option<String>,
}

impl Login {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
            password: None,
        }
    }

    pub fn update_password(&mut self, password: &str) {
        self.password = Some(password.to_string())
    }
}
