use std::io; // prelude
use rand::Rng; // trait 接口
use std::cmp::Ordering;

fn main() {
    // !表示为宏定义
    println!("猜数游戏");

    // 不可修改的变量
    // ::表示是rand命名空间下的函数
    let res = rand::thread_rng().gen_range(1..101);

    loop {
        println!("请输入一个数（1 ~ 100）：");

        // 可修改的变量
        let mut num = String::new();

        // 获取输入，传入一个变量的引用，需要注明为可修改
        // expect表示处理read_line返回的Result，类似js Promise
        io::stdin()
            .read_line(&mut num)
            .expect("无法读取行");

        // shadow 隐藏之前变量（得到类似覆盖的结果）
        // trim去除空格（\n）
        // parse把num转化为i32类型
        // Ok(n)中n表示捐结果数字
        let num: i32 = match num.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("请输入数字！");
                continue;
            }
        };

        // 类似switch
        match num.cmp(&res) {
            Ordering::Less => println!("猜小了。"),
            Ordering::Greater => println!("猜大了。"),
            Ordering::Equal => {
                println!("答案正确！");
                break;
            }
        }
    }
}
