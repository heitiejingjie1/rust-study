fn main() {
    user_demo();
}

#[derive(Debug)] // 需要派生 Debug 特征才能打印结构体
struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u32,
}

fn user_demo() {
    let user1 = create_user("fengmuxia".to_string(), "xxxxmksh@git.com".to_string());
    println!("user1: {:?}", user1);

    // 结构体更新语法
    let user2 = User {
        name: "jiushiwo".to_string(),
        ..user1 // 使用 user1 的其他字段
    };

    // println!("user1: {:?}", user1);
    // user1 email字段所有权转移到user2，所以user1无法在被使用，但copy特征字段可被使用
    println!("user1.active: {}", user1.active);
}

fn create_user(name: String, email: String) -> User {
    User {
        active: true,
        name,
        email,
        sign_in_count: 1,
    }
}

// 元组结构体，结构体的字段没有名称
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 单元结构体，没有任何字段的结构体
struct UnitStruct;
