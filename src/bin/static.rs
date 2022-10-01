// 声明全局静态变量，命名规范和常量类似
static mut COUNTER: i32 = 0;

unsafe fn add_counter(x: i32) {
    // 全局静态变量会有线程安全问题，是unsafe的
    COUNTER += x;
}

fn main() {
    unsafe { add_counter(3) }
    unsafe {
        // 打印全局静态变量也是不安全的
        println!("{}", COUNTER);
    }
}
