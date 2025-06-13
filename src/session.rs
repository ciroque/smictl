pub struct Session {
    pub provider: Option<String>,
    pub model: Option<String>,
    pub document: Option<String>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            provider: None,
            model: None,
            document: None,
        }
    }
}