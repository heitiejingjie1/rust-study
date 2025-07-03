fn main() {
    life_demo();
}

struct Cat {
    name: String,
    age: u8,
}

fn life_demo () {

    // let age = 29;

    // let my_age = &age;

    // print!("{} is my age", my_age);
    //
    // let my_age;

    // {
    //     let age = 29;
    //     my_age = &age;
    // }

    // print!("{} is my age", my_age);
    //
    let cat1 = Cat {
        name: String::from("Whiskers"),
        age: 3,
    };

    let cat2 = Cat {
        name: String::from("Mittens"),
        age: 5,
    };

    let older_cat = boss_cat(&cat1, &cat2);

    println!("boss cat is {}", older_cat.name);

}

fn boss_cat<'a>(c1: &'a Cat, c2: &'a Cat) -> &'a Cat {
    if c1.age > c2.age {
        c1
    } else {
        c2
    }
}
