pub struct Trackers {
    pub url: String,
    pub tracker: String,
}

impl Trackers {
    pub fn youtube_trackers(&self) -> String {
        if let Some((clean_url, _tracker)) = self.url.split_once("?si=") {
            clean_url.to_string()
        } else {
            self.url.clone()
        }
    }
}
