struct SignInAuth {
    email: String,
    password: String,
}

struct SignUpAuth {
    credentials: SignInAuth,
    username: String,
    phone_number: i64,
}

impl SignInAuth {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
    pub fn sign_in() {}
}

impl SignUpAuth {
    pub fn new(email: String, password: String, username: String, phone_number: i64) -> Self {
        Self {
            credentials: SignInAuth::new(email, password),
            username,
            phone_number,
        }
    }
    pub fn sign_up() {}
}
