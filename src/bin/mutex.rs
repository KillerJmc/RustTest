use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    simple();
    count();
}

fn simple() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num += 1;
        // 离开作用域自动释放不需要unlock
    }

    println!("{:?}", m);
}

fn count() {
    // 此处不能使用Rc，Rc线程不安全
    // Arc是Atomic RC，使用原子性计算（cas乐观锁）的引用计数器，其中的值不能修改
    // Arc要配合Mutex进行使用，Mutex为互斥锁
    let counter = Arc::new(Mutex::new(0));

    (0..10)
        .into_iter()
        .map(|_| {
            // 互斥锁引用（必须写在外面让线程move）
            let num = Arc::clone(&counter);
            thread::spawn(move || {
                // 减小锁的粒度
                let mut lock = num.lock().unwrap();
                // 加1
                (0..10).for_each(|_| *lock += 1);
            })
        })
        .for_each(|t| t.join().unwrap());

    // 线程安全
    println!("counter = {}", *counter.lock().unwrap());
}
