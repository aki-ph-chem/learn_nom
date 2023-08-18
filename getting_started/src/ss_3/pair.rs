use nom::{bytes::complete::tag, branch::alt, sequence::pair, IResult};
use std::error::Error;

fn parse_pair(input: &str) -> IResult<&str, (&str,&str)> {
    pair(
        tag("abc"),
        tag("xyz")
    )(input)
}

fn main() -> Result<(),Box<dyn Error>> {
    let (_x, (abc, xyz)) = parse_pair("abcxyz")?; 
    println!("abc = {}", abc);
    println!("xyz = {}", xyz);

    Ok(())
}
