use std::fmt::Display;

fn main() {
    // 函数版println，繁琐
    println(vec!(1, 2, 3));
    println(Vec::<i32>::new());

    // 宏定义版println，简单
    my_println!(1, 2, 3);
    my_println!();
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

