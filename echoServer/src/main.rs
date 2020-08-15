mod tcp_server;

use tcp_server::TcpServer;


/**
 * 写个简单的TcpEchoServer, 就暂时不用处异步io了
 * 线程池队列处理接收就好了
 */
fn main() -> std::io::Result<()> {
    // 启动端口监听
    let mut tcp_server = TcpServer::new();

    tcp_server.listen("127.0.0.1:9123");

    tcp_server.dispatch();

    // 等待线程关闭
    tcp_server.join_all();

    Ok(())
}
