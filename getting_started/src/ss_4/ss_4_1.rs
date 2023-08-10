use nom::{IResult, bytes::complete::tag, combinator::value, branch::alt };
use std::error::Error;

// valueコンビネータを使って型変換を行う
// ErrorハンドリングのチャプターまではRustに組み込みのパーサーを使うことは避けよう
// なぜなら、nomと一緒に使うのは特別なハンドリングが必要であるからである


// boolに変換する
fn parse_bool(input: &str) -> IResult<&str, bool> {
    alt((
            value(true, tag("true")),
            value(false, tag("false")),
    ))(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (remaining, parsed) = parse_bool("true|false")?;
    assert_eq!(parsed, true);
    assert_eq!(remaining, "|false");

    let parsing_error = parse_bool(remaining);
    assert!(parsing_error.is_err());

    // 先頭の'|'を除いてからパース
    let (remaining, parsed) = parse_bool(&remaining[1..])?;
    assert_eq!(parsed, false);
    assert_eq!(remaining, "");

    Ok(())
}
