// #![allow(unused)]

pub use self::error::{Error, Result};

use axum::{
    body::{Body, Bytes}, // 요청 바디 타입
    extract::{Path, Query, Request}, // 요청 추출 타입
    middleware::{self, Next}, // 미들웨어 타입
    response::{Html, IntoResponse, Response}, // 응답 타입
    routing::{get, get_service}, // 라우팅 타입
    Router, // 라우터 타입
};
use http_body_util::BodyExt;
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir; // HTML 응답을 보내기 위한 타입 
use std::net::SocketAddr; // IP주소와 포트를 다루는 타입
use tokio::net::TcpListener; // 비동기 TCP 서버 리스너 
use tracing_subscriber;

mod error;
mod web;
#[tokio::main] // 이 함수를 비동기 런타임에서 실행하도록 하는 매크로
async fn main() {

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::route_login::router())
        .layer(middleware::from_fn(print_request_response))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    //레이어는 아래에서 위로 쌓인다.



    // region: --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    // 열어둔 소켓 주소를 바인딩
    let listener = TcpListener::bind(addr).await.unwrap();
    // 서버 시작
    println!("--> LISTENING on {addr}\n");


    axum::serve(listener, routes_all).await.unwrap();

}

async fn print_request_response(req: Request, next: Next) -> Result<impl IntoResponse> {
    //요청 정보 출력
    println!("--> Request: {req:?}");
    println!("--> Method: {:?}", req.method());
    println!("--> URI: {:?}", req.uri());
    println!("{: <12}", "Headers");
    for (key, value) in req.headers() {
        println!("{}:{:?}", key, value);
    }
    println!();
    println!("--> Request Cookies");
    if let Some(cookie_header) = req.headers().get("Cookie") {
        println!("--> - {cookie_header:?}");
    }


    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));
    
    let res = next.run(req).await;


    //응답 정보 출력
    println!("--> Response: {res:?}");
    println!("--> Status: {:?}", res.status());
    println!("{: <12}", "Headers");
    for (key, value) in res.headers() {
        println!("{}:{:?}", key, value);
    }

    println!();
    println!("--> Response Cookies");
    for (key, value) in res.headers().iter() {
        if key == "set-cookie" {
            println!("--> - {value:?}");
        }
    }

    println!();
    println!("Other Headers");
    for (key, value) in res.headers() {
        if key != "set-cookie" {
            println!("{}:{:?}", key, value);
        }
    }

    println!("--------------------------------");
    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => return Err(Error::BadRequest(format!("Failed to collect body: {err}")))
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{direction} body = {body}");
    }

    Ok(bytes)
}


fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}


    // region: --- Routes Hello
fn routes_hello() -> Router {
    Router::new()
    .route("/hello", get(handler_hello))
    .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}


async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("--> {:<12} -handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong> {name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse { 
    println!("--> {:<12} -handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong> {name}</strong>"))
}

    //endregion: --- Start Server