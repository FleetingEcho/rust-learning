pub fn vec_test() {
    // 1️⃣ 创建一个 Vec 并填充数据
    let mut numbers: Vec<i32> = vec![10, 20, 30, 40, 50];
    println!("Initial Vec: {:?}", numbers);

    // 2️⃣ 追加元素 60
    numbers.push(60);
    println!("After push(60): {:?}", numbers);

    // 3️⃣ 在索引 2 插入 25
    numbers.insert(2, 25);
    println!("After insert(2, 25): {:?}", numbers);

    // 4️⃣ 删除索引 4 的元素（40）
    numbers.remove(4);
    println!("After remove(4): {:?}", numbers);

    // 5️⃣ 修改索引 3 的元素为 100
    numbers[3] = 100;
    println!("After modifying index 3 to 100: {:?}", numbers);

    // 6️⃣ 弹出最后一个元素
    if let Some(last) = numbers.pop() {
        println!("Popped element: {}", last);
    }
    println!("After pop(): {:?}", numbers);

    // 7️⃣ 遍历 Vec 并打印所有元素
    println!("Iterating through Vec:");
    for num in &numbers {
        print!("{} ", num);
    }
    println!();

    // 8️⃣ 查找 50 的索引
    match numbers.iter().position(|&x| x == 50) {
        Some(index) => println!("50 found at index: {}", index),
        None => println!("50 Not Found"),
    }

    // 9️⃣ 计算 Vec 中所有元素的和
    let sum: i32 = numbers.iter().sum();
    println!("Sum of Vec: {}", sum);

    // 🔟 排序 Vec
    numbers.sort();
    println!("Sorted Vec: {:?}", numbers);
}
