pub fn loop_test() {
    // 判断
    let n = 5;
    if n > 3 {
        println!("n > 3");
    } else if n < 3 {
        println!("n < 3");
    } else {
        println!("n = 3");
    }

    // 利用块获取返回值，类似三目
    let n = if n > 3 { 666 } else { 333 };
    println!("n = {}", n);

    // 无限循环
    let n = loop {
        println!("loop");
        // 退出循环并返回结果
        break 123;
    };
    println!("n = {}", n);

    // while循环
    let mut i = 2;
    while i > 0 {
        println!("loop!");
        i = i - 1;
    }

    // 增强for
    let a = [1, 2, 3, 4, 5];
    for t in a {
        println!("{}", t);
    }

    println!("-----------------");

    // for循环
    for t in (1..4).rev() {
        println!("{}", t);
    }
}
