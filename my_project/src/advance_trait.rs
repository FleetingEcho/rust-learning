//Rust 关联类型 (Associated Types)

/*
关联类型 减少泛型参数的使用，提高可读性。
关联类型 必须在 impl 里具体化，而泛型可以保持灵活性。
适用于 每个实现都必须绑定特定类型 的场景。
*/

trait Container {
    type A;
    type B;

    fn contains(&self, a: &Self::A, b: &Self::B) -> bool;
}

// 关联类型方式
fn difference<C: Container>(container: &C) -> i32 {
    42
}

/*
使用泛型的话， 写很多泛型

struct NumberContainer<T, U> {
    item1: T,
    item2: U,
}

impl<T: PartialEq, U: PartialEq> Container<T, U> for NumberContainer<T, U> {
    fn contains(&self, a: T, b: U) -> bool {
        self.item1 == a && self.item2 == b
    }
}

fn main() {
    let container = NumberContainer { item1: 10, item2: 20 };
    println!("{}", container.contains(10, 20)); // true
}

泛型适用于灵活的类型适配（如 Container<A, B>）。
关联类型适用于特定的类型约束（如 type A; type B;）。
如果 impl 需要绑定具体类型，关联类型比泛型更直观，提升可读性。
*/

struct NumberContainer {
    item1: i32,
    item2: i32,
}
// Container 不需要泛型参数，提升可读性。
// type A = i32; 明确规定了 A 和 B 的具体类型
impl Container for NumberContainer {
    type A = i32;
    type B = i32;

    fn contains(&self, a: &Self::A, b: &Self::B) -> bool {
        self.item1 == *a && self.item2 == *b
    }
}

fn main() {
    let container = NumberContainer { item1: 10, item2: 20 };
    println!("{}", container.contains(&10, &20)); // true
}

//默认泛型类型

struct Container<T = String> {
    value: T,
}

fn main() {
    let a = Container { value: "Hello".to_string() }; // 默认是 String
    let b = Container::<i32> { value: 42 }; // 显式指定为 i32

    println!("{}", a.value); // 输出: Hello
    println!("{}", b.value); // 输出: 42
}


//=========================================
// 调用同名的方法
// 如果都实现了fly方法

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;
    Pilot::fly(&person); // 调用Pilot特征上的方法
    Wizard::fly(&person); // 调用Wizard特征上的方法
    person.fly(); // 调用Human类型自身的方法
}


//完全限定语法
/*
🌟 为什么需要完全限定语法？
当以下情况发生时，我们可能会遇到方法调用的歧义：

多个特征（Trait）提供了相同的方法名。
特征方法与结构体的方法名称相同。
特征方法与 impl 里的方法重名。
在这些情况下，Rust 无法自动推导 你想调用的具体方法，因此需要用完全限定语法来消除歧义。
*/

trait A {
    fn hello(&self);
}

trait B {
    fn hello(&self);
}

struct MyStruct;

impl A for MyStruct {
    fn hello(&self) {
        println!("Hello from A!");
    }
}

impl B for MyStruct {
    fn hello(&self) {
        println!("Hello from B!");
    }
}

fn main() {
    let obj = MyStruct;

    // obj.hello(); // ❌ 编译错误：方法调用存在歧义

    // 解决歧义：使用完全限定语法
    <MyStruct as A>::hello(&obj); // ✅ 输出：Hello from A!
    <MyStruct as B>::hello(&obj); // ✅ 输出：Hello from B!
}



//结构体

trait Greet {
    fn hello(&self);
}

struct Person;

impl Person {
    fn hello(&self) {
        println!("Hello from struct!");
    }
}

impl Greet for Person {
    fn hello(&self) {
        println!("Hello from trait!");
    }
}

fn main() {
    let p = Person;

    p.hello(); // ✅ 默认调用结构体的方法，输出：Hello from struct!

    // 调用特征的方法
    <Person as Greet>::hello(&p); // ✅ 输出：Hello from trait!
}


//泛型中的完全限定语法

trait Speak {
    fn talk();
}

trait Shout {
    fn talk();
}

struct Dog;

impl Speak for Dog {
    fn talk() {
        println!("Dog says: Woof!");
    }
}

impl Shout for Dog {
    fn talk() {
        println!("Dog shouts: WOOF!");
    }
}

// 泛型约束
fn make_noise<T: Speak + Shout>() {
    // <T>::talk(); // ❌ Rust 无法推断调用哪个 talk()

    // 使用完全限定语法
    <T as Speak>::talk(); // ✅ 调用 Speak 版本
    <T as Shout>::talk(); // ✅ 调用 Shout 版本
}

fn main() {
    make_noise::<Dog>(); // ✅ 输出 Woof! 和 WOOF!
}



/*
孤儿规则

孤儿规则，简单来说，就是特征或者类型必需至少有一个是本地的，才能在此类型上定义特征。
 Newtype 模式的作用：

绕过孤儿规则，允许在 MyString 上实现 Display。
防止与标准库冲突，避免对 String 进行不受控的修改。
*/


use std::fmt;

// 1️⃣ 定义 Newtype 结构体，封装 String
struct MyString(String);

// 2️⃣ 为 MyString 实现 Display 特征
impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom String: {}", self.0) // 访问内部 `String`
    }
}

fn main() {
    let s = MyString("Hello, world!".to_string());
    println!("{}", s); // ✅ 输出：Custom String: Hello, world!
}


/*
✅ 为什么需要 Newtype 模式？

绕过孤儿规则，允许在外部类型上实现外部特征（如 Display）。
限制访问，隐藏原始类型的方法，只暴露需要的方法。
扩展功能，添加额外的方法，如 shout()。
提高类型安全性，区分 UserId(u32) 和 OrderId(u32)。
✅ 如何使用？

定义 Newtype
struct MyType(OriginalType);
实现外部特征
impl Display for MyType { ... }

*/







