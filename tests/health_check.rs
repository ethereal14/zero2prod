//! tests/health_check.rs
// 'tokio::test'是'tokio::main'的测试等价物
// 它还使你不必指定#[test]属性
//
// 你可以使用以下命令检查生成了哪些代码
// cargo expand --test health_check
use std::net::TcpListener;
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

/// 启动应用程序的一个实例
/// 并返回其地址(例如：http://localhost:XXXX)
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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // 准备
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // 执行
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // 断言
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // 准备
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the name"),
        ("email=ursula_le_guin%40gmail.com", "missing the email"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // 断言
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Requset when the payload was {}.",
            error_message
        );
    }
}
