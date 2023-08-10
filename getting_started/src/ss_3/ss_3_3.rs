use nom::{branch::alt, bytes::complete::tag_no_case, IResult, 
    //sequence::tuple, character::complete::{digit1} };
    sequence::tuple};
use std::error::Error;

fn parse_base(input: &str) -> IResult<&str, &str> {
    alt((
            tag_no_case("a"),
            tag_no_case("t"),
            tag_no_case("c"),
            tag_no_case("g")
    ))(input)
}

fn parse_pair(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((
            parse_base,
            parse_base
    ))(input)
}


fn main() -> Result<(), Box<dyn Error>> {
    let (remainig, parsed) = parse_pair("aTcG")?;
    assert_eq!(parsed, ("a", "T"));
    assert_eq!(remainig, "cG");

    Ok(())
}
