fn main() {
    traits_demo();
}

fn traits_demo() {
    // 可以在 Rust 裡定義一些行為，然後將這些行為套用到不同的類別上
    let kitty = Cat {
        name: String::from("Kitty"),
        age: 3,
    };

    kitty.fly();

    let wawa = Dog {
        name: String::from("Wawa"),
    };

    wawa.fly(); // 使用默认实现
    //
    bungee(&kitty);

    let post = Post{
        title: "first".to_string(),
        author: "2025.7.2".to_string(),
        content: "我就是我".to_string()
    };

    println!("{}", post.summarize());
}

// 定義一個 Trait 
trait Flyable {
    fn fly(&self) {
        // Trait 可以有预设实现
        // 如果没有提供具体实现，则会使用默认实现
        print!("Default fly behavior");
    }  // 只需定义函数签名就行了
    
}

#[derive(Debug)]
struct Cat {
    name: String,
    age: u32,
}

#[derive(Debug)]
struct Dog {
    name: String,
}
impl Flyable for Dog {}

// 挂载 Trait 到 Cat
impl Flyable for Cat {
    fn fly(&self) {
        println!("{} is flying!", self.name);
    }
}

// traits 实现多太
fn bungee(someone: &dyn Flyable) {
    // 只要 someone 实现了 Flyable trait，就可以调用 fly 方法
    someone.fly();
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post{
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post{
    fn summarize(&self) -> String {
        format!("标题是:{}, 作者是: {},内容是: {} ", self.title, self.author, self.content)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("用户是: {}, 内容是: {}", self.username, self.content)
    }
}
