fn main() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
}

// 4 字节

fn main2() {
    let t = true;
    let f: bool = false; // 使用类型标注,显式指定f的类型
    if f {
        println!("这是段毫无意义的代码");
    }
}

macro_rules! my_print {
    ($msg:expr) => {
        println!(">>> {}", $msg);
    };
}

fn main3() {
    my_print!("Hello Rust!"); // 输出 >>> Hello Rust!
}



/*
println! 是 宏（macro），不是函数，所以需要 !。
宏 在编译期展开，支持 可变参数 和 格式化解析，比普通函数更灵活。
*/