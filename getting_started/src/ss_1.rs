use nom::IResult;
use std::error::Error;

/// 返り値は `IResult<I,O>`でIはinputされる型(多くの場合&str),
/// Oはパースされた結果の型
///
///以下で定義されている
///
///```Rust
/// pub type IResult<I, O, E = error::Error<I>> = Result<(I, O), Err<E>>;
///```
///
/// 例) &str型の "123"を u64型の数値としての123に変換する
///
/// まずはじめに簡単なパーサー`do_nothing_parser()`を実装する
pub fn do_nothing_parser(input: &str) -> IResult<&str,&str> {
    Ok((input, ""))
}

fn main() -> Result<(), Box<dyn Error >> {
    let (remainig_input, output) = do_nothing_parser("my_input")?;
    assert_eq!(remainig_input, "my_input");
    assert_eq!(output, "");

    Ok(())
}
