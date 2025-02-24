## String

# Rust `String` 方法表

| **类别**      | **方法**                                  | **返回类型**          | **是否消耗 `String`** | **说明** |
|--------------|-----------------------------------------|----------------------|-----------------------|---------|
| **创建**     | `String::new()`                        | `String`             | ✅ 是 | 创建一个空 `String` |
|              | `String::from("text")`                 | `String`             | ✅ 是 | 从 `&str` 创建 `String` |
|              | `"text".to_string()`                   | `String`             | ✅ 是 | `&str` 转 `String` |
|              | `String::with_capacity(n)`             | `String`             | ✅ 是 | 预分配 `n` 字节的 `String` |
| **拼接**     | `push(char)`                           | `()`                 | ✅ 是 | 追加单个字符 |
|              | `push_str("text")`                     | `()`                 | ✅ 是 | 追加字符串 |
|              | `+`                                    | `String`             | ✅ 是 | 连接两个 `String`（所有权转移） |
|              | `format!("{} {}", s1, s2)`             | `String`             | ❌ 否 | 格式化连接字符串 |
| **修改**     | `insert(index, char)`                  | `()`                 | ✅ 是 | 在 `index` 位置插入字符 |
|              | `insert_str(index, "text")`            | `()`                 | ✅ 是 | 在 `index` 位置插入字符串 |
|              | `replace("old", "new")`                | `String`             | ❌ 否 | 替换所有匹配的子串（返回新字符串） |
|              | `replacen("old", "new", n)`            | `String`             | ❌ 否 | 替换前 `n` 个匹配项（返回新字符串） |
|              | `replace_range(range, "text")`         | `()`                 | ✅ 是 | 替换指定范围的内容 |
|              | `truncate(n)`                          | `()`                 | ✅ 是 | 截断 `String`，保留前 `n` 个字符 |
|              | `clear()`                              | `()`                 | ✅ 是 | 清空 `String` |
|              | `remove(index)`                        | `char`               | ✅ 是 | 移除 `index` 位置的字符（返回被移除字符） |
|              | `drain(range)`                         | `Drain` (迭代器)     | ✅ 是 | 移除 `range` 内的字符（可收集到 `String`） |
| **查询**     | `len()`                                | `usize`              | ❌ 否 | 获取字符串长度（字节数） |
|              | `is_empty()`                           | `bool`               | ❌ 否 | 是否为空 |
|              | `contains("text")`                    | `bool`               | ❌ 否 | 是否包含子串 |
|              | `starts_with("prefix")`               | `bool`               | ❌ 否 | 是否以 `prefix` 开头 |
|              | `ends_with("suffix")`                 | `bool`               | ❌ 否 | 是否以 `suffix` 结尾 |
|              | `find("text")`                        | `Option<usize>`      | ❌ 否 | 查找子串索引 |
|              | `rfind("text")`                       | `Option<usize>`      | ❌ 否 | 从后向前查找索引 |
|              | `matches("text")`                     | `Matches` (迭代器)   | ❌ 否 | 迭代匹配项 |
| **切割**     | `split("delimiter")`                  | `Split` (迭代器)     | ❌ 否 | 按分隔符拆分 |
|              | `split_whitespace()`                  | `SplitWhitespace`    | ❌ 否 | 按空白字符拆分 |
|              | `splitn(n, "delimiter")`              | `SplitN` (迭代器)    | ❌ 否 | 限制 `n` 次拆分 |
|              | `rsplit("delimiter")`                 | `RSplit` (迭代器)    | ❌ 否 | 从后向前拆分 |
|              | `lines()`                              | `Lines` (迭代器)     | ❌ 否 | 按行拆分 |
|              | `strip_prefix("text")`                | `Option<&str>`       | ❌ 否 | 去掉前缀（如果匹配） |
|              | `strip_suffix("text")`                | `Option<&str>`       | ❌ 否 | 去掉后缀（如果匹配） |
| **转换**     | `as_str()`                            | `&str`               | ❌ 否 | 获取 `&str` 引用 |
|              | `clone()`                             | `String`             | ❌ 否 | 克隆字符串 |
|              | `to_lowercase()`                      | `String`             | ❌ 否 | 转换为小写 |
|              | `to_uppercase()`                      | `String`             | ❌ 否 | 转换为大写 |
|              | `trim()`                              | `String`             | ❌ 否 | 去除前后空格 |
|              | `trim_start()`                        | `String`             | ❌ 否 | 去除开头空格 |
|              | `trim_end()`                          | `String`             | ❌ 否 | 去除结尾空格 |
| **字符操作** | `chars()`                             | `Chars` (迭代器)     | ❌ 否 | 迭代字符 |
|              | `char_indices()`                      | `CharIndices`        | ❌ 否 | 迭代字符及索引 |
|              | `bytes()`                             | `Bytes` (迭代器)     | ❌ 否 | 迭代字节 |
| **容量管理** | `capacity()`                          | `usize`              | ❌ 否 | 获取分配的容量 |
|              | `reserve(n)`                          | `()`                 | ✅ 是 | 预留至少 `n` 字节 |
|              | `shrink_to_fit()`                     | `()`                 | ✅ 是 | 释放多余的分配空间 |


