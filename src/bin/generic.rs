use std::ops::Add;

fn main() {
    println!("{}", add(3, 4));
    println!("{}", add(1.0, 3.5));

    let a: i32 = 666;
    // 需要指明返回类型
    let _: i64 = cast(a);
    // 通过Rust语法补全所有泛型信息，_表示这个泛型参数自动推导
    let _ = cast::<_, i64>(a);
    println!("{}", a);
}

// Output = T表示返回类型是T，该类型在Add接口里面
fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// 实现(Try)Into<转换后类型>或者(Try)From<转换前类型> 就可以使结构体实现转换
// Try表示可能会转换失败，如果没有Try就必须保证转换成功
// 调用转换有3种方式：
// 例：a, b (Type A -> Type B)
// let _ = a as B; // 实现Into接口
// let _: B = a.try_into().unwrap(); // 实现TryInto接口
// let _: B = B::try_from(a).unwrap(); // 实现TryFrom接口
// let _ = B::from(a); // 实现From接口
// let _: B = A::into(a); // 实现Into接口
fn cast<T: Into<R>, R>(t: T) -> R {
    t.into()
}
