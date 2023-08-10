use nom::{bytes::complete::tag, character::complete::alpha0, IResult};
use std::error::Error;

/// `tag()`は input: &str -> IResult<&str, &str>な関数を返す
fn parse_input(input: &str) -> IResult<&str, &str> {
    tag("abc")(input)
}

/// 大文字、小文字のアルファベット([a-zA-Z])を取り出す
fn parser_alphabet(input: &str) -> IResult<&str, &str> {
    alpha0(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (leftover_input, output) = parse_input("abcworld")?;
    assert_eq!(leftover_input, "world");
    assert_eq!(output, "abc");
    assert!(parse_input("defworld").is_err());

    let (remainig, letters) = parser_alphabet("abc123")?;
    assert_eq!(remainig, "123");
    assert_eq!(letters, "abc");

    Ok(())
}
