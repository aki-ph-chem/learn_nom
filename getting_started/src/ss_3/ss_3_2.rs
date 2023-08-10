use nom::{branch::alt, bytes::complete::tag, IResult};
use std::error::Error;

// ２つのパーサーを組み合わせる

// "abc"にマッチングする
fn parse_abc(input: &str) -> IResult<&str, &str> {
    tag("abc")(input)
}

// "def" or "ghi"にマッチングする
fn parse_def_or_ghi(input: &str) -> IResult<&str, &str> {
    alt((tag("def"), tag("ghi")))(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = "abcghi";
    let (remaider, abc) = parse_abc(input)?;
    let (remainder, def_or_ghi) = parse_def_or_ghi(remaider)?;
    println!("first parsed: {abc}; then parsed: {def_or_ghi}");

    Ok(())
}
