use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::is_space;
use nom::sequence::terminated;
use nom::IResult;
use std::error::Error;

fn parse_sentence(input: &str) -> IResult<&str, &str> {
    terminated(take_until("."), take_while(|c| c == '.' || c == ' '))(input)
}

// predicateを使ってパーサーを繰り返す
fn main() -> Result<(), Box<dyn Error>> {
    let (remainig, parsed) = parse_sentence("I am Tom. I write Rust.")?;
    assert_eq!(parsed, "I am Tom");
    assert_eq!(remainig, "I write Rust.");

    let parsing_error = parse_sentence("Not sentence (no period at end)");
    assert!(parsing_error.is_err());

    Ok(())
}
