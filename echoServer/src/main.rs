use std::io::{Error, BufReader, BufRead, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::{thread};

fn handle_client(stream: TcpStream) -> Result<(), Error>{
    // 打印远程连接地址
    let addr = stream.peer_addr().unwrap();
    println!("[{} -> online]", addr);

    // stream 转为reader, writer处理, 写入为mut类型
    let (reader, mut writer) = (&stream, &stream);

    // 分配一个reader的buffer用于接收数据
    let reader = BufReader::new(reader);

    // reader 转化为lines逐行读取数据
    let mut lines = reader.lines();
    while let Some(line) = lines.next() {
        // 模式匹配读取正确及其错误数据的逻辑
        match line {
            // 读取成功, 处理数据逻辑
            Ok(mut line) => {
                // 接收到 ":quit" 指令, 断开连接退出线程
                if line.eq_ignore_ascii_case(":quit") {
                    println!("[{}] remote request close", addr);
                    stream.shutdown(Shutdown::Both).unwrap_or_else(|err| eprintln!("{:?}", err));
                    break;
                }

                // 处理echo回声
                println!("[{}]: {}", addr, line.as_str());
                line.push_str("\n");
                writer.write_all(line.as_bytes()).unwrap_or_else(|error| eprintln!("{:?}", error));
            }

            // 读取数据错误处理逻辑
            Err(error) => {
                eprintln!("[{}] -> {:?}", addr, error)
            }
        }
    }

    Ok(())
}

/**
 * 写个简单的TcpEchoServer, 就暂时不用处异步io了
 * 线程池队列处理接收就好了
 */
fn main() -> std::io::Result<()> {
    // 启动端口监听
    let listener = TcpListener::bind("127.0.0.1:9123")?;
    println!("start listener: {}", listener.local_addr()?.to_string());

    // 分配线程Vec存储线程实例
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    // 处理循环, 接收连接
    for stream in listener.incoming() {
        let stream = stream.expect("failed!");

        // 新接入连, 分配线程处理
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });

        // 线程池句柄存入Vec
        thread_vec.push(handle);
    }

    // 等待线程关闭
    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}
