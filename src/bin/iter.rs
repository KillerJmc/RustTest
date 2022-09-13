fn main() {
    let list = List::of(1..=5);
    for i in 0..list.size() {
        println!("{}", list.get(i).unwrap());
    }

    println!("--------------------------");

    // move
    for t in list {
        println!("{}", t);
    }
}

/// 结点
struct Node<T: Copy> {
    /// 储存的值
    value: Option<T>,
    /// 下一个结点 <br>
    /// 生成在堆上，Box是一个智能指针
    next: Option<Box<Node<T>>>
}

/// 列表
struct List<T: Copy> {
    /// 列表头节点（不储存值）
    head: Box<Node<T>>,

    /// 列表长度
    size: u32,

    /// 内置迭代器
    iter: SimpleIter
}

impl<T: Copy> List<T> {
    /// 从迭代器创建实例
    fn of(objs: impl Iterator<Item = T>) -> List<T> {
        let mut instance = Self::new();

        for t in objs {
            instance.add(t);
            instance.size += 1;
        }
        instance
    }

    /// 创建初始化方法
    fn new() -> List<T> {
        let head = Box::new(Node {
            value: None,
            next: None
        });

        List {
            head,
            size: 0,
            iter: SimpleIter::new()
        }
    }

    /// 添加元素
    fn add(&mut self, t: T) {
        let mut last = &mut self.head;

        while last.next.is_some() {
            last = last.next.as_mut().unwrap();
        }

        let next = Node {
            value: Some(t),
            next: None
        };

        last.next = Some(Box::new(next));
    }

    /// 获取元素
    fn get(&self, mut idx: u32) -> Option<&T> {
        let mut n = self.head.next.as_ref();
        while idx > 0 {
            if n.is_none() {
                return None
            }
            n = n.unwrap().next.as_ref();
            idx -= 1;
        }
        n.unwrap().value.as_ref()
    }

    // 获取并转移所有权
    fn get_and_move(&self, mut idx: u32) -> Option<T> {
        let mut n = self.head.next.as_ref();
        while idx > 0 {
            if n.is_none() {
                return None
            }
            n = n.unwrap().next.as_ref();
            idx -= 1;
        }
        n.unwrap().value
    }

    /// 获取列表长度
    fn size(&self) -> u32 {
        self.size
    }
}

// IntoIter迭代器
impl<T: Copy> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.curr == self.size() {
            self.iter = SimpleIter::new();
            return None
        }
        let res = self.get_and_move(self.iter.curr);
        self.iter.curr += 1;
        res
    }
}

/// 简单迭代器
struct SimpleIter {
    curr: u32
}

impl SimpleIter {
    fn new() -> Self {
        SimpleIter {
            curr: 0
        }
    }
}


