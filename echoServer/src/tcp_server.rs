use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Error, BufReader, BufRead, Write};
use std::thread;

pub struct TcpServer {
    listener: Option<TcpListener>,
    threads: Vec<thread::JoinHandle<()>>,
}

impl TcpServer {
    pub fn new() -> Self {
        return TcpServer{
            listener: None,
            threads: vec![]
        };
    }

    pub fn listen(&mut self, hostname: &str) {
        self.listener = TcpListener::bind(hostname).ok();
        println!("start listener: {}", self.listener.as_ref().unwrap().local_addr().unwrap().to_string());
    }

    pub fn dispatch(&mut self) {
        // 处理循环, 接收连接
        for stream in self.listener.as_ref().unwrap().incoming() {
            let stream = stream.expect("failed!");

            // 新接入连, 分配线程处理
            let handle = thread::spawn(move || {
                EchoHandler::handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
            });

            // 线程池句柄存入Vec
            self.threads.push(handle);
        }
    }

    pub fn join_all(self) {
        // 等待线程关闭
        for handle in self.threads {
            handle.join().unwrap();
        }
    }
}

trait StreamHandler {
    fn handle_client(stream: TcpStream) -> Result<(), Error>;
}

pub struct EchoHandler{

}

impl EchoHandler {
    pub fn handle_client(stream: TcpStream) -> Result<(), Error>{
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
}