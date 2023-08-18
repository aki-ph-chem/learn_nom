use nom::{bytes::complete::tag, branch::alt, sequence::terminated, IResult};
use std::error::Error;

fn parse_abc(input: &str) -> IResult<&str, &str> {
    terminated(
        tag("abc"),
        tag("xyz")
    )(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (_x, abc) = parse_abc("abcxyz_BBB")?;
    println!("abc = {}", abc);

    let (_x, abc) = parse_abc("abcxyz")?;
    println!("abc = {}", abc);

    Ok(())
}
