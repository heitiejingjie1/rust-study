use std::net::IpAddr;

fn main() {
    match_demo();
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

    // 使用match来匹配IP地址类型
    let ip = IPAddr::IPV6;
    let ip_str = match ip {
        IPAddr::IPV4 => "127.0.0.1",
        _ => "::1",
    };
    println!("IP address is: {}", ip_str);
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
