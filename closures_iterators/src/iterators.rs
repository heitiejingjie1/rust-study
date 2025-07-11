pub fn test_iter() {
    /*
     * 迭代器模式允许你依次对一个序列中的项执行某些操作.
     * 迭代器（iterator）负责遍历序列中的每一项并确定序列何时结束的逻辑.
     * 在rust中，迭代器是惰性的，在执行操作前不会自动调用迭代器.
     * */

    let vector = vec![2, 3, 9, 8];

    // 生成不可变迭代器引用
    let vector_iter = vector.iter();

    // let sum = vector_iter.sum();

    for val in vector_iter {
        println!("{val}");
    }

    // let sum: i32 = vector_iter.sum();
    // print!("total: {sum}");

    // 迭代器定义
    // pub trait Iterator {
    //     type Item;

    //     fn next(&mut self) -> Option<Self::Item>;

    //     // 此处省略了方法的默认实现
    // }

    // 不会消耗当前迭代器的适配器
    let v1 = vec![2, 6, 7, 8];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![3, 7, 8, 9]);
}

#[test]
fn iterator_demo() {
    let v = vec![5, 9, 7, 3];
    let mut v_iter = v.iter();

    assert_eq!(v_iter.next(), Some(&5));
    assert_eq!(v_iter.next(), Some(&9));
    assert_eq!(v_iter.next(), Some(&7));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
