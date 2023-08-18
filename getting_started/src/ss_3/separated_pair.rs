use nom::{bytes::complete::tag, branch::alt, sequence::separated_pair, IResult};
use std::error::Error;

fn parse_pair(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        tag("abc"),
        tag(","),
        tag("xyz")
    )(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (_x, (abc, xyz)) = parse_pair("abc,xyz")?;
    println!(" abc = {}\n xyz = {}", abc, xyz);

    Ok(())
}
