pub mod base {
    use std::{i32, io};

    // 常量命名一般采用大写
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    pub fn display() {
        // variable_mutable();
        // data_type();
        // complex_type();
        another_function(15);
        test_expressipns();
        control_floor();
    }

    fn variable_mutable() {
        {
            const _PI: f32 = 3.1415926;
        }

        let mut x = 5;

        println!("这个变量是: {x}");
        x = 6;
        println!("修改后变量是: {x}");

        // 遮蔽,使用原来变量的名字
        let x = x + 1;
        {
            let x = x * 2;
            println!("内部变量为: {x}");
        }
        println!("外部变量为: {x}");

        println!("{THREE_HOURS_IN_SECONDS}");

        // 遮蔽与mut
        {
            let spaces = "       ";
            let spaces = spaces.len();
            println!("len = {spaces}");

            let mut _spaces2 = "     ";
            // spaces2 = spaces2.len();
        }
    }

    fn data_type() {
        let _guess: i32 = "30".parse().expect("not a number!");

        // 整型字面值
        {
            let dec: i32 = 98_222;
            let hex: i32 = 0xff;
            let oct: i32 = 0o77;
            let bin: i32 = 0b0111_0011;
            let by: u8 = b'a';

            println!("dec = {dec}\n hex = {hex}\n oct = {oct}\n bin = {bin}\n by = {by}");
        }
    }

    fn complex_type() {
        // 元组类型
        {
            let tup: (i32, u32, char) = (92, 30, 'a');

            // 解构
            // let (x, y, z) = tup;
            // println!("y = {y}");

            // 也可使用元组索引来使用元组
            let x = tup.0;
            println!("x = {x}");
        }

        // 数组类型
        // 确定数组元素个数时，使用数组
        // 不确定元素个数使用vector
        {
            let months = [
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ];
            println!("{:#?}", months);

            let arr2 = [3; 5];
            println!("{:#?}", arr2);

            let arr = [1, 2, 3, 4, 5];
            println!("请输入数组下标。");

            let mut index = String::new();
            io::stdin().read_line(&mut index).expect("请输入纯数字。");
            let index: usize = index.trim().parse().expect("转换失败。");

            let element = arr[index];

            println!("{element}");
        }
    }

    fn another_function(x: i32) {
        println!("the value x is {x}");
    }

    fn test_expressipns() -> i32 {
        // let x = (let y = 6);
        // let x = let y = 6;
        let x = {
            let y = 6;
            y + 1
        };

        return x;
    }

    fn control_floor() {
        // if表达式
        {
            let number = 5;
            if number > 5 {
                // if number {  // 条件表达式必须是bool类型，其他类型不会自动转换为bool类型
                println!("number grade 5");
            } else {
                println!("number low 5");
            }

            // 可在let语句中使用if表达式
            let st: &str = if number >= 5 {
                "我就是我"
            } else {
                "我黑铁境界"
            };
            println!("{st}");
        }

        // loop循环
        {
            // loop {
            //     println!("jump!!!");
            // }

            // 跳出循环
            let mut counter = 0;
            let result = loop {
                counter += 1;
                if counter == 30 {
                    break counter * 2;
                }
            };
            println!("counter = {result}");
        }

        // 循环标签：在多个循环之间消除歧义
        {
            let mut count = 0;

            'counting_up: loop {
                println!("count = {count}");
                let mut remaining = 10;

                loop {
                    println!("remaining = {remaining}");

                    if remaining == 9 {
                        break;
                    }

                    if count == 2 {
                        break 'counting_up;
                    }

                    remaining -= 1;
                }

                count += 1;
            }
            println!("End count = {count}");
        }
    }
}
