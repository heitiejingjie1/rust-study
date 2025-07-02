fn main() {
    // 创建一个实例
    let circle = Circle::new(0.0, 0.0, 0.9);
    // 调用方法
    println!("Circle area: {}", circle.area());

    // 访问字段
    circle.x; // 访问x字段
    // 访问方法
    circle.radius(); // 调用radius方法
    circle.radius; // 访问radius字段

    // 枚举的使用
    let msg = Message::Move { x: 10, y: 20 };
    // 调用枚举的方法
    msg.call();
}


#[warn(dead_code)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// 实现方法
// 将数据与方法分开， 
// 通过impl关键字为Circle结构体实现方法
impl Circle {
    // 创建一个新的Circle实例
    // 关联函数
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x, y, radius }
    }

    // Circle的方法, &self表示借用当前的Circle结构体
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    // 允许方法名与字段名相同
    fn radius(&self) -> f64 {
        self.radius
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 为枚举实现方法
impl Message {
    fn call(&self) {
        // 方法体
        println!("Message: {:?}", self);
    }
}
