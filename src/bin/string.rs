fn main() {
    // 右边是个字符串常量值，存在预读文本中，只能通过引用标识
    // 如果写str的话，会导致它尝试在栈上创建字符串，但因为字符串长度未知，所以不成立
    // 'static表示它的生命周期是全局的，贯穿整个程序的执行
    let s: &'static str = "111";
    println!("{}", s);

    let s: String = "666".to_string();
    // 签名：fn deref(&self) -> &str
    // &String会自动解引用成&str
    let str_s: &str = &s;
    println!("{}", str_s);

    let s = "a".to_string();
    let s2 = "b".to_string();
    let s3 = "c".to_string();

    // 签名：fn add(mut self, other: &str) -> String
    // 因此只能传入&str
    let mut res = s + "-" + &s2 + "-" + &s3;
    println!("{}", res);

    // 切片是string的引用，不能move
    let slice = &mut res[1..];
    println!("{}", slice);
    // 报错。Rust只允许一个可变借用，因为多个可变借用会导致线程安全问题
    // res.pop();

    // 简单format（s已经被移动了，不存在了）
    let s = format!("{}-{}-{}", "a", s2, s3);
    println!("{}", s);
}
