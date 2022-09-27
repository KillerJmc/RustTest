use std::rc::Rc;
use jmc_tools::console::new_line;

fn main() {
    let t = "?".to_string();
    let _ = Box::new(t);
    // 报错，t已经在Box::new被move
    //let r_2 = Box::new(t);

    let s = "hello".to_string();
    // Reference Counting 引用计数指针，在堆中存放数据，提供多个不可变引用达到共享
    // 只能用于单线程
    let s_ref = Rc::new(s);
    println!("强引用个数：{}", Rc::strong_count(&s_ref));
    new_line();
    // 引用计数 + 1
    let _ = Rc::clone(&s_ref);
    println!("强引用个数：{}", Rc::strong_count(&s_ref));
    new_line();
    {
        // 引用计数 + 1
        let _ = Rc::clone(&s_ref);
        println!("现在强引用个数：{}", Rc::strong_count(&s_ref));
        new_line();
    }
    println!("之后强引用个数：{}", Rc::strong_count(&s_ref));
}
