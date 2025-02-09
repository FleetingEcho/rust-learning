pub fn const_test(name: &str) {
   let mut x = name;// 可变
    println!("The value of x is: {}", x);
    x = "test";// 数据类型不可变
    println!("The value of x is: {}", x);
    destructuring_assignment();
    my_const();
    variable_shadowing()
}


struct Struct {
    e: i32
}

fn destructuring_assignment() {
    let (a, b) = (1, 2);
    let [c, .., d, _] = [1, 2, 3, 4, 5];
    let Struct { e, .. } = Struct { e: 5 };
    println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

const MAX_POINTS: u32 = 100_000;


fn my_const(){
  println!("const_value={}",MAX_POINTS)
}

fn variable_shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}


/*
变量遮蔽  两个都会存在
pub fn const_test(name: &str) {
    let x = name; // x 是 &str
    println!("The value of x is: {}", x);

    let x = 6; // 重新定义 x，类型变为 i32
    println!("The value of x is: {}", x);
}

fn main() {
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}


let number = 42;
let text = number.to_string();
println!("{}", text); // 输出 "42"



如果必须要任意类型
use std::any::Any;

fn main() {
    let mut x: Box<dyn Any> = Box::new(42); // 存整数
    x = Box::new(3.14); // 存浮点数
    x = Box::new("Hello Rust".to_string()); // 存字符串

    // 获取值的类型
    if let Some(value) = x.downcast_ref::<String>() {
        println!("String value: {}", value);
    }
}



或者

enum AnyType {
    Int(i32),
    Float(f64),
    Str(String),
}

fn main() {
    let mut x = AnyType::Int(42);
    println!("{:?}", x);

    x = AnyType::Float(3.14);
    println!("{:?}", x);

    x = AnyType::Str("Hello Rust!".to_string());
    println!("{:?}", x);
}


*/