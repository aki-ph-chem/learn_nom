use std::error::Error;
use nom::{IResult, bytes::complete::tag, branch::alt};
use nom::sequence::{separated_pair, tuple};
use nom::character::complete::{i32, line_ending};

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

fn first_integer_2d(input: &str) -> IResult<&str, ((i32, i32), &str)> {
    tuple((
            parse_integer_2d,
            alt((line_ending, tag("")))
    ))(input)
}

fn parse_integer_2d_colum(input: &str, result: &mut Vec<(i32, i32)>){
    match first_integer_2d(input) {
        Err(e) => panic!("Error: {}", e),
        Ok((remaining, (pair, _delimiter))) => {
            result.push(pair);
            if remaining == "" {
                return;
            }
            parse_integer_2d_colum(remaining, result);
        }
    }
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

fn first_integer_3d(input: &str) -> IResult<&str, ((i32, i32, i32), &str)> {
    tuple((
            parse_integer_3d,
            alt((line_ending, tag("")))
    ))(input)
}

fn parse_integer_3d_colum(input: &str, result: &mut Vec<(i32, i32, i32)>) {
    match first_integer_3d(input) {
        Err(e) => panic!("Error: {}", e),
        Ok((remaining, (coo, _delimiter))) => {
            result.push(coo);
            if remaining == "" {
                return;
            } 
            parse_integer_3d_colum(remaining, result);
        }
    }
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

fn first_integer(input: &str) -> IResult<&str, (i32, &str)> {
    tuple((
            i32,
            alt((tag(","),tag("")))
    ))(input)
}

fn parse_integer_nd(input: &str, result: &mut Vec<i32>){
    match first_integer(input) {
        Err(e) => panic!("Error: {}",e),
        Ok((remain,(value, _delimiter))) => {
            result.push(value);
            if remain == "" {
                return;
            }
            parse_integer_nd(remain, result);
        }
    }
}

fn first_abc_line(input: &str) -> IResult<&str, (&str,&str)> {
    tuple((
            tag("abc"),
            alt((line_ending, tag("")))
    ))(input)
}

fn parse_abc_n_colum(input: &str, result: &mut Vec<String>) {
    match first_abc_line(input) {
        Err(e) => panic!("Error: {}", e),
        Ok((remain, (abc, _delimiter))) => {
            result.push(abc.to_string());
            if remain == ""{
                return;
            }
            parse_abc_n_colum(remain, result);
        }
    }
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

    #[test]
    fn test_first_integer() -> Result<(), Box<dyn Error>> {
        let (remain, (value, _delimiter)) = first_integer("1,2,3")?;
        assert_eq!((1,"2,3"), (value, remain));

        let (remain, (value, _delimiter)) = first_integer("1,2,3,")?;
        assert_eq!((1,"2,3,"), (value, remain));

        let (remain, (value, _delimiter)) = first_integer("3,")?;
        assert_eq!((3,""), (value, remain));

        let (_remain, (_value, _delimiter)) = first_integer("3")?;

        Ok(())
    }

    #[test]
    fn test_parse_integer_nd() -> Result<(), Box<dyn Error>> {
        let mut result = vec![];
        parse_integer_nd("1,2,3,", &mut result);
        assert_eq!(vec![1,2,3], result);

        let mut result = vec![];
        parse_integer_nd("1,2,3,4,5,", &mut result);
        assert_eq!(vec![1,2,3,4,5], result);

        let mut result = vec![];
        parse_integer_nd("1,2,3", &mut result);
        assert_eq!(vec![1,2,3], result);

        Ok(())
    }

    #[test]
    fn test_first_abc_colum() -> Result<(), Box<dyn Error>> {
        let abc_clum = 
r"abc
abc
abc";

        let (remaining, (abc,_delimiter)) = first_abc_colum(abc_clum)?;
        assert_eq!(abc, "abc");
        assert_eq!(remaining, "abc\nabc");

        Ok(())
    }

    #[test]
    fn test_parse_abc_n_colum() -> Result<(), Box<dyn Error>> {
        let abc_clum = 
r"abc
abc
abc";
        let mut result = vec![];
        parse_abc_n_colum(abc_clum, &mut result);
        assert_eq!(result, vec!["abc".to_string(),"abc".to_string(),"abc".to_string()]);

        Ok(())
    }

    #[test]
    fn test_first_integer_2d() -> Result<(),Box<dyn Error>> {
        let data_sample =
r"1,2
3,4
5,6";
        let (remain,((x,y), _delimiter)) = first_integer_2d(data_sample)?;
        assert_eq!((x,y),(1,2));
        assert_eq!(remain,"3,4\n5,6");

        Ok(())
    }

    #[test]
    fn test_parse_integer_2d_colum() -> Result<(), Box<dyn Error>> {
        let data_sample =
r"1,2
3,4
5,6";
        let mut result = vec![];
        parse_integer_2d_colum(data_sample, &mut result);
        assert_eq!(vec![(1,2),(3,4),(5,6)], result);

        let data_sample =
r"1,2
3,4
13,14
31,41
5,6";
        let mut result = vec![];
        parse_integer_2d_colum(data_sample, &mut result);
        assert_eq!(vec![(1,2),(3,4),(13,14),(31,41),(5,6)], result);

        Ok(())
    }

    #[test]
    fn test_first_integer_3d() -> Result<(), Box<dyn Error>> {
        let data_sample =
r"1,2,3
3,4,9
5,8,12";
        let (remain, ((x,y,z), _delimiter)) = first_integer_3d(data_sample)?;
        assert_eq!((1,2,3), (x,y,z));
        assert_eq!("3,4,9\n5,8,12", remain);

        Ok(())
    }

    #[test]
    fn test_parse_integer_3d_colum() -> Result<(), Box<dyn Error>> {
        let data_sample =
r"1,2,3
3,4,9
5,8,12";
        let mut result = vec![];
        parse_integer_3d_colum(data_sample, &mut result);
        assert_eq!(vec![(1,2,3),(3,4,9),(5,8,12)], result);

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

    let mut result_vector = vec![];
    parse_integer_nd("1,2,3,4", &mut result_vector);
    println!("result_vector = {:?}", result_vector);

    let mut result_vector = vec![];
    let sample_data =
r"1,2
3,4
5,6";
    parse_integer_2d_colum(sample_data, &mut result_vector);
    println!("result_vector = {:?}", result_vector);

    Ok(())
}
