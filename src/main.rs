use std::fmt::Write;
use tower_http::trace::TraceLayer;
use axum::{Router, routing::get, http::{Method, HeaderMap}, extract::{Path, OriginalUri}};

fn format_response(lines: Vec<String>) -> String {
    let mut memo = String::new();
    memo += "<HTML>\n<BODY>\n";
    // TODO: assemble into different formats depending on the request
    lines.iter().for_each(|l| memo.write_str(&format!("  <P>{}</P>\n", l)).unwrap());
    memo += "</BODY>\n</HTML>\n";

    memo
}

async fn root_handler(
    method: Method,
    headers: HeaderMap,
    original_uri: OriginalUri,
    // `String` consumes the request body and thus must be the last extractor
    // https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors
    body: String) -> String {
    let mut lines: Vec<String> = vec![];
    lines.push(format!("Original URL: {}", original_uri.to_string()));
    lines.push(format!("Path: /"));
    lines.push(format!("Method: {}", method));

    lines.push("Request Headers".into());

    headers.iter().for_each(|(k, v)| lines.push(format!("{}: {}", k, v.to_str().unwrap_or("n/a"))));

    lines.push("Request body".into());
    lines.push(body);
    format_response(lines)
}

async fn path_handler(
    path:  Path<String>,
    method: Method,
    headers: HeaderMap,
    original_uri: OriginalUri,
    // `String` consumes the request body and thus must be the last extractor
    // https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors
    body: String) -> String {
    let mut lines: Vec<String> = vec![];
    lines.push(format!("Original URL: {}", original_uri.to_string()));
    lines.push(format!("Path: {}", path.as_str()));
    lines.push(format!("Method: {}", method));

    lines.push("Request Headers".into());

    headers.iter().for_each(|(k, v)| lines.push(format!("{}: {}", k, v.to_str().unwrap_or("n/a"))));

    lines.push("Request body".into());
    lines.push(body);

    format_response(lines)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/*path", get(path_handler))
        .layer(TraceLayer::new_for_http());

    let listen_port = "3000";
    let listen_addr = format!("0.0.0.0:{}", listen_port);
    println!("Listening on {}", listen_addr);
    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
