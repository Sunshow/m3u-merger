use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{char, line_ending, multispace0, not_line_ending};
use nom::combinator::opt;
use nom::IResult;
use nom::multi::many1;
use nom::number::complete::float;
use nom::sequence::{pair, preceded};

use crate::playlist::{MediaEntry, TagExtInf};

// Parse #EXTINF
// 格式一：（带扩展信息）
// #EXTINF:-1 tvg-id="1" tvg-name="CCTV1" tvg-logo="https://raw.githubusercontent.com/drangjchen/IPTV/main/Logo/CCTV1.png" group-title="央视",CCTV-1
// 格式二：（基础）
// #EXTINF:-1,CCTV1
fn parse_extinf(input: &str) -> IResult<&str, TagExtInf> {
    // duration
    let (input, (_, duration)) = pair(multispace0, float)(input)?;
    // comma
    let (input, (_, comma)) = pair(multispace0, opt(char(',')))(input)?;
    let title: Option<String>;

    // 自定义扩展信息
    let ext = match comma {
        Some(_) => {
            // title
            let (_, (input, t)) = pair(multispace0, take_till(|c| c == ' '))(input)?;
            title = Some(t.to_string());
            let (_, (_, input)) = pair(multispace0, not_line_ending)(input)?;
            input
        }
        None => {
            title = None;
            let (_, (_, input)) = pair(multispace0, not_line_ending)(input)?;
            input
        }
    };
    println!("ext: {}", ext);

    Ok((input, TagExtInf {
        duration,
        title,
    }))
}

// Parse one MediaEntry
fn parse_entry(input: &str) -> IResult<&str, MediaEntry> {
    let (input, extinf) = preceded(tag("#EXTINF:"), not_line_ending)(input)?;
    println!("extinf: {}", extinf);
    let (_, tag_ext_inf) = parse_extinf(extinf)?;
    let (input, _) = line_ending(input)?;
    let (input, uri) = not_line_ending(input)?;
    let (input, _) = line_ending(input)?;
    // println!("input: {}", input);
    Ok((input, MediaEntry {
        uri: uri.to_string(),
        tag_ext_inf,
    }))
}

pub fn parse_playlist(input: &str) -> IResult<&str, Vec<MediaEntry>> {
    let (input, _) = preceded(tag("#EXTM3U"), not_line_ending)(input)?;
    let (input, _) = line_ending(input)?;
    many1(parse_entry)(input)
}