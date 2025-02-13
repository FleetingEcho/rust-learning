// Drop 释放资源——Rust 的自动内存管理机制

/*
1. 什么是 Drop？
Drop 是 Rust 自动资源管理 的核心特征，它允许在变量 超出作用域时 运行一段特定的代码，自动执行清理任务。Rust 会在编译时帮我们 插入 这段收尾代码，确保资源得到正确释放。

在智能指针、文件操作、网络连接等场景，Drop 特别有用。
*/

// 2. Drop 的基本用法
// 来看一个简单的例子：

struct Resource;

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Resource is being dropped!");
    }
}

fn main() {
    let _r = Resource; // `_r` 作用域结束时，会自动调用 `drop`
    println!("Main function is running...");
}
// 输出
// Main function is running...
// Resource is being dropped!
// 解释：

// _r 变量在 main 结束时会超出作用域
// Rust 自动调用 drop 方法，打印 "Resource is being dropped!"
// Rust 保证每个值都会被正确释放，不会有内存泄漏。



// 3. Drop 的执行顺序
// 多个变量的 Drop 顺序
// 来看一个例子：

struct A;
struct B;

impl Drop for A {
    fn drop(&mut self) {
        println!("Dropping A");
    }
}

impl Drop for B {
    fn drop(&mut self) {
        println!("Dropping B");
    }
}

fn main() {
    let _a = A;
    let _b = B;
    println!("Main is running...");
}
// 输出
// Main is running...
// Dropping B
// Dropping A
// 规律：

// 变量的 Drop 顺序是 "创建顺序的逆序"：
// _a 先创建，后释放
// _b 后创建，先释放
// 结构体内字段的 Drop 顺序
struct HasDrop1;
struct HasDrop2;

impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HasDrop1");
    }
}

impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Dropping HasDrop2");
    }
}

struct Container {
    field1: HasDrop1,
    field2: HasDrop2,
}

fn main() {
    let _c = Container {
        field1: HasDrop1,
        field2: HasDrop2,
    };
    println!("Main is running...");
}
// 输出

// Main is running...
// Dropping HasDrop1
// Dropping HasDrop2
// 规律：

// 结构体的字段按定义顺序 Drop
// field1 先创建，先释放
// field2 后创建，后释放
// 如果 Container 结构体自己实现了 Drop，它会 先被 Drop，然后才轮到字段：

impl Drop for Container {
    fn drop(&mut self) {
        println!("Dropping Container");
    }
}
// 输出

// Main is running...
// Dropping Container
// Dropping HasDrop1
// Dropping HasDrop2



// 4. 手动提前 Drop
// 有时候，我们希望 在变量作用域结束前 释放它占用的资源。例如：

// 释放文件锁，避免阻塞
// 关闭网络连接，让其他代码及时访问
// 错误的做法
// Rust 不允许手动调用 drop()：

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!")
    }
}

fn main() {
    let foo = Foo;
    foo.drop(); // ❌ 直接调用 Drop 会报错
}
// 错误信息
// error[E0040]: explicit use of destructor method
// Rust 禁止手动调用 drop()，因为：

// Rust 需要 确保 drop 只被调用一次，手动调用可能导致二次释放错误
// 安全性问题：如果 drop() 被手动调用，Rust 可能仍然认为变量有效，可能引发未定义行为


// 正确的做法
// 使用 std::mem::drop() 手动释放资源：

fn main() {
    let foo = Foo;
    drop(foo); // ✅ 释放 `foo`
    println!("Running!"); // ✅ `foo` 已被释放，无法再使用
}
// std::mem::drop() 会拿走变量的所有权，所以 foo 在 drop(foo); 之后就不能再使用。

// 5. Drop 的应用场景
// 1. 释放文件或数据库连接
// use std::fs::File;

// struct FileHandler {
//     file: File,
// }

// impl Drop for FileHandler {
//     fn drop(&mut self) {
//         println!("Closing file!");
//     }
// }

// fn main() {
//     let _handler = FileHandler {
//         file: File::create("test.txt").unwrap(),
//     };
// } // `_handler` 作用域结束时，文件会自动关闭


// 2. 释放锁
// use std::sync::{Mutex, Arc};
// use std::thread;

// fn main() {
//     let data = Arc::new(Mutex::new(0));

//     let data_clone = Arc::clone(&data);
//     let handle = thread::spawn(move || {
//         let mut lock = data_clone.lock().unwrap();
//         *lock += 1; // ✅ 操作完成后 `lock` 自动释放
//     });

//     handle.join().unwrap();
//     println!("Final value: {:?}", *data.lock().unwrap());
// }
// MutexGuard 在离开作用域时会自动释放锁，避免死锁问题。

// 3. 解决内存泄漏
// Drop 可以确保所有资源被正确释放，即使程序发生 panic 也不会泄漏：

struct LeakGuard;

impl Drop for LeakGuard {
    fn drop(&mut self) {
        println!("Cleaning up before panic!");
    }
}

fn main() {
    let _guard = LeakGuard;
    panic!("Unexpected error!"); // `LeakGuard` 仍然会被 Drop
}
// 即使发生 panic!，Rust 仍然会调用 Drop 释放 _guard。

// 6. Drop vs Copy
// Rust 不允许 结构体同时实现 Copy 和 Drop：

// #[derive(Copy)] // ❌ 报错
// struct Foo;

// impl Drop for Foo {
//     fn drop(&mut self) {
//         println!("Dropping Foo!");
//     }
// }
// 原因
// Copy 意味着数据会被隐式复制，Rust 无法预测 drop 何时会执行
// Drop 意味着数据有明确的生命周期，它的资源管理不能被复制
// 如果一个类型实现了 Drop，Rust 不会自动实现 Copy，你必须手动使用 Clone：

// Rust 不允许 结构体同时实现 Copy 和 Drop：

#[derive(Clone)]
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!");
    }
}

fn main() {
    let a = Foo;
    let _b = a.clone(); // ✅ 手动 `clone`，不会影响 `drop`
}
// 7. 总结
// Drop 让 Rust 自动释放资源

// 变量 超出作用域 时，Rust 自动调用 drop 释放资源
// 适用于 文件、数据库连接、锁、内存管理
// 变量的 Drop 顺序

// 作用域内的变量：按创建顺序的逆序 Drop
// 结构体字段：按定义顺序 Drop
// 实现了 Drop 的结构体：先 drop 结构体，再 drop 字段
// 手动 Drop

// 不能手动调用 drop()
// 使用 std::mem::drop(x) 提前释放资源
// Drop 和 Copy 互斥

// 实现 Drop 的类型不能 Copy
// 可以使用 Clone 代替 Copy
// Drop 是 Rust 内存安全 和 资源管理 的核心机制之一，让 Rust 既无需 GC，也不需要手动回收内存，确保资源管理简单、高效、安全！