use nom::{bytes::complete::tag, branch::alt, sequence::delimited, IResult};
use std::error::Error;

fn parse_braket(input: &str) -> IResult<&str, &str> {
    delimited(
            alt((tag("("), tag("<"), tag("{"))),
            tag("abc"),
            alt((tag(")"), tag(">"), tag("}"))),
    )(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (_x, abc) = parse_braket("(abc)")?;
    println!("abc = {}", abc);

    let (_x, abc) = parse_braket("<abc>")?;
    println!("abc = {}", abc);

    let (_x, abc) = parse_braket("{abc}")?;
    println!("abc = {}", abc);

    Ok(())
}
