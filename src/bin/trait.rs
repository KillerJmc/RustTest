fn main() {
    let cat = Cat {
        name: "猫".to_string()
    };
    cat.eat();
    eat(&cat);
    eat2(&cat);
    eat3(&cat);
    println!("{}", cat.name());

    get_cat().eat();
    get_animal(false).eat();
}

// 作为参数
fn eat(animal: &(impl Animal + Alive)) {
    animal.eat();
}

// 作为泛型参数
fn eat2<T: Animal + Alive>(animal: &T) {
    animal.eat();
}

// 泛型参数写法2
fn eat3<T>(animal: &T)
    where T: Animal + Alive
{
    animal.eat();
}

// impl为静态匹配，编译时会解开自动替换成对象类型，可以得知对象大小，编译通过
fn get_cat() -> impl Animal {
    Cat { name: "猫".to_string() }
}

// 此处不能使用impl，因为返回不同类型，编译时不能解开
// 需要用到dyn(dynamic)进行动态匹配（类似c++多态）
// 而多态不能得知对象大小，必须使用指针，最简单就是用Box
fn get_animal(is_cat: bool) -> Box<dyn Animal> {
    if is_cat {
        Box::new(Cat { name: "猫".to_string() })
    } else {
        Box::new(Dog { name: "狗".to_string() })
    }
}

// 相当于接口
trait Alive {}

trait Animal {
    fn eat(&self);
}

struct Cat {
    name: String
}

impl Cat {
    fn name(&self) -> &String {
        &self.name
    }
}

// 实现接口
impl Alive for Cat {}

impl Animal for Cat {
    fn eat(&self) {
        println!("{}：正在吃东西", self.name);
    }
}

struct Dog {
    name: String
}

impl Animal for Dog {
    fn eat(&self) {
        println!("{}：正在吃东西", self.name);
    }
}


