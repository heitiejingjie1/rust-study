pub mod owner {
    pub fn display() {
        scope();
        ownership_function();
    }

    // 所用权规则
    // 1、Rust 中的每一个值都有一个 所有者（owner）。
    // 2、值在任一时刻有且只有一个所有者。
    // 3、当所有者离开作用域，这个值将被丢弃。
    fn scope() {
        // 作用域
        {
            let mut s = String::from("hello");
            s.push_str(", world!");
            println!("{s}");

            // 浅拷贝
            // let s2 = s;

            // println!("{s}");
            // println!("{s2}");

            // 克隆操作，深拷贝
            let s2 = s.clone();
            println!("{s}");
            println!("{s2}");
        }
    }

    fn ownership_function() {
        let s = String::from("hello");
        takes_ownership(s);
        // s作用域在这里释放
        // println!("{s}");

        let x = 5; // 实现了copy特性
        makes_copy(x);
        println!("{x}");

        let s_take = String::from("hello,heitiejingjie");
        let s2 = takes_and_gives_ownership(s_take);
        // println!("{s_take}");
        println!("{s2}");
    }

    fn takes_ownership(some_string: String) {
        println!("{some_string}");
    }

    fn makes_copy(some_integer: i32) {
        println!("{some_integer}");
    }

    fn takes_and_gives_ownership(some_string: String) -> String {
        println!("{some_string}");
        some_string
    }
}
