/*

智能指针往往是基于结构体实现，它与我们自定义的结构体最大的区别在于它实现了 Deref 和 Drop 特征：

Deref 可以让智能指针像引用那样工作，这样你就可以写出同时支持智能指针和引用的代码，例如 *T
Drop 允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作
智能指针在 Rust 中很常见，我们在本章不会全部讲解，而是挑选几个最常用、最有代表性的进行讲解：

Box<T>，可以将值分配到堆上
Rc<T>，引用计数类型，允许多所有权存在
Ref<T> 和 RefMut<T>，允许将借用规则检查从编译期移动到运行期进行

*/

/*
Box<T> —— 把数据放进堆里的智能指针
什么是 Box<T>？
Box<T> 是 Rust 最常见的智能指针，它的作用很简单：把一个值存到堆上，然后在栈上保存一个指向它的指针。

在 Rust 里，我们平时创建的变量默认是存储在 栈 上的。但是栈的大小是有限的，比如 Rust 里主线程的栈大小是 8MB，普通线程是 2MB。如果我们要存放比较大的数据，或者数据的大小在编译时无法确定，就需要把数据放到 堆 上，而 Box<T> 就是帮我们做到这一点的工具。

栈和堆的区别
栈：

从高地址往低地址增长
申请和释放都很快，类似“进出栈”操作
存储的是固定大小的数据
作用域结束时，Rust 会自动释放栈上的数据


堆：

从低地址往高地址增长
申请和释放比栈慢，需要管理指针
可以存储动态大小的数据
只要有指针指向，数据就不会被释放
在 Rust 里，所有放在 堆 上的数据都有一个“所有者”，当所有者被销毁时，对应的堆数据也会被释放。

Box<T> 的基本用法
如果我们直接创建一个整数变量，它默认是存储在栈上的：

fn main() {
    let a = 3; // a 存在栈上
}


但如果我们希望这个值存储在 堆 上，可以使用 Box<T>：
fn main() {
    let a = Box::new(3); // a 存在堆上，a 变量指向这个值
    println!("a = {}", a);
}
在这个例子中：

Box::new(3) 创建了一个 堆分配 的整数 3，然后返回一个 智能指针 a，指向这个值
a 持有 这个智能指针，意味着 a 作用域结束时，堆上的数据也会被自动释放
为什么 let b = a + 1 会报错？

fn main() {
    let a = Box::new(3);
    let b = a + 1; // ❌ 报错：无法对 Box 进行算术运算
}


Box<T> 是个智能指针，不能直接参与数学运算。
如果我们想对 Box<T> 里的值进行运算，需要手动 解引用：


fn main() {
    let a = Box::new(3);
    let b = *a + 1; // ✅ 正确
    println!("b = {}", b);
}



避免栈上的数据拷贝
如果数据存储在 栈上，那么每次转移所有权都会拷贝数据：


fn main() {
    let arr = [0; 1000]; // 在栈上创建 1000 个元素的数组
    let arr1 = arr; // ❗ 栈上数据被**拷贝**了一份
    println!("{}", arr.len());
    println!("{}", arr1.len());
}

但如果数据存储在 堆上，转移所有权时只会拷贝指针：

fn main() {
    let arr = Box::new([0; 1000]); // 在堆上创建 1000 个元素的数组
    let arr1 = arr; // ✅ 只拷贝了 Box 指针，底层数据**没有被拷贝**
    println!("{}", arr1.len());
}
在堆上存储大块数据时，Box<T> 让所有权转移更高效，不会复制大量数据。

Box<T> 的使用场景
Box<T> 的核心特点是“把数据放到 堆 上”，因此适用于以下情况：

1.存储大数据：避免栈上数据拷贝
2.让动态大小的类型（DST）变成固定大小（Sized）：比如递归类型
3.存储特征对象（Trait Object）：用于存储不同类型的对象

递归类型的存储
Rust 需要在 编译期 知道类型的大小，但递归类型的大小是不确定的：


enum List {
    Cons(i32, List), // ❌ 递归嵌套，大小无法确定
    Nil,
}
这个代码会报错，因为 Rust 不知道 List 需要多少内存。

解决办法：使用 Box<T> 把递归数据存到堆上：


enum List {
    Cons(i32, Box<List>), // ✅ 用 Box 让 List 变成固定大小
    Nil,
}
Box<T> 让递归类型存到 堆 上，确保每个 List 只占用一个固定大小的 Box 指针。

*/
//特征对象
// Rust 不允许不同类型的对象放进同一个数组：

trait Draw {
    fn draw(&self);
}

struct Button { id: u32 }
struct Select { id: u32 }

impl Draw for Button {
    fn draw(&self) { println!("按钮 {}", self.id); }
}

impl Draw for Select {
    fn draw(&self) { println!("选择框 {}", self.id); }
}

fn main() {
    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];
    for e in elems {
        e.draw();
    }
}
/*

这里 Box<dyn Draw> 就是特征对象，它把不同的类型 Button 和 Select 存到了 堆上，然后用智能指针 Box 来指向它们。

Box<T> 内存布局
来看 Vec<i32> 的内存布局：


(stack)    (heap)
┌──────┐   ┌───┐
│ vec1 │──→│ 1 │
└──────┘   ├───┤
           │ 2 │
           ├───┤
           │ 3 │
           ├───┤
           │ 4 │
           └───┘
它的智能指针 vec1 存在 栈 上，而数据存到 堆 里。

如果 Vec 里存的是 Box<i32>：


(stack)    (heap)   (heap)
┌──────┐   ┌───┐ ┌─→│ 1 │
│ vec2 │──→│B1 │─┘  └───┘
└──────┘   ├───┤    ┌───┐
           │B2 │───→│ 2 │
           ├───┤    └───┘
           │B3 │─┐  ┌───┐
           ├───┤ └─→│ 3 │
           │B4 │─┐  └───┘
           └───┘ │  ┌───┐
                 └─→│ 4 │
                    └───┘
Box 让数据多跳了一次指针，所以取数据时要解引用 **box。


*/

fn main() {
    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + **second;
}

/*


Box::leak
Box::leak 可以让数据在程序整个生命周期内都有效：


fn main() {
    let s = gen_static_str();
    println!("{}", s);
}

fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello, world");
    Box::leak(s.into_boxed_str())
}
这个函数返回了 'static 生命周期的 &str，可以作为全局数据使用。
那么我说一个简单的场景，你需要一个在运行期初始化的值，但是可以全局有效，也就是和整个程序活得一样久，那么就可以使用 Box::leak，例如有一个存储配置的结构体实例，它是在运行期动态插入内容，那么就可以将其转为全局有效，虽然 Rc/Arc 也可以实现此功能，但是 Box::leak 是性能最高的。



总结
Box<T> 把数据存到堆上
适用于大数据、递归类型、特征对象
堆上的数据所有权转移更高效
可用 Box::leak 让数据全局有效
Rust 通过 Box<T> 实现类似 GC 语言的对象管理
Box<T> 让 Rust 既能享受手动管理内存的高性能，又能拥有智能指针的便利性。
*/