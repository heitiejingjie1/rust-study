pub mod trait_type {
    use std::fmt::{Debug, Display};

    pub fn test_trait() {
        let post = SocialPost {
            username: "fengmuxia".to_string(),
            content: "今天很伤心.".to_string(),
            reply: false,
            repost: false,
        };

        println!("1 new post {}", post.summarize());

        let new_article = NewArticle {
            headline: "first line".to_string(),
            location: "second".to_string(),
            author: "fengmuxia".to_string(),
            content: "我就是我".to_string(),
        };

        println!("1 new article {}", new_article.summarize());

        notify(&post);
        notify(&new_article);
        returns_summarizable(true);
    }

    // 将trait作为参数
    // trait bound 语法糖
    pub fn notify(item: &impl Summary) {
        println!("Breaking new! {}.", item.summarize());
    }

    // trait bound语法
    pub fn notify2<T: Summary>(item: &T) {
        println!("Breaking new! {}", item.summarize());
    }

    // bound 多个trait
    pub fn some_function<T: Summary + Display, U: Summary + Clone>(item: &T, item2: &U) {}
    // 通过where从句简化多个trait
    pub fn some_function2<T, U>(item: &T, item2: &U)
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
    }

    // 返回实现了trait类型
    pub fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
        if switch {
            Box::new(SocialPost {
                username: "heitiejingjie".to_string(),
                content: "好累啊".to_string(),
                reply: true,
                repost: false,
            })
        } else {
            Box::new(NewArticle {
                headline: "first lien".to_string(),
                location: "/home/user/".to_string(),
                author: "fengmuxia".to_string(),
                content: "但很开心".to_string(),
            })
        }
    }

    // 定义trait
    pub trait Summary {
        fn summarie_author(&self) -> String;
        // 默认实现
        fn summarize(&self) -> String {
            format!("Read more from {}...", self.summarie_author())
        }
    }

    #[derive(Debug)]
    pub struct NewArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    #[derive(Debug)]
    pub struct SocialPost {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub repost: bool,
    }

    // 实现trait
    impl Summary for NewArticle {
        // fn summarize(&self) -> String {
        //     format!("{}, by {} {}", self.headline, self.author, self.location)
        // }
        fn summarie_author(&self) -> String {
            format!("@{}", self.author)
        }
    }

    impl Summary for SocialPost {
        fn summarie_author(&self) -> String {
            format!("@{}", self.username)
        }
    }
}
