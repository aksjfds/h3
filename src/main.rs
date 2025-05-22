use salvo::conn::rustls::RustlsConfig;
use salvo::prelude::*;

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().get(hello);

    let config = RustlsConfig::new(None);

    let listener = TcpListener::new(("0.0.0.0", 5800));

    let acceptor = QuinnListener::new(config.build_quinn_config().unwrap(), ("0.0.0.0", 5800))
        .join(listener)
        .bind()
        .await;

    Server::new(acceptor).serve(router).await;
}
