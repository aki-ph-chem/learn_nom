use std::error::Error;
use nom::{bytes::complete::tag, branch::alt, sequence::preceded, IResult};

fn parse_abc(input: &str) -> IResult<&str, &str> {
    preceded(tag("abc"), tag("xyz"))(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (_x, abc) = parse_abc("abcxyz_hoge")?;
    println!("abc = {}", abc);

    /* error
    let (_x, abc) = parse_abc("aaaxyz_hoge")?;
    println!("abc = {}", abc);
    */

    Ok(())
}
