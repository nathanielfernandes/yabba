use axum::{
    body::StreamBody,
    extract::Path,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use std::{sync::Arc, time::Duration};

use canvas::prelude::*;

use gifstream::{GifStream, GIF_HEADERS};
use image::RgbaImage;
use once_cell::sync::Lazy;

use crate::{scripts::*, CORS_HEADERS};

const_script!(TIME, "time.ql");

async fn generate_time_frame(tz: Arc<chrono_tz::Tz>) -> Result<Vec<u8>, &'static str> {
    let time = chrono::Utc::now()
        .with_timezone(&*tz)
        .format("%I:%M:%S %p")
        .to_string();

    let mut img = RgbaImage::new(190, 40);
    run(&mut img, &TIME, MINIMAL_RUNTIME, &[("time", time)], &[])?;
    Ok(img.into_raw())
}

pub async fn time_gif(Path(tz): Path<String>) -> Response {
    let tz = match tz.to_uppercase().as_str() {
        "EST" => chrono_tz::America::New_York,
        "CST" => chrono_tz::America::Chicago,
        "MST" => chrono_tz::America::Denver,
        "PST" => chrono_tz::America::Los_Angeles,
        "AKST" => chrono_tz::America::Anchorage,
        "HST" => chrono_tz::Pacific::Honolulu,
        "GMT" => chrono_tz::Etc::GMT,
        "UTC" => chrono_tz::UTC,
        _ => return (StatusCode::BAD_REQUEST, CORS_HEADERS, "Invalid timezone").into_response(),
    };

    let gs = GifStream::new(
        Duration::from_secs(1),
        190,
        40,
        Arc::new(tz),
        generate_time_frame,
    )
    .speed(30);

    let stream = gs.stream_auto_palette(32);
    let body = StreamBody::new(stream);

    (GIF_HEADERS, body).into_response()
}
