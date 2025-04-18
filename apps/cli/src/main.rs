use std::sync::Arc;

use anki_backend::{self, AnkiBackend};
use axum::{
    body::{to_bytes, Body},
    extract::{Path, State},
    http::Response,
    response::IntoResponse,
    routing::post,
    Router,
};
use clap::{Parser, Subcommand};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    langs: Option<Vec<String>>,

    #[arg(short, long, default_value_t = 3001)]
    port: usize,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Dev {},
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    match cli.command.unwrap_or(Commands::Dev {}) {
        Commands::Dev {} => {
            let backend = Arc::new(
                anki_backend::AnkiBackend::new(cli.langs.unwrap_or(vec!["en".to_owned()])).unwrap(),
            );
            let app = Router::new()
                .route("/rpc/{service}/{method}", post(rpc))
                .with_state(AppState { backend })
                .layer(TraceLayer::new_for_http());

            let addr = format!("127.0.0.1:{}", cli.port);
            tracing::info!("rpc serve at {}", addr);
            let listener = TcpListener::bind(addr).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }
    }
}

#[derive(Clone)]
struct AppState {
    backend: Arc<AnkiBackend>,
}

async fn rpc(
    State(state): State<AppState>,
    Path((service, method)): Path<(u32, u32)>,
    body: Body,
) -> impl IntoResponse {
    let input = to_bytes(body, usize::MAX).await.unwrap();
    let (result, error) = match state.backend.run_method(service, method, &input) {
        Ok(res) => (res, false),
        Err(err) => (err, true),
    };

    Response::builder()
        .status(200)
        .header("x-rpc-error", if error { 1 } else { 0 })
        .body(Body::from(result))
        .unwrap()
}
