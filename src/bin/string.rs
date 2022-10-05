use std::ops::Index;
use jmc_tools::console::new_line;

fn main() {
    // 右边是个字符串常量值，存在预读文本中，只能通过引用标识
    // 如果写str的话，会导致它尝试在栈上创建字符串，但因为字符串长度未知，所以不成立
    // 'static表示它的生命周期是全局的，贯穿整个程序的执行
    // 而&str表示str的引用，一个指针，只包含str的地址和长度
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

    new_line();

    // 字符串的索引（只能支持字节遍历的索引）
    let s = "abcdefg";
    println!("{}", s.index(3..4));
    let s = s.to_string();
    println!("{}", s.index(4..5));
    // 如果用多字节文字必须计算好字节，否则会报错
    println!("{}", "你好".index(0..3));
    // 使用字符流获取
    println!("{}", "你好".chars().collect::<Vec<char>>()[1]);

    new_line();

    // 获取字符串长度
    let s = "你好啊渣渣666";
    // 字节长度
    println!("{}", s.len());
    // 字符长度
    println!("{}", s.chars().count());
}
