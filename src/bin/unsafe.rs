fn main() {
    let mut num = 5;
    // 定义2个原始指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // unsafe块定义中的指针不会被Rust编译器释放
    unsafe {
        // 对其解引用必须在unsafe里面进行
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);
        // unsafe函数必须在unsafe块中被调用
        dangerous();
    }
    // 包含unsafe块的函数也可以是安全函数
    safe();
}

unsafe fn dangerous() {
    let t = 6;
    let ptr = &t as *const i32;
    println!("t = {}", *ptr);
}

fn safe() {
    let u = 6;
    let ptr = &u as *const i32;
    unsafe {
        println!("u = {}", *ptr);
    }
}
