use std::fs::File;
use std::io::{Error, Read};

fn main() {
    // panic!("炸啦！")

    let v = vec![1, 2, 3];
    // 这种情况可以恢复
    match v.get(100) {
        None => println!("索引越界！"),
        Some(_) => println!("成功获取")
    }

    // 此错误不可恢复
    // v[100];

    // 如果错误就直接panic，否则返回数据
    let content = try_read("hello.txt").unwrap();
    println!("content: {}", content);
}

fn try_read(path: &str) -> Result<String, Error> {
    let mut s = String::new();

    // let mut file = match File::open(path) {
    //     Ok(f) => f,
    //     // 错误就直接返回
    //     Err(e) => return Err(e)
    // };
    // match file.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => return Err(e)
    // }
    // 等同于以上
    File::open(path) ?
        .read_to_string(&mut s) ?;
    Ok(s)
}

