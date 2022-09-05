fn main() {
    // vector普通创建方式
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    println!("{}", v.get(0).expect("None!"));

    // vector宏定义创建方式
    let mut v = vec![1, 2, 3, 4, 5];

    // 非索引（拷贝的方式）
    let e = v[0];
    println!("{}", e);

    // 遍历读取（获取引用方式）
    for t in &mut v {
        // 取值并操作
        *t += 100;
        println!("{}", t);
    }
    for t in &v {
        println!("{}", t);
    }
}
