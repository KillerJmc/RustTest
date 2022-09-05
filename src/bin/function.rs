fn main() {
    println!("函数返回：{}", f(5));
}

// 函数必须标明类型
fn f(x: i32) -> i32 {
    println!("x = {}", x);
    // 最后一个表达式为返回值，不需要return
    x
}
