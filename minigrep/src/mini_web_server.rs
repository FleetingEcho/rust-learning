use async_std::fs;
use async_std::io::{Read, Result as AsyncResult, Write};
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;
use futures::stream::StreamExt;
use futures::task::{Context, Poll};
use std::cmp::min;
use std::marker::Unpin;
use std::pin::Pin;
use std::time::Duration;

pub async fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();
            task::spawn(handle_connection(stream));
        })
        .await;
}

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\r\n", "404.html")
    };
    let contents = fs::read_to_string(format!("src/{}", filename))
        .await
        .unwrap();

    let response = format!("{status_line}{contents}");
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

//================TEST =================

struct MockTcpStream {
    //rust要求stream必须提供的，Read 需要 read_data 和 cursor 来模拟流式读取：Write 需要 write_data 来存储写入的 HTTP 响应：
    read_data: Vec<u8>,
    write_data: Vec<u8>,
    cursor: usize, //如果 cursor 不存在，也就是是用let size = min(self.read_data.len(), buf.len()); buf[..size].copy_from_slice(&self.read_data[..size]); 那么poll_read 每次都会返回相同的数据，导致 handle_connection 无法正确解析 HTTP 请求。
}
/*
你要模拟 TcpStream 的行为，而 TcpStream 是双向的（可读 + 可写）：
TcpStream 读取 HTTP 请求（read_data）
TcpStream 发送 HTTP 响应（write_data）
*/

// 实现 `AsyncRead`
impl async_std::io::Read for MockTcpStream {
    fn poll_read(
        mut self: Pin<&mut Self>, // 被 `Pin` 固定的可变引用，防止 `Self` 移动
        _: &mut Context,          // 异步 `Context`，用于 `Waker`
        buf: &mut [u8],           // 目标缓冲区，数据会被拷贝到这里
    ) -> Poll<AsyncResult<usize>> // 返回 `Poll::Ready(Ok(n))` 或 `Poll::Pending`
    {
        let remaining = self.read_data.len().saturating_sub(self.cursor); //计算 还剩下多少数据可以读取
        let size = min(remaining, buf.len()); //size 计算本次实际可读取的数据大小：

        if size == 0 {
            return Poll::Ready(Ok(0)); // 没有更多数据可读
                                       //如果 size == 0，表示 没有可读的数据，直接返回 Poll::Ready(Ok(0))，表示 EOF（文件结束）。
        }

        buf[..size].copy_from_slice(&self.read_data[self.cursor..self.cursor + size]); //将 read_data 的 [cursor..cursor+size] 片段拷贝到 buf：
        self.cursor += size;
        Poll::Ready(Ok(size))
    }
}

impl async_std::io::Write for MockTcpStream {
    fn poll_write(
        mut self: Pin<&mut Self>, //确保 Self 不能被安全地移动，避免 poll_read 和 poll_write 过程中 Self 内部的指针或 Future 失效。异步 Read 和 Write 需要 Pin<&mut Self>，因为 async 任务可能在 poll 过程中暂停，而 Self 不能在暂停期间被移动，否则会破坏内部状态。
        _: &mut Context, //提供 Waker，用于在 poll 不能立刻完成时，注册唤醒机制，让 Future 在可读/可写时继续执行。异步任务可能会暂停 (Pending)，Context 允许异步运行时（如 Tokio、async-std）在未来合适的时机重新 poll 这个任务。
        buf: &[u8],
    ) -> Poll<AsyncResult<usize>> {
        /*
                Poll<AsyncResult<usize>> 表示异步 poll_read 或 poll_write 的返回值，可能是 Poll::Ready(Ok(n))（成功读取/写入 n 字节）或 Poll::Pending（当前不可读/写，需等待唤醒）。
        AsyncResult<usize> 是 Result<usize, Error> 的别名，表示操作成功时返回读取/写入的字节数，失败时返回错误。
                */

        self.write_data.extend_from_slice(buf);
        //self.write_data.extend_from_slice(buf); 将 buf 追加到 write_data，确保 MockTcpStream 累积所有写入的数据，而不是覆盖原数据。
        // 相比 self.write_data = Vec::from(buf);，这种方式避免丢失之前写入的内容，模拟真实的 TcpStream 写入行为。
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<AsyncResult<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<AsyncResult<()>> {
        Poll::Ready(Ok(()))
    }
}

//这样，Rust 就知道 MockTcpStream 可以安全移动，可以传递到 async 代码中。
impl Unpin for MockTcpStream {}

#[async_std::test]
async fn test_handle_connection() {
    let input_bytes = b"GET / HTTP/1.1\r\n";
    let mut contents = vec![0u8; 1024];
    contents[..input_bytes.len()].clone_from_slice(input_bytes);
    let mut stream = MockTcpStream {
        read_data: contents,
        write_data: Vec::new(),
        cursor: 0, // 添加 cursor
    };

    handle_connection(&mut stream).await;

    let expected_contents = async_std::fs::read_to_string("src/hello.html")
        .await
        .unwrap();
    let expected_response = format!("HTTP/1.1 200 OK\r\n{}", expected_contents);
    println!("Expected Response:\n{}", expected_response);
    println!(
        "Actual Write Data:\n{}",
        String::from_utf8_lossy(&stream.write_data)
    );
}

/*
在 async 代码中：

stream 可能会被 Future 持有，并在 poll 过程中 被 .await 挂起。
Future 的生命周期可能比 stream 长，如果 stream 不是 Unpin，它可能在 poll 期间被错误地移动，导致数据损坏。
async 运行时（如 async-std、Tokio）期望 stream 是 Unpin，否则需要 Pin<Box<T>>。


*/

/*
let s1: &str = "banana"; //切片引用的大小在编译器都是已知的。 不能使用str，因为大小未知
let s2: &str = &String::from("banana");

let arr = [1, 2, 3, 4, 5];

let s3: &[i32] = &arr[1..3];

切片	切片引用
str 字符串切片	&str 字符串切片的引用
[u8] 数组切片	&[u8] 数组切片的引用

*/
