fn main() {
    if_demo();
}

fn if_demo() {
    let x = 5;
    if x < 10 {
        println!("x is less than 10");
    } else if x == 10 {
        println!("x is equal to 10");
    } else {
        println!("x is greater than 10");
    }

    let condition = true;
    let y = if condition { 10 } else { 20 };
    println!("{}", y);
}
