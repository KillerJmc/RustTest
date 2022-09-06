fn main() {
    let cat = Cat {
        name: "猫".to_string()
    };
    cat.eat();
    eat(&cat);
    eat2(&cat);
    eat3(&cat);
    println!("{}", cat.name());
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
