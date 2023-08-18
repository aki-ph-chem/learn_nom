use std::error::Error;
use nom::{IResult, bytes::complete::tag};
use nom::sequence::{separated_pair, tuple};
use nom::character::complete::{i32,alpha0};

#[derive(Debug,PartialEq)]
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

fn parse_integer_2d(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        i32,
        tag(","),
        i32
    )(input)
}

fn parse_integer_3d(input: &str) -> IResult<&str, (i32, i32, i32)> {
    let (remaining, (x, _,y, _,z)) = tuple((
            i32,
            tag(","),
            i32,
            tag(","),
            i32
    ))(input)?;

    Ok((remaining, (x,y,z)))
}

fn parse_to_point_3d(input: &str) -> IResult<&str, Point3D> {
    let (remaining, (x, _,y, _,z)) = tuple((
            i32,
            tag(","),
            i32,
            tag(","),
            i32
    ))(input)?;

    Ok((remaining, Point3D{x,y,z}))
}

mod tests{
    use super::*;

    #[test]
    fn test_parse_integer_2d() -> Result<(), Box<dyn Error>> {
        let (_remain, (x, y)) = parse_integer_2d("1,2")?;
        assert_eq!((1,2), (x,y));

        Ok(())
    }

    #[test]
    fn test_parse_integer_3d() -> Result<(), Box<dyn Error>> {

        let (_remain, (x, y, z)) = parse_integer_3d("1,2,3")?;
        assert_eq!((1,2,3), (x,y,z));

        Ok(())
    }

    #[test]
    fn test_parse_to_point_3d() -> Result<(), Box<dyn Error>> {
        let (_remain, point) = parse_to_point_3d("1,2,3")?;
        assert_eq!(Point3D{x: 1, y: 2, z: 3}, point);

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (_remaining,(x,y)) = parse_integer_2d("1,3")?;
    println!("x,y = {},{}", x, y);

    let (_remaining, (x, y, z)) = parse_integer_3d("1,2,3")?;
    println!("x,y,z = {},{},{}", x, y, z);

    let (_remaining, point) = parse_to_point_3d("1,2,3")?;
    println!("point = {:?}", point);

    Ok(())
}
