#[macro_use]
pub mod scripts;
pub mod text;
pub mod time;

use axum::{
    body::Bytes,
    routing::{get, post},
    Router,
};
use moka::future::Cache;
use std::{net::SocketAddr, time::Duration};

use canvas::prelude::FontDB;

use text::*;
use time::*;

async fn root() -> &'static str {
    r#"Infinite Gifs, by @nathanielfernandes

repo: https://github.com/nathanielfernandes/yabba
gifstream: https://github.com/nathanielfernandes/gifstream"#
}

#[derive(Clone)]
pub struct AppState {
    pub text_slots: Cache<String, Bytes>,
}

#[tokio::main]
async fn main() {
    FontDB::load_from_dir("./assets");

    let state = AppState {
        text_slots: Cache::builder()
            .time_to_idle(Duration::from_secs(60 * 2))
            .build(),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/:tz/time.gif", get(time_gif))
        .route("/t/:id/text.gif", get(get_text_gif))
        .route("/t/:id/text", post(set_text))
        .route("/t/get_id", get(gen_id))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 2223));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub const CORS_HEADERS: [(&str, &str); 2] = [
    ("Access-Control-Allow-Origin", "*"),
    ("Access-Control-Allow-Methods", "GET, POST"),
];

// scripts
// const_script!(SPOTIFY, "spotify.ql");
