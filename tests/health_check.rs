// tests/health_check.rs
// tokio::test 是tokio::main的测试等价物
// 它还使你不必指定#[test]属性
// 你可以使用如下命令检查生成了哪些代码：cargo expand --test health_check

use reqwest::header::EXPECT;
use std::net::TcpListener;
use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    // 准备
    let address = spawn_app();
    // 需要引入reqWest对应用程序执行HTTP请求
    let client = reqwest::Client::new();

    // 执行
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // 断言
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// 此处没有.await调用，因此现在spawn_app函数不需要是异步的
// 我们也在此运行测试，所以传播错误是不值得的
// 如果未能执行所需的初始化，则可能发生panic并让所有工作崩溃
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // 检索操作系统分配的port
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address.");
    // 启动服务器作为后台任务
    // tokio::spawn 返回一个指向spawned future的handle
    // 但是这里没有用它。因为这是一个非绑定的let用法
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
