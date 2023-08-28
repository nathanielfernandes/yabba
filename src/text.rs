use axum::{
    body::{Bytes, StreamBody},
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
};
use moka::future::Cache;
use rand::Rng;
use reqwest::StatusCode;
use std::{sync::Arc, time::Duration};

use canvas::prelude::*;

use gifstream::{GifStream, GIF_HEADERS};
use image::RgbaImage;
use once_cell::sync::Lazy;

use crate::{scripts::*, AppState};

const_script!(TEXT, "text.ql");

fn generate_text_frame(text: String) -> Result<Bytes, &'static str> {
    let mut img = RgbaImage::new(500, 50);
    run(&mut img, &TEXT, MINIMAL_RUNTIME, &[("text", text)], &[])?;
    Ok(img.into_raw().into())
}

async fn get_text_frame(
    (id, text_slots): (Arc<String>, Cache<String, Bytes>),
) -> Result<Bytes, &'static str> {
    let Some(img) = text_slots.get(&*id) else {
        return generate_text_frame("not found".into());
    };

    Ok(img)
}

pub async fn get_text_gif(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    let gs = GifStream::new(
        Duration::from_millis(512),
        500,
        50,
        (Arc::new(id), state.text_slots.clone()),
        get_text_frame,
    )
    .speed(30);

    let stream = gs.stream_auto_palette(16);
    let body = StreamBody::new(stream);

    (GIF_HEADERS, body).into_response()
}

#[derive(serde::Deserialize)]
pub struct Q {
    t: String,
}

pub async fn set_text(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(q): Query<Q>,
) -> Response {
    let text = q.t;

    let Ok(img) = generate_text_frame(text) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to generate text frame",
        )
            .into_response();
    };

    if state.text_slots.contains_key(&*id) {
        state.text_slots.insert(id, img).await;
    } else {
        return (StatusCode::NOT_FOUND, "text slot not found").into_response();
    }

    "ok".into_response()
}

pub async fn gen_id(State(state): State<AppState>) -> Response {
    let mut id = String::new();
    {
        let mut rng = rand::thread_rng();
        for _ in 0..8 {
            id.push(rng.gen_range(b'a'..=b'z') as char);
        }
    }

    let Ok(img) = generate_text_frame("this is some sample text, edit me!".into()) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to generate text frame",
        )
            .into_response();
    };

    state.text_slots.insert(id.clone(), img).await;

    id.into_response()
}
