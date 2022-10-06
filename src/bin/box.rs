use std::ops::Deref;
use jmc_tools::Console;

fn main() {
    // box是个智能指针，指向堆内存，没有性能开销
    // rust默认把对象创建在栈中
    let i = Box::new(3);

    // 相当于 *i.deref()，即会自动调用i的deref方法，然后才用*取值
    let deref_i = *i;
    println!("{}", i);
    println!("{}", deref_i);

    Console::new_line();

    let s = Box::new("666".to_string());
    println!("{}", s);
    let deref_s = *s;
    println!("{}", deref_s);
    // 报错：在*s已经被移动了！
    // println!("{}", s);

    Console::new_line();

    let j = MyBox::new(4);
    let deref_j = *j;
    println!("{:?}", j);
    println!("{}", deref_j);
    Console::new_line();
}

/// 自定义Box <br>
/// 元组结构体，给(T)这个元组起了MyBox的名字
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}

// 实现解引用
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        println!("MyBox is being deref!");
        // 返回元组的第一个值，就是T
        &self.0
    }
}

// 实现析构
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("MyBox is being dropped!");
    }
}
