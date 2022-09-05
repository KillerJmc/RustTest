fn main() {
    let mut s = String::new();

    // 可变引用（类似java的引用，rust保证不会造成dangling ptr）
    let s_mut_ref = &mut s;

    s_mut_ref.push_str("hello");

    println!("{}", s_mut_ref);

    // 不可变引用
    let s_immut_ref = &s;
    // 报错
    // s_immut_ref.push_str("hello");
    println!("{}", s_immut_ref);
}
