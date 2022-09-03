pub fn tuple() {
    let tuple = (500, 60.4, 1);
    println!("({}, {}, {})", tuple.0, tuple.1, tuple.2);

    let (x, y, z) = tuple;
    println!("({}, {}, {})", x, y, z);
}

pub fn array() {
    // 普通数组（栈上）
    let a = [1, 2, 3, 4, 5];
    let (x, y, z) = (a[0], a[1], a[2]);
    println!("[ {}, {}, {} ]", x, y, z);

    // 3个666
    let a2 = [666; 3];
    println!("[ {}, {}, {} ]", a2[0], a2[1], a2[2]);

    // 二维数组
    let a3 = [[1, 2], [3, 4], [5, 6]];
    println!(
        "[[ {}, {} ], [ {}, {} ], [ {}, {} ]]",
        a3[0][0], a3[0][1],
        a3[1][0], a3[1][1],
        a3[2][0], a3[2][1]
    );
}
