use std::fmt::{Display, Formatter};

fn main() {
    // 构造逆序链表节点Vec
    let mut v = (1..=10)
        .rev() // 为了之后pop顺利，逆转一下
        .map(Node::new)
        .map(Box::new)
        .map(Some)
        .collect::<Vec<Option<Box<Node>>>>();

    // 链表，设置为可编辑的
    let mut list = v.pop().unwrap();

    // 临时变量n，储存节点的可编辑引用
    let mut n = list.as_mut().unwrap();

    // 如果v不为空
    while !v.is_empty() {
        // v.pop直接返回移动后的值，赋予n.next，n是可变引用，不会被移动
        n.next = v.pop().unwrap();
        // 把n设置为下一个节点的可变引用
        n = n.next.as_mut().unwrap();
    }

    // 打印整个链表
    println!("list: {}", list.as_ref().unwrap());
}

/// 节点
struct Node {
    next: Option<Box<Node>>,
    val: i32
}

impl Node {
    /// 构造方法
    pub fn new(val: i32) -> Self {
        Node {
            next: None,
            val
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // 下个节点为None就停止
        let next_str = match self.next.as_ref() {
            Some(x) => x.to_string(),
            None => "None".to_string()
        };

        write!(f, "{} -> {}", self.val, next_str)
    }
}
