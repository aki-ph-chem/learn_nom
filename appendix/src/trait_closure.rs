// Boxを使う
// ヒープ領域に置かれる
fn gen_closure_box(a: i32) -> Box<dyn Fn(i32) -> i32 > {
    Box::new(move |x| a * x + 1)
}

// implを使う
// スタック領域に置かれる
fn gen_closure_impl(a: i32) -> impl Fn(i32) -> i32 {
    move |x| a * x + 1
}

fn main() {
    let func_a_1 = gen_closure_box(2);
    println!("funx_a_1({}) = {}", 10, func_a_1(10));

    let func_a_2 = gen_closure_impl(3);
    println!("funx_a_2({}) = {}", 100, func_a_2(100));
}
