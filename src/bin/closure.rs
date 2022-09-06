
fn main() {
    // 闭包可以自动推断参数和返回值类型
    let add = |x, y| x + y;

    println!("{}", add(1, 2));

    calc_sum();
}


fn calc_sum() {
    let v = vec![1, 2, 3];
    let res = sum(&v, |t| *t);
    println!("sum = {}", res);
}

// Fn是个官方的接口，写法是个新语法（现在只在Fn系列函数里有效）
// 简写：Fn(i32, i64) -> String
// 实际：Fn<(i32, i64), Output=String>
// Fn可以上文中捕获非mut变量
// FnOnce同上，但执行一次后会被Move，不能再使用
// FnMut可以从上文中捕获可修改的变量
fn sum<T, F: Fn(&T) -> i32>(list: &Vec<T>, to_i32: F) -> i32 {
    let mut res = 0;
    for t in list {
        res += to_i32(t);
    }
    res
}
