use std::string;

fn main() {
    slice();
}

fn slice() {
    let s = string::String::from("Hello, world!");

    let hello = &s[0..6];
    let world = &s[7..12];
    println!("{} {}", hello, world);

    let zn_s = "中国人";
    let zhong = &zn_s[0..3];
    println!("{}", zhong);
}
