pub struct Configuration {
    pub base_path: String,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            base_path: String::from("https://www.strava.com/api/v3"),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}
