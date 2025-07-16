pub mod box_test {
    /*
     * 使用情况:
     *   当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
     *   当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
     *   当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
     * */

    use std::{collections, ops::Deref};

    pub fn test() {
        box_demo();
        recurrence_type();
        deref_type();
    }

    pub fn box_demo() {
        // 在堆上创建数据
        let b = Box::new(9);
        println!("b = {b}");
    }
    pub fn recurrence_type() {
        let list = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );

        println!("{:?}", list);
    }

    // box允许创建递归类型
    // 递归类型: 递归类型的值可以拥有另一个同类型的值作为其自身的一部分
    #[derive(Debug)]
    pub enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    pub fn deref_type() {
        // 一般指针
        let x = 5;
        let y = &x;
        let z = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
        // 想一般引用使用智能指针
        assert_eq!(5, *z);
    }

    // 自定义智能指针
    pub struct MyBox<T>(T);

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}
