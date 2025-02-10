enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move{x:1,y:1};
    let m3 = Message::ChangeColor(255,255,0);
}

//Rust 中 的 null Option 枚举变量来表述这种结果。
//Rust 强制你在使用 Option<T> 之前先处理 None 的情况，这避免了 空指针错误（null pointer error）。


fn get_username(id: u32) -> Option<String> {
    if id == 1 {
        Some("Rust".to_string()) // 有用户名
    } else {
        None // 没有用户名
    }
}

fn main() {
    let user = get_username(1);
    match user {
        Some(name) => println!("Username: {}", name),
        None => println!("No user found"), // 处理 None
    }
    //✅ 好处： Rust 强制 你处理 None，不会出现 null 访问导致的崩溃。

    // 如果确定 Option 一定有值，可以用 .unwrap() 直接获取：，⚠️ 危险！ 如果 None.unwrap()，程序会直接崩溃：
    // let username = Some("Olivia".to_string());
    // println!("{}", username.unwrap()); // Olivia
    // ✔ 推荐用 .expect() 提供错误信息
// println!("{}", username.expect("Username not found!"));

    // 3️⃣ .unwrap_or() 提供默认值
    let username = None;
    println!("{}", username.unwrap_or("Guest".to_string())); // Guest


    fn first_char(s: Option<&str>) -> Option<char> {
        Some(s?.chars().next()?) // 如果 s 是 None，直接返回 None
    }

    println!("{:?}", first_char(Some("hello"))); // Some('h')
    println!("{:?}", first_char(None)); // None

}
