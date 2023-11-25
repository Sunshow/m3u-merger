#[derive(Debug, Default, PartialEq, Clone)]
pub struct MediaEntry {
    pub uri: String,

    pub tag_ext_inf: TagExtInf,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TagExtInf {
    /// `#EXTINF:<duration>,[<title>]`
    pub duration: f32,
    /// `#EXTINF:<duration>,[<title>]`
    pub title: Option<String>,
}