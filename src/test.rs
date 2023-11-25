#[cfg(test)]
mod tests {
    use m3u8_rs::Playlist;

    use crate::{fetch, parser};

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
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_parse_playlist_remote() {
        let resp = fetch::fetch_text("https://raw.githubusercontent.com/YueChan/Live/main/IPTV.m3u").await.unwrap();

        let (_, entries) = parser::parse_playlist(&resp).unwrap();
        println!("entries: {:?}", entries);
    }

    #[tokio::test]
    async fn test_parse_playlist() {
        let content = r#"#EXTM3U
#EXTINF:-1
http://[2409:8087:1e03:21::42]:6610/cms001/ch00000090990000001022/index.m3u8?
#EXTINF:-1,CCTV1
http://[2409:8087:1e03:21::42]:6610/cms001/ch00000090990000001022/index.m3u8?
#EXTINF: -1,CCTV1
http://[2409:8087:1e03:21::42]:6610/cms001/ch00000090990000001022/index.m3u8?
#EXTINF:-1 group-title="央视",CCTV-7
http://[2409:8087:2001:20:2800:0:df6e:eb26]/wh7f454c46tw3984282630_1427246842/ott.mobaibox.com/PLTV/3/224/3221228283/index.m3u8?icpid=3&RTS=1668594483&from=40&popid=40&hms_devid=2293&prioritypopid=40&vqe=3
#EXTINF:-1 tvg-id="9" tvg-name="CCTV8" tvg-logo="https://raw.githubusercontent.com/drangjchen/IPTV/main/Logo/CCTV8.png" group-title="央视",CCTV-8
http://[2409:8087:2001:20:2800:0:df6e:eb26]/ott.mobaibox.com/PLTV/4/224/3221228578/index.m3u8
#EXTINF:-1 tvg-id="9" tvg-name="CCTV8" tvg-logo="https://raw.githubusercontent.com/drangjchen/IPTV/main/Logo/CCTV8.png",CCTV-8
http://[2409:8087:2001:20:2800:0:df6e:eb26]/ott.mobaibox.com/PLTV/4/224/3221228578/index.m3u8
#EXTINF:-1 tvg-id="9" tvg-name="CCTV8" tvg-logo="https://raw.githubusercontent.com/drangjchen/IPTV/main/Logo/CCTV8.png" group-title="央视", CCTV-8 少儿频道
http://[2409:8087:2001:20:2800:0:df6e:eb26]/ott.mobaibox.com/PLTV/4/224/3221228578/index.m3u8
#EXTINF:-1 tvg-id="10" tvg-name="CCTV9" tvg-logo="https://raw.githubusercontent.com/drangjchen/IPTV/main/Logo/CCTV9.png" group-title="央视",CCTV-9
http://[2409:8087:2001:20:2800:0:df6e:eb21]/wh7f454c46tw4254168827_1850088835/ott.mobaibox.com/PLTV/3/224/3221228303/index.m3u8?icpid=3&RTS=1668594753&from=40&popid=40&hms_devid=2290&prioritypopid=40&vqe=3
#EXTINF:-1, CCTV1
http://[2409:8087:2001:20:2800:0:df6e:eb12]/wh7f454c46tw3589111099_-1793408755/ott.mobaibox.com/PLTV/3/224/3221227543/index.m3u8?icpid=3&RTS=1668594088&from=40&popid=40&hms_devid=2112&prioritypopid=40&vqe=3"#;

        let (_, entries) = parser::parse_playlist(content).unwrap();
        println!("entries: {:?}", entries);
    }
}