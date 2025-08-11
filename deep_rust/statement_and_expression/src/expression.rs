pub mod expression {
    pub fn display() {
        statement_chunk_expression();
        if_else_demo();
        loop_demo();
        while_demo();
        for_demo();
    }

    fn statement_chunk_expression() {
        // 语句块表达式
        let x: () = {
            println!("hello");
        };

        // 表达式不加";", 就是返回该表达式的值
        let y: i32 = {
            println!("hello");
            29
        };

        println!("{:?}", x);
        println!("{:?}", y);
    }

    fn if_else_demo() {
        let x = 10;
        if x < 10 {
            println!("{x} less ten");
        } else if x > 10 {
            println!("{x} largset ten");
        } else {
            println!("{x} is ten");
        }

        // 将if_else结构当表达式使用
        let y = if x < 5 { 29 } else { 55 };
        println!("{y}");
    }

    fn loop_demo() {
        // 死循环
        let mut count = 0;
        println!("Let's count until infinity!");

        loop {
            count += 1;
            if count == 3 {
                println!("Three");
                continue; // 返回到循环开头
            }

            println!("{}", count);
            if count == 5 {
                println!("OK, that's enough");
                break; // 跳出循环
            }
        }
    }

    fn while_demo() {
        let mut x = 1;

        while x < 101 {
            if x % 15 == 0 {
                println!("fizzbuzz");
            } else if x % 5 == 0 {
                println!("fizz");
            } else if x % 3 == 0 {
                println!("buzz");
            } else {
                println!("{}", x);
            }

            x += 1;
        }
    }

    fn for_demo() {
        // for循环
        for number in 0..15 {
            println!("The number is {number}");
        }
    }
}
