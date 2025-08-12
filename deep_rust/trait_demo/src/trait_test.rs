pub mod trait_demo {
    use std::f32::consts::PI;

    pub fn display() {
        trait_display();
        trait_static_display();
    }

    // trait可以定义函数
    // 大写Self为类型，小写self为变量
    trait T1 {
        fn method1(self: Self);
        fn method2(self: &Self);
        fn method3(self: &mut Self);
    }
    // 上下特征一致
    trait T2 {
        fn method1(self);
        fn method2(&self);
        fn method3(&mut self);
    }

    trait Shape {
        fn area(&self) -> f32;
        fn area_box(self: Box<Self>) -> f32;
    }

    trait Round {
        fn get_redius(&self) -> f32;
    }

    #[derive(Debug)]
    struct Circle {
        redius: f32,
    }

    // 成员方法
    impl Shape for Circle {
        fn area(&self) -> f32 {
            PI * self.redius * self.redius
        }

        fn area_box(self: Box<Self>) -> f32 {
            PI * self.redius * self.redius
        }
    }

    impl Round for Circle {
        fn get_redius(&self) -> f32 {
            self.redius
        }
    }

    // impl的对象可以是trait
    impl Shape for dyn Round {
        fn area_box(self: Box<Self>) -> f32 {
            PI * self.get_redius() * self.get_redius()
        }

        fn area(&self) -> f32 {
            PI * self.get_redius() * self.get_redius()
        }
    }

    fn trait_display() {
        let c = Circle { redius: 29f32 };
        println!("{}", c.area());

        let c_box = Box::new(Circle { redius: 30f32 });
        println!("{}", c_box.area_box());

        let c_box_t = Box::new(Circle { redius: 30f32 }) as Box<dyn Round>;
        println!("{}", c_box_t.area_box());
    }

    fn trait_static_display() {
        let st = St(29);
        St::func(&st);
    }

    struct St(i32);

    // 静态方法
    impl St {
        fn func(this: &Self) {
            println!("value: {}", this.0);
        }
    }
}