解释
✅ 是 表示会修改或消耗 String

例如 push()、insert()、clear() 会修改原字符串。
例如 drain()、+（拼接）会消耗 String，需要注意所有权转移。
❌ 否 表示不会消耗 String

例如 len()、contains() 只是 查询 操作，不影响原字符串。
例如 replace()、to_lowercase() 这些方法 返回新的 String，不会修改原字符串。


```rust
fn main() {
    let mut s = String::from("hello");

    // 追加字符
    s.push(' ');
    s.push_str("world");

    // 替换（不会消耗 `s`）
    let new_s = s.replace("world", "Rust");

    // 迭代字符
    for c in s.chars() {
        print!("{} ", c);
    }

    println!("\n原字符串: {}", s);
    println!("新字符串: {}", new_s);
}

```


# Rust `Vec<T>` 方法表

| **类别**      | **方法**                                    | **返回类型**           | **是否消耗 `Vec<T>`** | **说明** |
|--------------|-------------------------------------------|----------------------|----------------------|---------|
| **创建**     | `Vec::new()`                              | `Vec<T>`              | ✅ 是 | 创建一个空 `Vec` |
|              | `vec![1, 2, 3]`                          | `Vec<T>`              | ✅ 是 | 直接创建 `Vec` |
|              | `Vec::with_capacity(n)`                  | `Vec<T>`              | ✅ 是 | 预分配容量 `n` |
| **添加元素** | `push(value)`                            | `()`                  | ✅ 是 | 追加元素 |
|              | `extend(iter)`                           | `()`                  | ✅ 是 | 批量追加多个元素 |
|              | `insert(index, value)`                   | `()`                  | ✅ 是 | 在 `index` 位置插入元素 |
| **删除元素** | `pop()`                                  | `Option<T>`           | ✅ 是 | 移除并返回最后一个元素 |
|              | `remove(index)`                         | `T`                   | ✅ 是 | 移除 `index` 位置的元素 |
|              | `clear()`                                | `()`                  | ✅ 是 | 清空 `Vec` |
|              | `drain(range)`                          | `Drain<T>` (迭代器)   | ✅ 是 | 移除 `range` 内的元素 |
| **访问元素** | `get(index)`                            | `Option<&T>`          | ❌ 否 | 获取元素的引用 |
|              | `get_mut(index)`                        | `Option<&mut T>`      | ❌ 否 | 获取可变引用 |
|              | `first()`                               | `Option<&T>`          | ❌ 否 | 获取第一个元素 |
|              | `first_mut()`                           | `Option<&mut T>`      | ❌ 否 | 获取第一个元素的可变引用 |
|              | `last()`                                | `Option<&T>`          | ❌ 否 | 获取最后一个元素 |
|              | `last_mut()`                            | `Option<&mut T>`      | ❌ 否 | 获取最后一个元素的可变引用 |
|              | `as_slice()`                            | `&[T]`                | ❌ 否 | 转换为切片 |
|              | `as_mut_slice()`                        | `&mut [T]`            | ❌ 否 | 转换为可变切片 |
| **查询**     | `len()`                                 | `usize`               | ❌ 否 | 获取元素个数 |
|              | `is_empty()`                            | `bool`                | ❌ 否 | 是否为空 |
|              | `contains(&value)`                      | `bool`                | ❌ 否 | 是否包含某个值 |
|              | `binary_search(&value)`                 | `Result<usize, usize>` | ❌ 否 | 二分查找（要求排序） |
| **修改**     | `resize(new_len, value)`               | `()`                   | ✅ 是 | 调整 `Vec` 长度 |
|              | `swap(i, j)`                           | `()`                   | ✅ 是 | 交换 `i` 和 `j` 位置的元素 |
|              | `reverse()`                            | `()`                   | ✅ 是 | 反转 `Vec` 顺序 |
|              | `fill(value)`                          | `()`                   | ✅ 是 | 用 `value` 填充整个 `Vec` |
|              | `fill_with(fn)`                        | `()`                   | ✅ 是 | 用 `fn` 生成的值填充 |
|              | `dedup()`                              | `()`                   | ✅ 是 | 移除连续重复的元素 |
|              | `sort()`                               | `()`                   | ✅ 是 | 排序 `Vec` |
|              | `sort_by(|a, b| a.cmp(b))`            | `()`                   | ✅ 是 | 自定义排序 |
|              | `sort_unstable()`                      | `()`                   | ✅ 是 | 不稳定排序（更快） |
| **转换**     | `clone()`                              | `Vec<T>`               | ❌ 否 | 克隆 `Vec` |
|              | `split_at(mid)`                        | `(&[T], &[T])`         | ❌ 否 | 在 `mid` 处分割为两个切片 |
|              | `split_off(at)`                        | `Vec<T>`               | ✅ 是 | 分割 `Vec`，保留前半部分 |
|              | `into_boxed_slice()`                   | `Box<[T]>`             | ✅ 是 | 转换为 `Box<[T]>` |
|              | `into_iter()`                          | `IntoIter<T>` (迭代器) | ✅ 是 | 转换为迭代器（消耗 `Vec`） |
|              | `iter()`                               | `Iter<T>` (迭代器)     | ❌ 否 | 迭代不可变引用 |
|              | `iter_mut()`                           | `IterMut<T>` (迭代器)  | ❌ 否 | 迭代可变引用 |
| **容量管理** | `capacity()`                           | `usize`                | ❌ 否 | 获取分配的容量 |
|              | `reserve(n)`                           | `()`                   | ✅ 是 | 预留至少 `n` 个额外元素的空间 |
|              | `shrink_to_fit()`                      | `()`                   | ✅ 是 | 释放多余的分配空间 |


