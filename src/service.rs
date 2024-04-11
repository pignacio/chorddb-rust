pub(crate) trait Songs: Send {
    fn get_songs(&self) -> Vec<String>;
}

#[derive(Debug, Clone)]
pub(crate) struct FileSongs {
    path: String,
    songs: Vec<String>,
}

impl FileSongs {
    pub(crate) fn new<S: AsRef<str>>(path: S) -> Self {
        let path = path.as_ref().to_string();

        FileSongs {
            path,
            songs: vec!["Some Title".to_string(), "Another Title".to_string()],
        }
    }
}

impl Songs for FileSongs {
    fn get_songs(&self) -> Vec<String> {
        leptos::logging::log!("Loading songs from {}", self.path);
        let contents = std::fs::read_to_string(&self.path).unwrap();
        serde_json::from_str(&contents).unwrap()
    }
}
