use std::error::Error;
use nom::IResult;
use nom::multi::many0;
use nom::bytes::complete::tag;

// パーサーを繰り返すよりもコンビネータの方が便利
// コンビネータ nom::multi::many0を使う

// parser
fn parser(s: &str) -> IResult<&str, Vec<&str>> {
    many0(tag("abc"))(s)
}

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
    assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
    assert_eq!(parser("123123"), Ok(("123123", vec![])));
    assert_eq!(parser(""), Ok(("", vec![])));

    Ok(())
}
