fn main() {
    enum_demo();
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
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
