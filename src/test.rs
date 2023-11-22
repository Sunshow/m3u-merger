#[cfg(test)]
mod tests {
    use m3u8_rs::Playlist;
    use crate::fetch;

    #[tokio::test]
    async fn test_fetch_url() {
        let resp = fetch::fetch_text("https://raw.githubusercontent.com/YueChan/Live/main/IPTV.m3u").await.unwrap();

        println!("resp: {}", resp);
    }

    #[tokio::test]
    async fn test_parse_m3u() {
        let resp = fetch::fetch_text("https://raw.githubusercontent.com/YueChan/Live/main/IPTV.m3u").await.unwrap();

        match m3u8_rs::parse_playlist_res(resp.as_bytes()) {
            Ok(Playlist::MasterPlaylist(pl)) => println!("Master playlist:\n{:?}", pl),
            Ok(Playlist::MediaPlaylist(pl)) => {
                println!("MediaPlaylist");
                for segment in pl.segments {
                    println!("segment: {:?}", segment);
                }
            },
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}