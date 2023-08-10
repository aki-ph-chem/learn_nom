use std::error::Error;
use nom::{branch::alt, bytes::complete::tag,IResult};

// コンビネータ altを使う
fn parse_abc_or_def(input: &str) -> IResult<&str, &str> {
    alt((
        tag("abc"),
        tag("def")
    ))(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (leftover_input, output) = parse_abc_or_def("abcworld")?;
    assert_eq!(leftover_input, "world");
    assert_eq!(output, "abc");
    assert!(parse_abc_or_def("ghiworld").is_err());

    let (leftover_input, output) = parse_abc_or_def("defworld")?;
    assert_eq!(leftover_input, "world");
    assert_eq!(output, "def");

    let (leftover_input, output) = parse_abc_or_def("defabcworld")?;
    assert_eq!(leftover_input, "abcworld");
    assert_eq!(output, "def");

    Ok(())
}
