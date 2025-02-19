//! tests/health_check.rs
// 'tokio::test'是'tokio::main'的测试等价物
// 它还使你不必指定#[test]属性
//
// 你可以使用以下命令检查生成了哪些代码
// cargo expand --test health_check
#[tokio::test]
async fn health_check_works() {
    // 准备
    spawn_app().await.expect("Failed to spawn our app.");
    // 需要引入reqWest对应用程序执行Http请求
    let client = reqwest::Client::new();

    // 执行
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execite request.");

    // 断言
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> std::io::Result<()> {
    todo!()
}
