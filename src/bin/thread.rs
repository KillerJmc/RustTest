use std::sync::{mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    intern();
}

fn intern() {
    // mpsc: multi producer, single consumer
    // 创建2组通道，用于线程间通信
    let (finished_sender, begin_receiver) = mpsc::channel();
    let (finished_sender2, begin_receiver2) = mpsc::channel();

    let num_t = thread::spawn(move || {
        for i in 1..=26 {
            println!("num_t: {}", i);
            thread::sleep(Duration::from_secs(1));
            // 向通道发送消息（不堵塞）激活letter_t
            finished_sender.send(()).unwrap();
            // 从通道请求数据（阻塞）等待letter_t激活
            begin_receiver2.recv().unwrap();
        }
    });

    let letter_t = thread::spawn(move || {
        for c in 'a'..='z' {
            // 从通道请求数据（阻塞）等待num_t激活
            begin_receiver.recv().unwrap();
            println!("letter_t: {}", c);
            thread::sleep(Duration::from_secs(1));
            // 向通道发送消息（不堵塞）激活num_t
            finished_sender2.send(()).unwrap();
        }
    });

    letter_t.join().unwrap();
    num_t.join().unwrap();
}
