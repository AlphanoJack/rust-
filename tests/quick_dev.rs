
use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=Mark").await?.print().await?;
    hc.do_get("/hello2/Michael").await?.print().await?;
    // hc.do_get("/src/main.rs").await?.print().await?;
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "1234"
        })
    );
    req_login.await?.print().await?;

    Ok(())
}