解释
不会消耗 Vec<T> 的方法（❌ 否）

查询类方法（如 len()、is_empty()、contains()）。
获取引用的方法（如 get()、first()）。
迭代器方法（如 iter()、iter_mut()）。
转换为切片（如 as_slice()）。
会消耗 Vec<T> 的方法（✅ 是）

修改原 Vec<T>（如 push()、pop()、clear()）。
移动所有权（如 into_iter()、split_off()）。
调整容量（如 shrink_to_fit()）。


```rust
fn main() {
    let mut v = vec![1, 2, 3];

    // 添加元素
    v.push(4);
    v.extend(vec![5, 6, 7]);

    // 访问元素
    println!("第一个元素: {:?}", v.first());
    println!("最后一个元素: {:?}", v.last());

    // 迭代
    for i in v.iter() {
        print!("{} ", i);
    }

    // 删除元素
    v.pop();  // 移除最后一个元素
    v.remove(1); // 移除索引 1 处的元素

    // 排序
    v.sort();

    // 转换
    let boxed_slice = v.into_boxed_slice(); // 转换为 `Box<[T]>`
    println!("\n转换后的切片: {:?}", boxed_slice);
}
```




# Rust 数组 `[T; N]` 方法表

| **类别**      | **方法**                          | **返回类型**      | **是否消耗 `[T; N]`** | **说明** |
|--------------|--------------------------------|-----------------|-----------------|---------|
| **创建**     | `[1, 2, 3, 4]`                 | `[T; N]`        | ✅ 是 | 直接定义数组 |
|              | `[0; 5]`                       | `[T; 5]`        | ✅ 是 | 生成 5 个 0 |
| **访问元素** | `arr[i]`                        | `T` 或 `&T`      | ❌ 否 | 通过索引访问元素 |
|              | `get(i)`                        | `Option<&T>`    | ❌ 否 | 安全地获取索引 `i` 处的元素 |
|              | `get_mut(i)`                    | `Option<&mut T>` | ❌ 否 | 可变引用获取元素 |
|              | `first()`                        | `Option<&T>`    | ❌ 否 | 获取第一个元素 |
|              | `first_mut()`                    | `Option<&mut T>` | ❌ 否 | 获取第一个元素的可变引用 |
|              | `last()`                         | `Option<&T>`    | ❌ 否 | 获取最后一个元素 |
|              | `last_mut()`                     | `Option<&mut T>` | ❌ 否 | 获取最后一个元素的可变引用 |
| **查询**     | `len()`                          | `usize`         | ❌ 否 | 数组长度（编译期已知） |
|              | `is_empty()`                     | `bool`          | ❌ 否 | 是否为空（数组永不为空） |
| **切片**     | `&arr[..]`                       | `&[T]`          | ❌ 否 | 获取整个数组切片 |
|              | `&mut arr[..]`                   | `&mut [T]`      | ❌ 否 | 获取可变数组切片 |
|              | `split_at(mid)`                  | `(&[T], &[T])`  | ❌ 否 | 在 `mid` 处分割数组 |
| **转换**     | `to_vec()`                       | `Vec<T>`        | ❌ 否 | 转换为 `Vec<T>` |
|              | `clone()`                        | `[T; N]`        | ❌ 否 | 克隆数组 |
|              | `as_slice()`                     | `&[T]`          | ❌ 否 | 转换为切片 |
|              | `as_mut_slice()`                 | `&mut [T]`      | ❌ 否 | 转换为可变切片 |
| **迭代**     | `iter()`                         | `Iter<T>`       | ❌ 否 | 迭代不可变引用 |
|              | `iter_mut()`                     | `IterMut<T>`    | ❌ 否 | 迭代可变引用 |
|              | `into_iter()`（`[T; N]` >= 1.51） | `IntoIter<T>`   | ✅ 是 | **按值** 迭代（所有权转移） |
| **修改**     | `fill(value)`                    | `()`            | ✅ 是 | 用 `value` 填充整个数组 |
|              | `fill_with(fn)`                  | `()`            | ✅ 是 | 用 `fn` 生成的值填充 |
|              | `swap(i, j)`                     | `()`            | ✅ 是 | 交换 `i` 和 `j` 位置的元素 |
|              | `reverse()`                      | `()`            | ✅ 是 | 反转数组 |
|              | `sort()`（仅 `[T; N] as [T]`）    | `()`            | ✅ 是 | 排序数组（需要切片转换） |
|              | `sort_unstable()`                | `()`            | ✅ 是 | 不稳定排序（更快） |


解释
不会消耗数组的方法（❌ 否）

查询类 方法，如 len()、first()、get() 仅 读取数据，不改变数组。
切片 方法，如 as_slice()、split_at() 仅提供 引用，不会修改原数组。
会消耗数组的方法（✅ 是）

修改类 方法，如 fill()、reverse() 会 直接修改数组。
所有权转移 方法，如 into_iter()（在 Rust 1.51+ 中 按值迭代 会消耗数组）。


```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];

    // 访问元素
    println!("第一个元素: {:?}", arr.first());
    println!("最后一个元素: {:?}", arr.last());
    println!("索引 2 的元素: {:?}", arr.get(2));

    // 转换
    let vec = arr.to_vec();
    println!("转换为 Vec: {:?}", vec);

    // 迭代
    for i in arr.iter() {
        print!("{} ", i);
    }

    // 可变数组
    let mut arr_mut = [10, 20, 30, 40];
    arr_mut.reverse();  // 反转数组
    println!("\n反转后: {:?}", arr_mut);

    arr_mut.fill(99);  // 用 99 填充数组
    println!("填充后: {:?}", arr_mut);
}

```