mod echo_server;

use echo_server::EchoServer;

/**
 * 写个简单的TcpEchoServer, 就暂时不用处异步io了
 * 线程池队列处理接收就好了
 */
fn main() -> std::io::Result<()> {
    // 启动端口监听
    let mut server = EchoServer::new();

    server.listen("127.0.0.1:9123");

    server.dispatch();

    // 等待线程关闭
    server.join_all();

    Ok(())
}
