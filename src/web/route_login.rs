use axum::{routing::post, Json, Router};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use crate::{web, Error, Result};
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new().route("/api/login",post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");
    
    // db 저장 로직은 추후 수정 예정임
    if payload.username != "demo1" || payload.pwd != "1234" {
        return Err(Error::LoginFailed);
    }

    // 쿠키 생성
    // 나중에 진짜 쿠키 생성 로직 추가 해야함 
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));


    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}


#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String
}