/*
初始化实例时，每个字段都需要进行初始化
初始化时的字段顺序不需要和结构体定义时的顺序一致
*/
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn test() {
    // 让 user1 变成可变的
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 修改 email
    user1.email = String::from("anotheremail@example.com");

    // 这里 user1.username 的所有权被转移给 user2，user1 不能再使用 username
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // 打印 user2
    println!("user2: {:?}", user2);

    // ❌ user1 已经失去了 username 和其他字段的所有权，不能再访问
    // println!("user1: {:?}", user1); // Rust 不允许访问已经被“部分移动”的变量。
    //🔴 Rust 规定：如果结构体的某些字段的所有权被移动了，整个结构体都不能再被使用！ 即使 active 只是 bool 类型，它仍然属于 user1，但因为 user1 的一部分已经被移动了，所以 user1 整体都不能访问。

    // ✅ 你仍然可以手动重新创建 user1 以便继续使用
    let user1 = User {
        email: String::from("recreated@example.com"),
        username: String::from("new_username"),
        active: false,
        sign_in_count: 0,
    };

    println!("user1: {:?}", user1);
}

//怎么解决？
/*
1. clone
let user2 = User {
    email: String::from("another@example.com"),
    username: user1.username.clone(),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};

2.让 username 使用 &str 或 Arc<String> 共享所有权
use std::sync::Arc;

struct User {
    active: bool,
    username: Arc<String>,  // 共享所有权
    email: String,
    sign_in_count: u64,
}

*/




fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

/*

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

*/

/*
还有一个简单的输出 debug 信息的方法，那就是使用 dbg! 宏，它会拿走表达式的所有权，然后打印出相应的文件名、行号等 debug 信息，当然还有我们需要的表达式的求值结果。除此之外，它最终还会把表达式值的所有权返回！

dbg! 输出到标准错误输出 stderr，而 println! 输出到标准输出 stdout。
*/

#[derive(Debug)] // 让结构体支持 `Debug`
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

/*
$ cargo run
[src/main.rs:10] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}

*/