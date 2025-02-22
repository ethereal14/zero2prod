//! tests/health_check.rs
// 'tokio::test'是'tokio::main'的测试等价物
// 它还使你不必指定#[test]属性
//
// 你可以使用以下命令检查生成了哪些代码
// cargo expand --test health_check
#[tokio::test]
async fn health_check_works() {
    // 准备
    let address = spawn_app();
    // 需要引入reqWest对应用程序执行Http请求
    let client = reqwest::Client::new();

    // 执行
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execite request.");

    // 断言
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // 检索操作系统分配的端口
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    // 启动服务器作为后台任务
    // tokio::spawn 返回一个指向spawned future的handle
    // 但是这里没有用它，因为这是非绑定的let用法
    let _ = tokio::spawn(server);
    // 将应用程序地址返回给调用者
    format!("http://127.0.0.1:{}", port)
}
