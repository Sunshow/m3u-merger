#[derive(Debug, Default, PartialEq, Clone)]
pub struct MediaEntry {
    pub uri: String,
    /// `#EXTINF:<duration>,[<title>]`
    pub duration: f32,
    /// `#EXTINF:<duration>,[<title>]`
    pub title: Option<String>,
}
