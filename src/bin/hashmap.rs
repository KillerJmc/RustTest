use std::collections::HashMap;

fn main() {
    let k = "id".to_string();
    let v = "1".to_string();

    // 如果后面会放入元素，则无需指明类型
    let mut map = HashMap::new();

    // 直接插入，会发生move
    // map.insert(k, v);

    // 插入引用，不会move
    map.insert(&k, &v);

    println!("{} -> {}", k, v);

    let k = "name".to_string();
    let mut v = "Jmc".to_string();
    // 当不存在键时候插入（返回的是插入的值）
    let inserted_value = map.entry(&k)
        .or_insert(&mut v);

    println!("inserted_value: {}", inserted_value);

    // 获取
    match map.get(&k) {
        Some(v) => println!("value: {}", v),
        None => println!("获取失败！")
    }

    // for遍历
    for (k, v) in &map {
        println!("{} -> {}", k, v);
    }

    // 直接打印
    println!("{:?}", map);
}
