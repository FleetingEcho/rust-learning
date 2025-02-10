fn main() {
    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn main() {
    for i in 1..=5 {
        println!("{}", i);
    }
}

/*
使用方法	等价使用方式	所有权
for item in collection	for item in IntoIterator::into_iter(collection)	转移所有权
for item in &collection	for item in collection.iter()	不可变借用
for item in &mut collection	for item in collection.iter_mut()	可变借用
*/


fn main() {
    let mut n = 0;

    while n <= 5  {
        println!("{}!", n);

        n = n + 1;
    }

    println!("我出来了！");
}

fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}


fn main() {
    let mut counter = 0;

    let result = loop { //loop 是一个表达式，因此可以返回一个值
        counter += 1;

        if counter == 10 {
            break counter * 2;//break 可以单独使用，也可以带一个返回值，有些类似 return
        }
    };

    println!("The result is {}", result);
}

