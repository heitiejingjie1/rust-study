fn main() {
    // string_and_str();
    // str_push();
    str_insert();
}

fn string_and_str() {
    // &str to String
    let str: String = "Hello world!".to_string();
    let str2: String = String::from("hello world");
    println!("str: {}, str2: {}", str, str2);

    // String to &str
    let to_string1 = &str;
    let to_string2 = &str2;
    println!("toString1: {}, toString2: {}", to_string1, to_string2);

    // rust中无法使用字符索引的方式去访问字符串字符
    // 使用字符串切片时要十分小心字符串的字节数
}

fn str_push() {
    let mut s = String::from("fengmuxia");

    // push_str()追加字符串
    s.push_str(", is's me");
    println!("{}", s);
    // push()追加字符
    s.push('!');
    println!("{}", s);
}

fn str_insert() {
    let mut s = String::from("今天又是元气满满的一天");
    let len = s.len();

    // insert()在指定位置插入字符
    s.insert(len - 1, ',');
    println!("{}", s);
}
