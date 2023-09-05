# error handling

[getting started](https://tfpk.github.io/nominomicon/)のchapter 7を見るとGitHubに置いてあるdocsを見るように書いてあったので、その内容をやっていく。

[error management](https://github.com/rust-bakery/nom/blob/main/doc/error_management.md)

<!-- ドラフト -->
## nomのエラーについて

nomのエラーは以下のようにデザインされている。
- どのパーサーがinputのどこで失敗したのかを示す。
- エラーがパーサーのチェーンを伝搬していくにつれてよく多くのコンテキストが蓄積される。
- エラーは呼び出しもとのパーサーに破棄されることが多いため、オーバーヘッドが非常に小さい。 
- ユーザが求めるのに応じて、変更が可能であること。というのも言語によっては、多くの情報が必要であるためである。

この要請を満たすように`IResult<I,O,E>`は以下のように定義されている。

```Rust
pub type IResult<I, O, E=nom::error::Error<I>> = Result<(I, O), nom::Err<E>>;

pub enum Err<E> {
    Incomplete(Needed),
    Error(E),
    Failure(E),
}
```
