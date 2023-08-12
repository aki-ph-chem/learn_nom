use nom::{IResult, bytes::complete::tag};
use nom::sequence::{separated_pair, delimited};
use std::error::Error;

//  もう少し複雑な例
// 座標
#[derive(Debug,PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

// 1.  &str -> i32
use nom::character::complete::i32;

// 2
fn parse_integer_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        i32,
        tag(","),
        i32
    )(input)
}

// 3
fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (remainig, (x, y)) = delimited(
        tag("("),
        parse_integer_pair,
        tag(")"),
    )(input)?;

    Ok((remainig, Coordinate {x, y}))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (_, parsed) = parse_coordinate("(3,5)")?; 
    assert_eq!(parsed, Coordinate{x: 3, y: 5});

    let (_, parsed) = parse_coordinate("(2, -4)")?;
    assert_eq!(parsed, Coordinate{x: 2, y: -4});

    let parsing_error = parse_coordinate("(3,)");
    assert!(parsing_error.is_err());

    let parsing_error = parse_coordinate("(,3)");
    assert!(parsing_error.is_err());

    let parsing_error = parse_coordinate("Ferris");
    assert!(parsing_error.is_err());

    Ok(())
}
