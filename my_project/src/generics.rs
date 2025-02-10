use std::fmt::Display;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn test() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result); // 100

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);//y

    create_and_print::<i64>();
}



/*
T 需要实现 PartialOrd

> 运算符需要 PartialOrd，否则编译器无法知道 T 是否可比较。
T 需要实现 Copy

list[0] 和 item 可能是非 Copy 类型（比如 String）。
Copy 确保 largest = item; 时不会发生所有权转移（否则需要 Clone）。

泛型约束的解释
T: PartialOrd → 使 T 可比较（支持 > 操作）。
T: Copy → 确保 T 是小数据类型（如 i32、char），直接复制，不转移所有权。

*/


/*
T: From<i32> → T 必须能够从 i32 类型转换（即 T 必须实现 From<i32> trait）。
T: Display → T 必须实现 Display trait，这样才能在 println! 中格式化输出。

100.into()：into() 是 From<T> trait 的方法，它会调用 T::from(100) 将 100 转换成 T 类型。
*/
fn create_and_print<T>()
where T: From<i32> + Display {
    let a: T = 100.into(); // 创建了类型为 T 的变量 a，它的初始值由 100 转换而来
    println!("a is: {}", a);
}

/*
✅ 泛型 T 必须满足两个约束：

实现 From<i32> → 确保可以从 i32 转换为 T。
实现 Display → 确保 T 可以被 println! 格式化输出。
✅ 运行时行为

create_and_print::<i64>() → 100.into() 变成 100i64，然后打印 a is: 100。
🚀 这个模式常用于构造泛型值，并确保它可以被转换和显示！
*/
