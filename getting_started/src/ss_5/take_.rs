use std::error::Error;
use nom::bytes::complete::{take_until, take_till, take_while};
use nom::character::is_alphabetic;
use nom::IResult;

fn till_colon(input: &str) -> IResult<&str, &str> {
    take_till(|c| c == ':')(input)
}

fn until_end(input: &str) -> IResult<&str, &str> {
    take_until("end")(input)
}

fn while_alphabet(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(is_alphabetic)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_till_colon() {
        assert_eq!(till_colon("abc:123"), Ok((":123", "abc")));
    }

    #[test]
    fn test_until_end() {
        assert_eq!(until_end("abc_123_xyz_end_cde_456_uvw"), 
            Ok(("end_cde_456_uvw", "abc_123_xyz_")));
    }

    #[test]
    fn test_while_alphabet() {
        assert_eq!(while_alphabet(b"abc123"), Ok((&b"123"[..], &b"abc"[..])));
        assert_eq!(while_alphabet(b"12345"), Ok((&b"12345"[..], &b""[..])));
        assert_eq!(while_alphabet(b"abcxyz"), Ok((&b""[..], &b"abcxyz"[..])));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (remaining, result) = till_colon("abc:123")?;
    println!(" remaining = {}\n result = {}", remaining, result);

    let (remaining, result) = until_end("def fun(x,y) x + y end")?;
    println!(" remaining = {}\n result = {}", remaining, result);

    Ok(())
}
