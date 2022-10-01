use std::fmt::{Display, Formatter};
use std::ops::Add;

fn main() {
    let p1 = Point { x: 2, y: 1 };
    let p2 = Point { x: 1, y: 3 };

    // 打印
    println!("p1 = {}", p1.to_string());
    println!("p2 = {}", p2);

    // 相加 p1.add(p2)的缩写
    let res = p1 + p2;
    println!("p1 + p2 = {}", res);

}

struct Point {
    x: i32,
    y: i32
}

// 实现to_string()操作
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // 这是f.write_fmt(format_args!("point: {{ x = {}, y = {} }}", self.x, self.y))的宏简写
        write!(f, "point: {{ x = {}, y = {} }}", self.x, self.y)
    }
}

// 实现加法运算符
// 默认是Add<Self>
// 可以指定自身类型使不同类型相加：impl Add<MillMeter> for Meter
// 意思是给Meter类型实现加上MillMeter
impl Add for Point {
    // 指明相加后类型
    // 这个类型限制只能实现一次Add trait
    // 因为实现签名（impl Add for xxx）相同
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

