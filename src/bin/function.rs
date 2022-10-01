fn main() {
    println!("函数返回：{}", f(5));
    print_res(|| { 3 + 4 });
}

// 函数必须标明类型
fn f(x: i32) -> i32 {
    println!("x = {}", x);
    // 最后一个表达式为返回值，不需要return
    x
}

// 传入函数指针
fn print_res(add: fn() -> i32) {
    println!("{}", add());
}
