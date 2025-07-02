fn main() {
    match_demo();
    match_banding();
    if_let_demo();
    match_sense();
}

#[derive(Debug)]
enum Direction {
    East,
    South,
    West,
    North,
}

fn match_demo() {
    let direction = Direction::West;
    // match的匹配必须穷举出所有可能的情况,其他情况可用"_"来匹配
    match direction {
        Direction::East => println!("East!"),
        Direction::South | Direction::West => println!("South or West!"),
        _ => println!("North"),
    };

    let count = value_of_count(Dice::Six);
    println!("Count is: {}", count);
    let ip = IPAddr::IPV6;
    let ip_str = match ip {
        IPAddr::IPV4 => "127.0.0.1",
        _ => "::1",
    };
    println!("IP address is: {}", ip_str);

    let six = match_option(Some(6));
    println!("Matched option is: {:?}", six);
}

#[derive(Debug)]
enum Dice {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

// 根据骰子的值返回对应的计数
fn value_of_count(dice: Dice) -> u32 {
    match dice {
        Dice::One => 1,
        Dice::Two => 2,
        Dice::Three => 3,
        Dice::Four => 4,
        Dice::Five => 5,
        Dice::Six => {
            println!("lucky count!");
            6
        }
    }
}

#[derive(Debug)]
enum IPAddr {
    IPV4,
    IPV6,
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u8, u8, u8),
}

// 模式绑定
fn match_banding() {
    let actions = [
        Action::Say("hello rust".to_string()),
        Action::MoveTo(10, 20),
        Action::ChangeColorRGB(255, 0, 0),
    ];

    for action in actions {
        match action {
            Action::Say(message) => println!("action is say: {} ", message),
            Action::MoveTo(x, y) => println!("action is move to: ({}, {})", x, y),
            Action::ChangeColorRGB(r, g, b) => {
                println!("action is change color to: RGB({}, {}, {})", r, g, b);
            }
        }
    }
}

// if let 匹配
// 只匹配一个条件就使用if let, 匹配多个就使用match
fn if_let_demo() {
    let v = Some(3u8);

    if let Some(3) = v {
        println!("Matched 3!");
    } else {
        println!("Not matched 3!");
    }
}

// if let 也可以用于解构
fn match_option(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i),
    }
}

fn match_sense() {
    /*
     * match 语句
     */
    //  match VALUE {
    //      PATTERN => EXPRESSION,
    //      PATTERN => EXPRESSION,
    //      PATTERN => EXPRESSION,
    //      _ => DEFAULT_EXPRESSION, // 可选的默认匹配
    //  }

    /*
     * if let 语句用于匹配一个特定的模式，如果匹配成功，则执行相应的代码块。它通常用于简化代码，避免使用完整的 match 语句。*/
    //  if let PATTERN = VALUE {
    //      // 执行代码
    //  }

    /*
     * while let 语句用于在循环中匹配一个特定的模式，直到不再匹配为止。它通常用于处理迭代器或其他可重复的操作。
     * */
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }

    /*
     * for 循环中的模式匹配
     * 在 for 循环中，可以使用模式匹配来解构迭代器中的元素。
     * */
    let v = vec![(1, 2), (3, 4), (5, 6)];
    for (x, y) in v.iter().enumerate() {
        println!("Index: {}, Value: ({}, {})", x, y.0, y.1);
    }

    /*
     * let语句也是一种模式匹配的方式，可以用于解构结构体、元组等数据类型。
     * */

    /*
     * 函数参数也是一种模式匹配的方式，可以在函数定义中直接解构参数。
     * */
}
