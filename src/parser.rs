use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_until};
use nom::character::complete::{char, line_ending, multispace0, not_line_ending};
use nom::combinator::{opt, peek};
use nom::IResult;
use nom::multi::many1;
use nom::number::complete::float;
use nom::sequence::{pair, preceded};

use crate::playlist::{MediaEntry, TagExtInf};

fn parse_key_value(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, key) = take_till(|c: char| c == '=')(input)?;
    let (input, _) = char('=')(input)?;
    let (input, value_with_next_key) = peek(alt((take_until("="), tag(""))))(input)?;

    let value = match value_with_next_key {
        "" => {
            // 说明到末尾了, 没有下一个 key, 所有剩余 input 都是当前 value
            input
        }
        s => {
            // 取 s 的最后一个空格之前的内容作为下一个 key
            let (next_key, _) = take_until(" ")(s)?;
            // input 去掉 next_key 之前的内容, 剩下的就是当前 value
            let (_, input) = take_until(next_key)(input)?;
            input.trim()
        }
    };

    // println!("key: |{}|, value: |{}|", key, value);

    let (input, _) = tag(value)(input)?;

    Ok((input.trim(), (key.trim(), value.trim())))
}


// Parse #EXTINF
// 格式一：（带扩展信息）
// #EXTINF:-1 tvg-id="1" tvg-name="CCTV1" tvg-logo="https://raw.githubusercontent.com/drangjchen/IPTV/main/Logo/CCTV1.png" group-title="央视",CCTV-1
// 格式二：（基础）
// #EXTINF:-1,CCTV1
fn parse_ext_inf(input: &str) -> IResult<&str, TagExtInf> {
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
    match ext.trim() {
        "" => {}
        ext => {
            many1(parse_key_value)(ext)?;
        }
    }

    Ok((input, TagExtInf {
        duration,
        title,
        tvg_id: None,
        tvg_name: None,
        tvg_logo: None,
        group: None,
    }))
}

// Parse one MediaEntry
fn parse_entry(input: &str) -> IResult<&str, MediaEntry> {
    let (input, extinf) = preceded(tag("#EXTINF:"), not_line_ending)(input)?;
    let (_, tag_ext_inf) = parse_ext_inf(extinf)?;
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