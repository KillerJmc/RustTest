fn test() {
    let s = String::from("Hello");
    let n = 5;

    // 为了防止指针2次释放，Rust把s转移给了函数（move）
    // 当对象离开作用域时，会自动调用drop函数从heap中清除，除非转移了所有权
    take_ownership(s);

    // 运行会报错
    // println!("{}", s);

    // n是i32类型，字面值纯拷贝，没有发生移动
    makes_copy(n);

    // 可以正常打印
    println!("{}", n);

    // 这里也会发生所有权的转移，从函数转移到这里
    let s = give_ownership();
    println!("{}", s);
}

fn take_ownership(s: String) {
    println!("s = {}", s);
}

fn makes_copy(n: i32) {
    println!("n = {}", n);
}

fn give_ownership() -> String {
    let s = String::from("here");
    s
}
