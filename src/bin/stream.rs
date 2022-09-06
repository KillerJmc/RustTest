use std::fs;

/*
 iterator原理：
    对于每一个操作，如：map, filter等，都会产生一个包装类，保存当前对象和转换函数
    如：Map { iter, f } 和 Filter { iter, predicate }，其中iter都是填的self
    而对于Map, Filter，他们都实现了Iterator，覆盖了next方法，添加了自己的筛选规则

    当用户调用终止操作（比如collect）时，会多次调用当前包装类的next消耗iterator, 此时
    如果是.filter.map就会先调用Map的next方法，执行：self.iter.next().map(...)
    可以得知他还调用上一级，即Filter的next方法后，才会调用自己的next方法，因此是filter先执行，合理。

    从宏观上看，都是对同一迭代器进行操作，必须保证元素的顺序没有发生变化才能进行，因此iter没有sort方法。
    Java Stream.sort方法里面使用的是数组作为中间值进行排序，排序完重新传给下一级iter。
 */

fn main() {
   /*
        Students

        现在有一个 students.dat 文件，其中内容是每行一个人名。
        请你完成下面的任务：

        · 将人名按字典序进行排序
        · 去掉以 B 开头 或以 n 结尾的人名
        · 将剩下的所有人名全部转换为大写，收集到 List 中并输出
    */
    let mut list = fs::read_to_string("students.dat")
        .expect("文件读取失败！")
        .lines()
        .filter(|t| !t.starts_with("B") && !t.ends_with("n"))
        .map(|t| t.to_uppercase())
        .collect::<Vec<_>>();

    // 不够优雅
    list.sort();

    list.iter()
        .for_each(|t| println!("{}", t));
}
