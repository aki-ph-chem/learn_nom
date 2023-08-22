use nom::bytes::complete::tag;
use nom::multi::{count, fold_many0, many0, many_m_n, 
    many_till, separated_list0, fold_many_m_n, length_count};
use nom::combinator::map;
use nom::number::complete::u8;
use nom::IResult;
use std::error::Error;

fn count_abc_nth(input: &str, n: usize) -> IResult<&str, Vec<&str>> {
    count(tag("abc"), n)(input)
}

fn many0_abc(input: &str) -> IResult<&str, Vec<&str>> {
    many0(tag("abc"))(input)
}

fn many_m_n_ab(input: &str, m: usize, n: usize) -> IResult<&str, Vec<&str>> {
    many_m_n(m, n, tag("ab"))(input)
}

fn many_till_abc(input: &str) -> IResult<&str, (Vec<&str>, &str)> {
    many_till(tag("abc"), tag("end"))(input)
}

fn separated_list0_abc<'a>(input: &'a str, delimiter: &'a str) -> IResult<&'a str, Vec<&'a str>> {
    separated_list0(tag(delimiter), tag("abc"))(input)
}

fn fold_many_0_abc(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many0(
        tag("abc"), 
        Vec::new, 
        |mut acc: Vec<&str>, item| {
            acc.push(item);
            acc
        })(input)
}

fn fold_many_m_n_abc(input: &str, m: usize, n: usize) -> IResult<&str, Vec<&str>> {
    fold_many_m_n(
        m,
        n,
        tag("abc"),
        Vec::new,
        |mut acc: Vec<&str>, item| {
            acc.push(item);
            acc
        }
    )(input)
}

fn length_count_abc(input: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    length_count(
        map(u8,
            |i| {println!("got numbeer : {}", i);
            i}),
            tag("abc")
    )(input)
} 

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
    fn test_many_till_abc() {
        assert_eq!(
            many_till_abc("abcabcend"),
            Ok(("", (vec!["abc", "abc"], "end")))
        );
        assert_eq!(
            many_till_abc("abcend_efg"),
            Ok(("_efg", (vec!["abc"], "end")))
        );
    }

    #[test]
    fn test_separated_list0_abc() {
        assert_eq!(
            separated_list0_abc("abc,abc,abc", ","),
            Ok(("", vec!["abc", "abc", "abc"]))
        );
        assert_eq!(
            separated_list0_abc("abc,def", ","),
            Ok((",def", vec!["abc"]))
        );
        assert_eq!(
            separated_list0_abc("abc;abc;abc", ";"),
            Ok(("", vec!["abc", "abc", "abc"]))
        );
        assert_eq!(
            separated_list0_abc("abc_and_abc_and_abc", "_and_"),
            Ok(("", vec!["abc", "abc", "abc"]))
        );
    }

    #[test]
    fn test_fold_many0_abc() {
        assert_eq!(
            fold_many_0_abc("abcabcabc"),
            Ok(("", vec!["abc", "abc", "abc"]))
        );
        assert_eq!(
            fold_many_0_abc("abcabc123"),
            Ok(("123", vec!["abc", "abc"]))
        );
    }

    #[test]
    fn test_fold_many_m_n_abc() {
        assert_eq!(
            fold_many_m_n_abc("abcabcabc", 0, 2),
            Ok(("abc", vec!["abc", "abc"]))
        );
    }

    #[test]
    fn test_length_count_abc() {
        assert_eq!(length_count_abc(&b"\x02abcabcabc"[..]), Ok((&b"abc"[..], vec![&b"abc"[..], &b"abc"[..]])));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("count_abc_nth");
    let (remaining, result) = count_abc_nth("abcabc", 2)?;
    println!(" remaining = {} \n result = {:?}", remaining, result);

    println!("many0_abc");
    let (remaining, result) = many0_abc("abcabc")?;
    println!(" remaining = {} \n result = {:?}", remaining, result);

    println!("many_m_n_ab");
    let (remaining, result) = many_m_n_ab("ababc", 0, 2)?;
    println!(" remaining = {} \n result = {:?}", remaining, result);

    println!("many_till_abc");
    let (remaining, result) = many_till_abc("abcabcend")?;
    println!(" remaining = {} \n result = {:?}", remaining, result);

    println!("separated_list0_abc");
    let (remaining, result) = separated_list0_abc("abc,abc,abc", ",")?;
    println!(" remaining = {} \n result = {:?}", remaining, result);
    let (remaining, result) = separated_list0_abc("abc_and_abc_and_abc", "_and_" )?;
    println!(" remaining = {} \n result = {:?}", remaining, result);

    println!("fold_many0_abc");
    let (remaining, result) = fold_many_0_abc("abcabcabc123")?;
    println!(" remaining = {} \n result = {:?}", remaining, result);

    println!("fold_many_m_n_abc");
    let (remaining, result) = fold_many_m_n_abc("abcabcabc", 0, 2)?;
    println!(" remaining = {} \n result = {:?}", remaining, result);

    println!("length_count_abc");
    let (remaining, result) = length_count_abc(&b"\x02abcabcabc"[..])?;
    println!(" remaining = {:?} \n result = {:?}", remaining, result);

    Ok(())
}
