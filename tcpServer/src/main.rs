mod args;
mod tcp_server;

use args::Opt;
use tcp_server::TcpServer;


/**
 * 写个简单的TcpEchoServer, 就暂时不用处异步io了
 * 线程池队列处理接收就好了
 */
fn main() -> std::io::Result<()> {

    let opt = Opt::parse_args();
    println!("{}", opt);

    // 启动端口监听
    let mut server = TcpServer::new();

    server.listen(opt.hostname().as_str());

    server.dispatch();

    // 等待线程关闭
    server.join_all();

    Ok(())
}
