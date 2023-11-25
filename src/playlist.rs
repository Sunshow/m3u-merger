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

    /// tvg-id="8" tvg-name="CCTV7" tvg-logo="https://raw.githubusercontent.com/drangjchen/IPTV/main/Logo/CCTV7.png" group-title="央视",CCTV-7
    pub tvg_id: Option<String>,
    pub tvg_name: Option<String>,
    pub tvg_logo: Option<String>,
    /// group-title="央视",CCTV-7
    /// 拆成 group 和 title 分别存储, 其中 title 复用上面的标准字段
    pub group: Option<String>,
}