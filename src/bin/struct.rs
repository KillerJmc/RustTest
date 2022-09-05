// 结构体
struct User {
    id: i64,
    name: String,
    password: String,
    sex: Sex
}

// 方法
impl User {
    // 静态方法
    fn info() -> &'static str {
        "A class named User"
    }

    // 成员方法
    fn to_str(&self) -> String {
        std::format!("id: {}, name: {}, password: {}, sex: {}",
                     self.id,
                     self.name,
                     self.password,
                     self.sex.to_str()
        )
    }
}

// 枚举
enum Sex {
    MALE,
    FEMALE
}

// 方法
impl Sex {
    fn to_str(&self) -> &'static str {
        match &self {
            Sex::MALE => "男",
            // _表示剩余的情况
            _ => "女"
        }
    }
}

fn main() {
    let u = User {
        id: 1,
        name: String::from("Jmc"),
        password: "123".to_string(),
        sex: Sex::MALE
    };

    let u2 = User {
        id: 2,
        name: String::from("Merry"),
        password: "456".to_string(),
        sex: Sex::FEMALE
    };

    println!("{}", User::info());
    println!("{}", u.to_str());
    println!("{}", u2.to_str());
}



