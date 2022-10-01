use std::ffi::CStr;
use std::os::raw::c_char;
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
/// 4. 运行
///     + Ubuntu
///     ```
///     // 在target文件夹中生成.a动态库文件
///     cargo build
///
///     // 编译
///     gcc -o main main.c librust2c.a
///
///     // 运行
///     ./main
///     ```
///     + MSVC
///     ```
///     :: 在target文件夹中生成.a动态库文件
///     cargo build
///
///     :: 编译生成汇编文件
///     cl /c main.c
///
///     :: 与Rust动态库链接
///     link main.obj -libpath .\rust2c.lib -libpath wsock32.lib -libpath ws2_32.lib -libpath advapi32.lib -libpath Userenv.lib -libpath Bcrypt.lib
///
///     :: 运行
///     main.exe
///     ```
#[no_mangle]
pub unsafe extern "C" fn print(s: *const c_char) {
    // 检查指针是否为null
    if s == null() {
        eprintln!("s is a nullptr!");
        return
    }

    // 把const char *转换成&str
    let res = CStr::from_ptr(s).to_str().unwrap();

    // 打印
    println!("Rust println: {}", res);
}
