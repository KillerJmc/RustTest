use std::fmt::Display;

fn main() {
    // 函数版println，繁琐
    println(vec!(1, 2, 3));
    println(Vec::<i32>::new());

    // 宏定义版println，简单
    my_println!(1, 2, 3);
    my_println!();

    // 自制min宏
    let res = min!(5, 3, 4, 1, 2);
    println!("min = {}", res);
}


/// 自己实现一个可打印多个参数的println
/// 因为Rust不支持变长参数和多个同名函数，所以只能借助Vec实现
fn println<E: Display, Args: Into<Option<Vec<E>>>>(args: Args) {
    if let Some(args) = args.into() {
        if !args.is_empty() {
            for t in args {
                println!("{}", t);
            }
            return;
        }
    }
    println!();
}

// 这个标注才能把宏导出（变为可导入的）
#[macro_export]
// 利用宏实现对不同参数个数的匹配
// 设计一个可打印多个参数的println
// 注：不能在调用宏时使用宏内的变量（两者变量相隔离）
macro_rules! my_println {
    // 匹配0个参数
    () => {
        println!();
    };

    // 匹配多个参数
    // 重复模式语法$(...)sep*
    // ...: 表示重复的变量
    // sep: 分隔符，可选
    // *: 表示0次或多次重复
    // +: 表示1次或多次重复
    // ?: 表示1次或0次重复
    // expr表示传入类型是表达式
    ($($x: expr),*) => {
        // 展开语法：$(...)*
        // ...: 表示语句
        $(println!("{}", $x);)*
    };
}

#[macro_export]
macro_rules! min {
    ($n: expr) => { $n };
    // tt：token tree，可以匹配非表达式，比如+
    // 这里使用多次重复：匹配举例：
    // min!(4, 1, 3, 2)
    // -> min!(4, min!(1, 3, 2))
    // -> min!(4, min!(1, min!(3, 2)))
    // -> min!(4, min!(1, std::cmp::min(3, 2)))
    // 从以上可以看出重复语法不断匹配tt类型，直接把数字和逗号都匹配了
    ($n: expr, $($tail: tt)+) => {
        std::cmp::min($n, min!($($tail)*))
    };
}

