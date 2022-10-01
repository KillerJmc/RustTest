// 类型的别名，不是新s类型
type Int = i32;

fn main() {
    let a = 3;
    let b: Int = 4;
    println!("{}", a + b);
}
