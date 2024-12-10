use crate::powerstation::PowerStation;
use axum::{extract::State, http::StatusCode, response::Json, routing::get, routing::post, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

const MAX_TDP: u8 = 30;
const MIN_TDP: u8 = 8;

#[derive(Serialize)]
struct SettingsResponse {
    tdp: u8,
    tdp_limits: (u8, u8),
    power_profile: String,
    power_profiles: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct SettingsRequest {
    tdp: Option<u8>,
    power_profile: String,
}

pub async fn web_server(power_station: Arc<PowerStation>) {
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow any origin
        .allow_headers(Any)
        .allow_methods(Any); // Allow specific methods

    let app = Router::new()
        .route("/settings", get(get_settings))
        .route("/settings", post(set_settings))
        .layer(cors)
        .with_state(power_station);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1338").await.unwrap();
    println!("Starting web server");
    axum::serve(listener, app).await.unwrap();
}

async fn get_settings(State(power_station): State<Arc<PowerStation>>) -> Json<SettingsResponse> {
    Json(SettingsResponse {
        tdp: power_station.get_tdp().await.unwrap(),
        tdp_limits: (MIN_TDP, MAX_TDP),
        power_profile: power_station.get_gpu_profile().await.unwrap(),
        power_profiles: power_station.get_gpu_profiles().await.unwrap(),
    })
}

async fn set_settings(
    State(power_station): State<Arc<PowerStation>>,
    request: Json<SettingsRequest>,
) -> Result<Json<bool>, StatusCode> {
    let available_profiles = power_station.get_gpu_profiles().await.unwrap();

    if !available_profiles.contains(&request.power_profile) {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Set TDP from request or based on power profile selected
    if let Some(tdp_value) = request.tdp {
        if tdp_value > MAX_TDP  || tdp_value < MIN_TDP {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        power_station.set_tdp(tdp_value).await.unwrap();
    } else {
        let tdp_values = get_tdp_for_profiles(&available_profiles, MIN_TDP, MAX_TDP);
        let power_profile_index = available_profiles.iter().position(|r| *r == request.power_profile).unwrap();
        power_station.set_tdp(tdp_values[power_profile_index]).await.unwrap();
    }

    power_station
        .set_gpu_profile(&request.power_profile)
        .await
        .unwrap();

    Ok(Json(true))
}

fn get_tdp_for_profiles(vec: &Vec<String>, min_value: u8, max_value: u8) -> Vec<u8> {
    let len = vec.len();
    if len == 0 {
        return vec![];
    }

    vec.iter()
        .enumerate()
        .map(|(index, _)| {
            let proportion = index as f64 / (len - 1).max(1) as f64;
            // Map the proportion to the desired range [min_value, max_value]
            (proportion * (max_value - min_value) as f64 + min_value as f64).round() as u8
        })
        .collect()
}