use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, not_line_ending};
use nom::IResult;
use nom::multi::many1;
use nom::sequence::preceded;

use crate::playlist::MediaEntry;

// Parse #EXTINF
// 格式一：（带扩展信息）
// #EXTINF:-1 tvg-id="1" tvg-name="CCTV1" tvg-logo="https://raw.githubusercontent.com/drangjchen/IPTV/main/Logo/CCTV1.png" group-title="央视",CCTV-1
// 格式二：（基础）
// #EXTINF:-1, CCTV1

// Parse one MediaEntry
fn parse_entry(input: &str) -> IResult<&str, MediaEntry> {
    let (input, extinf) = preceded(tag("#EXTINF:"), not_line_ending)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, uri) = not_line_ending(input)?;
    let (input, _) = line_ending(input)?;
    // println!("input: {}", input);
    Ok((input, MediaEntry {
        uri: uri.to_string(),
        duration: 0.0,
        title: Some("".to_string()),
    }))
}

pub fn parse_playlist(input: &str) -> IResult<&str, Vec<MediaEntry>> {
    let (input, _) = preceded(tag("#EXTM3U"), not_line_ending)(input)?;
    let (input, _) = line_ending(input)?;
    many1(parse_entry)(input)
}