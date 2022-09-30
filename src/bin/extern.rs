use std::ptr::null;

extern "C" {
    fn pow(x: f64, y: f64) -> f64;
}

fn main() {
    unsafe {
        println!("{}", pow(2.0, 5.0));
    }
}

/// 该函数可以被C语言调用 <br>
/// #\[no_mangle\] 不要被损坏：被C语言调用时函数不要被编译器改名 <br><br>
/// 用法： <br>
/// 1. 新建rust lib项目，在lib.rs里面写入该函数
/// 2. 在 `Cargo.toml` 中写入
/// ```
/// [lib]
/// crate-type = ["staticlib"]
/// name = "rust2c"
/// ```
/// 3. 编写main.c
/// ```
/// extern void print(const char* s);
///
/// int main()
/// {
///     print("666");
/// }
/// ```
/// 4. 在Ubuntu里面输入（MSVC太麻烦不采用）
/// ```
/// #在target文件夹中生成.a动态库文件
/// cargo build
/// gcc -o main main.c librust2c.a
/// #函数成功被调用，但print函数打印都会乱码，现在还无法解决
/// ./main
/// ```
#[no_mangle]
pub extern "C" fn print(s: *const char) {
    // 检查指针是否为null
    if s == null() {
        eprintln!("s is a nullptr!");
        return
    }

    unsafe {
        // 打印
        println!("Rust println: {}", (*s).to_string());
    }
}
