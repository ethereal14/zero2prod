use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;

    // 如果绑定地址失败，则会发生错误io::Error
    // 否则，在服务器上调用 .await
    run(listener)?.await
}
