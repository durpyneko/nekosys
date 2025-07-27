use axum::{Router, routing::get};
use log::{debug, info};
use std::{net::SocketAddr, path::PathBuf};
use tower_http::services::{ServeDir, ServeFile};

pub async fn init() {
    debug!("Starting Servx...");
    let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let _ = nyannel::create("tray").unwrap();
    // wait some time to create channel as without this the tray doesnt get the value
    // TODO could probably find a better fix
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    let app = Router::new()
        // * Routes
        .route_service("/", ServeFile::new(manifest_path.join("index.html")))
        .nest_service("/public", ServeDir::new(manifest_path.join("public/")))
        // * Apis
        .route("/api/hello", get(api_hello));
    //.fallback_service(not_found)
    //.fallback(get(not_found));

    let port = 4989;
    let local_ip = local_ip_address::local_ip().unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let indent = "\t\t\t\t\t";
    // TODO ? maybe on one line instead seperated with commas?
    info!(
        "hosting on:\n{indent} - http://localhost:{}\n{indent} - http://{}:{}",
        port, local_ip, port
    );
    tokio::spawn(async move {
        let msg = serde_json::json!({
            "location": format!("http://{}:{}", local_ip.to_string(), port)
        })
        .to_string();
        if let Err(e) = nyannel::send("tray", msg) {
            debug!("Failed to send tray message: {}", e);
        }
    });

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap() // .with_graceful_shutdown(signal);
}

/* async fn not_found() -> &'static str {
    "404 - Page not found"
} */

async fn api_hello() -> &'static str {
    "servx says hello :)"
}
