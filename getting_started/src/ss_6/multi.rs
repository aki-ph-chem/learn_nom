use std::error::Error;
use nom::IResult;
use nom::multi::{count, many0, many_m_n, many_till};
use nom::bytes::complete::tag;

fn count_abc_nth(input: &str, n: usize) -> IResult<&str, Vec<&str>> {
    count(tag("abc"), n)(input)
}

fn many0_abc(input: &str) -> IResult<&str, Vec<&str>> {
    many0(tag("abc"))(input)
}

fn many_m_n_ab(input: &str, m: usize, n: usize) -> IResult<&str, Vec<&str>> {
    many_m_n(m, n, tag("ab"))(input)
}

fn many_till_ab(input: &str) -> IResult<&str, (Vec<&str>, &str)> {
    many_till(tag("abc"), tag("end"))(input)
}

// To Do: separated_list_0, fold_many0, fold_many_m_n, length_count  

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_abc_nth() {
        assert_eq!(count_abc_nth("abcabc", 2), Ok(("", vec!["abc", "abc"])));
        assert_eq!(count_abc_nth("abc_abc", 1), Ok(("_abc", vec!["abc"])));
    }

    #[test]
    fn test_many0_abc() {
        assert_eq!(many0_abc("abcabc"), Ok(("", vec!["abc", "abc"])));
        assert_eq!(many0_abc("abc123"), Ok(("123", vec!["abc"])));
        assert_eq!(many0_abc("123"), Ok(("123", vec![])));
    }

    #[test]
    fn test_many_m_n_ab() {
        assert_eq!(many_m_n_ab("abab", 0, 2), Ok(("", vec!["ab", "ab"])));
        assert_eq!(many_m_n_ab("abab123", 0, 2), Ok(("123", vec!["ab", "ab"])));
        assert_eq!(many_m_n_ab("", 0, 2), Ok(("", vec![])));
    }

    #[test]
    fn test_many_till_ab() {
        assert_eq!(many_till_ab("abcabcend"), Ok(("", (vec!["abc", "abc"], "end"))));
        assert_eq!(many_till_ab("abcend_efg"), Ok(("_efg", (vec!["abc"], "end"))));
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    Ok(())
}
