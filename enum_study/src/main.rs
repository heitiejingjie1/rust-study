fn main() {
    println!("Hello, world!");
    enum_example();
}

#[derive(Debug)]
enum PokerCard {
    Clubs(u8),
    Diamonds(u8),
    Hearts(char),
    Spades(char),
}

fn enum_example() {
    let card1 = PokerCard::Clubs(9);
    let card2 = PokerCard::Spades('K');
}
