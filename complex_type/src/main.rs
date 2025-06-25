fn main() {
    // slice();
    // string_and_str();
    // operation_string();
    // tuple();
    struct_test();

    enum_test();
}

// 枚举
#[derive(Debug)]
enum PokerSuit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}
// 枚举的实例化
fn enum_test() {
    let clubs = PokerSuit::Clubs;
    let hearts = PokerSuit::Hearts;

    print_suit(clubs);
    print_suit(hearts);
}

fn print_suit(suit: PokerSuit) {
    println!("{:?}", suit);
}

// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 单元结构体
struct UnitStruct;

// 结构体的定义
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 结构体的实例化
fn struct_test() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@git.com"),
        sign_in_count: 1,
    };

    // 访问结构体的字段
    println!("Username: {}", user1.username);

    // 根据已有实例创建新实例
    let user2 = User {
        email: String::from("fengmuxia@git.com"),
        ..user1 // 使用user1的其他字段
    };

    // 利用函数创建结构体实例
    let user3 = build_user(
        String::from("fengmuxia@gmail.com"),
        String::from("fengmuxia"),
    );
    println!("{:#?}", user3);
    println!("User3: {}, Email: {}", user3.username, user3.email);

    let blank = Color(0, 0, 0);
    let zero = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn tuple() {
    // 元组的定义
    let tuple: (i32, f64, char) = (500, 6.4, 'a');
    // 元组的解构
    let (x, y, z) = tuple;
    println!("x: {}, y: {}, z: {}", x, y, z);
    // 访问元组元素
    println!("First element: {}", tuple.0);
    println!("Second element: {}", tuple.1);
    println!("Third element: {}", tuple.2);

    let str = String::from("Hello, world!");
    let (s, length) = calculate_length(str);
    println!("String: {}, Length: {}", s, length);
}

fn calculate_length(s: String) -> (String, usize) {
    // 计算字符串的长度
    let length = s.len();
    (s, length)
}

fn slice() {
    let s = String::from("Hello, world!");

    let hello = &s[0..6];
    let world = &s[7..12];
    println!("{} {}", hello, world);

    let zn_s = "中国人"; // 该类型为&str
    let zhong = &zn_s[0..3];
    println!("{}", zhong);

    let ch = 's';
    let len = ch.len_utf8();
    println!("The length of '{}' in bytes is {}", ch, len);
}

fn string_and_str() {
    /*
     * &str to string:可以使用String::from()或者to_string()方法
     * */
    let str_to_string = String::from("Hello, world!");
    let str_to_string2 = "Hello, world!".to_string();
    println!("{} {}", str_to_string, str_to_string2);

    /*
     * String to &str:对string类型取引用即可
     */
    let string_to_str: &str = &str_to_string2;
    let string_to_str2: &str = &str_to_string2[..];
    let string_to_str3: &str = &str_to_string.as_str();

    println!("{} {} {}", string_to_str, string_to_str2, string_to_str3);
}

fn operation_string() {
    let mut str = String::from("Hello, world!");

    // 追加(push)
    // push_str()方法用于追加字符串切片
    str.push_str("my name is Rust.");
    // push()方法用于追加单个字符
    str.push('!');
    println!("{}", str);

    // 插入(insert)
    // insert()方法用于在指定位置插入单个字符
    str.insert(str.len() - 1, 'h');
    // insert_str()方法用于在指定位置插入字符串切片
    str.insert_str(str.len() - 1, " Rustacean");
    println!("{}", str);

    // 替换(replaced)
    // replace()方法用于替换字符串中的子字符串，替换所有匹配到的字符串，返回一个新的字符串
    str = str.replace("Rustacean", "Rust");
    println!("{}", str);
    // replacen()方法用于替换字符串中的子字符串，替换指定次数的匹配到的字符串，返回一个新的字符串
    str = str.replacen("Rust", "Rustacean", 2);
    println!("{}", str);
    // replace_range()方法用于替换字符串中的指定范围的子字符串，在指定范围内替换字符串，直接操作原字符串，不生成新的字符串
    str.replace_range(.., "y");
    println!("{}", str);

    // 删除(delete)
    let mut str2 = String::from("my name is Rust!");
    // pop()方法用于删除字符串的最后一个字符，并返回该字符
    println!("Popped character: {:?}", str2.pop());
    // remove()方法用于删除字符串中指定位置的字符，按字节索引删除，如果索引超出范围，则会导致panic
    str2.remove(0); // 删除第一个字符
    str2.remove(str2.len() - 1); // 删除最后一个字符
    println!("{}", str2);
    // truncate()方法用于截断字符串到指定长度，超过长度的部分将被删除，指定位置到结束的字符，按字节处理，超出范围会导致panic
    str2.truncate(5);
    println!("{}", str2);
    // clear()方法用于清空字符串，将其长度设置为0
    str2.clear();
    println!("After clear: '{}'", str2);

    // 连接字符串(concataenate)
    // +或+=运算符用于连接两个字符串切片或字符串，右边的字符串必须是&str类型
    let str3 = String::from("Hello, ");
    let str4 = String::from("world!");
    let result = str3 + &str4; // 注意：str3被移动，不能再使用
    println!("Concatenated string: {}", result);

    let mut result = result + "!"; // 可以继续连接字符串
    result += " How are you?";
    println!("Concatenated string with +=: {}", result);
    // format!宏用于连接字符串，类似于println!宏，但不会移动原字符串，可以多次使用
    let format = format!("{} {}", str4, "I am fine.");
    println!("Formatted string: {}", format);

    // 遍历字符串
    // 以字符为单位遍历字符串，可以使用chars()方法
    for ch in "风慕夏wojiudhiwo".chars() {
        println!("Character: {}", ch);
    }
    // 以字节为单位遍历字符串，可以使用bytes()方法
    // 注意：字节遍历可能会导致中文字符被拆分成多个字节
    for byte in "风慕夏wojiudhiwo".bytes() {
        println!("Byte: {}", byte);
    }
}
