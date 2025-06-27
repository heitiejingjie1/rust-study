fn main() {
    enum_demo();
    option_demo();
}

#[derive(Debug)]
enum PokerCard {
    Clubs(u8),
    Speades(u8),
    Diamonds(char),
    Hearts(char),
}

fn enum_demo() {
    let card1 = PokerCard::Clubs(6);
    let card2 = PokerCard::Hearts('J');

    println!("Card 1: {:?}", card1);
    println!("Card 2: {:?}", card2);

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 20, y: 30 };
    let m3 = Message::ChangeColor(255, 255, 0);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn option_demo() {
    // option 枚举用于处理空值
    /*
     * Option<T> 是一个枚举类型，用于表示一个值可能存在或不存在。
     * 它有两个变体：
     * * - Some(T): 包含一个值 T
     * * - None: 不包含值
     * */
    let some_number = Some(69);
    let some_string = Some("fengmuxia");

    let absont_number: Option<i32> = None;
    let x: i32 = 70;

    let sum = some_number + x;
}
