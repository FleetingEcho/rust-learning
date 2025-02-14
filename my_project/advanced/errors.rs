// Rust 错误处理详解
// Rust 提供了一种安全、健壮的错误处理机制。本文将深入探讨：

// 基本错误处理概念
// 组合器 (Combinators)
// 自定义错误类型
// 错误转换 (From Trait)
// 错误归一化
// 错误处理库 (thiserror、anyhow)


// 1. Rust 基本错误处理概念
// Rust 使用 Result<T, E> 和 Option<T> 进行错误和可选值的处理：

// Option<T>：用于表示 可能为空 的情况，常见于 Some(value) 或 None。
// Result<T, E>：用于表示 可能发生错误 的情况，常见于 Ok(value) 或 Err(error)。

fn may_return_none(flag: bool) -> Option<i32> {
    if flag {
        Some(42)
    } else {
        None
    }
}

fn may_fail(flag: bool) -> Result<i32, String> {
    if flag {
        Ok(42)
    } else {
        Err("Something went wrong!".to_string())
    }
}


// 2. 组合器（Combinators）
// 组合器是用于简化 Option 和 Result 处理的高阶函数。

// or() 和 and()
// or()：如果 self 是 Some 或 Ok，直接返回 self，否则返回 other。
// and()：如果 self 和 other 都是 Some 或 Ok，则返回 other。

fn main() {
    let s1 = Some("Rust");
    let s2 = Some("Language");
    let n: Option<&str> = None;

    assert_eq!(s1.or(s2), s1);  // Some("Rust") or Some("Language") -> Some("Rust")
    assert_eq!(n.or(s1), s1);    // None or Some("Rust") -> Some("Rust")

    assert_eq!(s1.and(s2), s2);  // Some("Rust") and Some("Language") -> Some("Language")
    assert_eq!(n.and(s1), n);    // None and Some("Rust") -> None
}
// or_else() 和 and_then()
// or_else()：接受一个闭包，若 self 是 None 或 Err，则调用闭包返回 Some 或 Ok。
// and_then()：接受一个闭包，若 self 是 Some 或 Ok，则调用闭包并返回其结果。

fn main() {
    let some_val = Some(5);
    let none_val: Option<i32> = None;

    // or_else 示例
    let result = none_val.or_else(|| Some(10));
    assert_eq!(result, Some(10));

    // and_then 示例
    let squared = some_val.and_then(|x| Some(x * x));
    assert_eq!(squared, Some(25));
}
// map() 和 map_err()
// map()：对 Some 或 Ok 内部的值进行映射转换。
// map_err()：对 Err 内部的错误值进行映射转换。

fn main() {
    let number: Option<&str> = Some("123");

    let parsed_number = number.map(|s| s.parse::<i32>().unwrap_or(0));
    assert_eq!(parsed_number, Some(123));

    let error_result: Result<i32, &str> = Err("404");

    let mapped_error = error_result.map_err(|e| format!("Error code: {}", e));
    assert_eq!(mapped_error, Err("Error code: 404".to_string()));
}



// 3. 自定义错误类型
// 在 Rust 中，可以使用 enum 定义错误类型，并实现 Display 和 Debug 以提供更友好的错误信息。


use std::fmt;

#[derive(Debug)]
struct AppError {
    code: usize,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Error {}] {}", self.code, self.message)
    }
}

fn produce_error() -> Result<(), AppError> {
    Err(AppError {
        code: 404,
        message: "Resource Not Found".to_string(),
    })
}

fn main() {
    match produce_error() {
        Err(e) => eprintln!("{}", e),
        _ => println!("No error"),
    }
}



// 4. 错误转换 (From Trait)
// From<T> 允许自动转换错误类型，结合 ? 操作符进行隐式转换。


use std::fs::File;
use std::io;

#[derive(Debug)]
struct AppError {
    message: String,
}

// 允许从 io::Error 转换成 AppError
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            message: error.to_string(),
        }
    }
}

fn open_file() -> Result<File, AppError> {
    let file = File::open("non_existent.txt")?;
    Ok(file)
}

fn main() {
    match open_file() {
        Err(e) => eprintln!("Error: {}", e.message),
        _ => println!("File opened successfully"),
    }
}


// 5. 错误归一化
// 如果函数涉及多个错误来源，可以用 特征对象 (Box<dyn Error>) 或 自定义枚举 统一错误类型。

// 使用 Box<dyn Error>

// use std::fs::read_to_string;
// use std::error::Error;

// fn read_config() -> Result<String, Box<dyn Error>> {
//     let filename = std::env::var("CONFIG_FILE")?;
//     let content = read_to_string(filename)?;
//     Ok(content)
// }
// 使用自定义错误枚举

use std::fs::read_to_string;
use std::env;

#[derive(Debug)]
enum MyError {
    EnvError(env::VarError),
    IOError(std::io::Error),
}

impl From<env::VarError> for MyError {
    fn from(e: env::VarError) -> Self {
        MyError::EnvError(e)
    }
}

impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        MyError::IOError(e)
    }
}

fn read_config() -> Result<String, MyError> {
    let filename = env::var("CONFIG_FILE")?;
    let content = read_to_string(filename)?;
    Ok(content)
}



// 6. 错误处理库
// 使用 thiserror
// thiserror 提供了一种简洁的方式定义错误：


use thiserror::Error;
use std::fs::read_to_string;

#[derive(Error, Debug)]
enum MyError {
    #[error("Environment variable not found")]
    EnvError(#[from] std::env::VarError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
// 使用 anyhow
// anyhow 适用于快速构建错误处理：


use anyhow::Result;
use std::fs::read_to_string;

fn read_config() -> Result<String> {
    let filename = std::env::var("CONFIG_FILE")?;
    let content = read_to_string(filename)?;
    Ok(content)
}


// 总结

// Option<T> 适用于 可选值，Result<T, E> 适用于 错误处理。
// 组合器 (map、or_else、and_then) 提供高阶操作。
// 自定义错误类型 让错误信息更清晰。
// 错误归一化 统一不同类型的错误处理。
// 使用 thiserror 和 anyhow 简化错误处理。

// Rust 的错误处理机制虽然稍显复杂，但其安全性和灵活性让代码更加健壮！ 🚀