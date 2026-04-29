use actix_web::{web, App, HttpServer, HttpResponse, middleware};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use log::info;

mod voice;
mod data;
mod models;

use models::{VoiceCommand, RealTimeData};

#[derive(Clone)]
pub struct AppState {
    voice_active: Arc<Mutex<bool>>,
    current_data: Arc<Mutex<RealTimeData>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_state = AppState {
        voice_active: Arc::new(Mutex::new(false)),
        current_data: Arc::new(Mutex::new(RealTimeData::default())),
    };

    info!("Starting Jarvis Web Assistant Server on 0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(middleware::Logger::default())
            .wrap(actix_cors::Cors::permissive())
            .route("/api/health", web::get().to(health_check))
            .route("/api/voice/activate", web::post().to(activate_voice))
            .route("/api/voice/deactivate", web::post().to(deactivate_voice))
            .route("/api/data", web::get().to(get_real_time_data))
            .route("/api/command", web::post().to(process_command))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "healthy"}))
}

async fn activate_voice(state: web::Data<AppState>) -> HttpResponse {
    let mut active = state.voice_active.lock().unwrap();
    *active = true;
    info!("Voice activation enabled");
    HttpResponse::Ok().json(serde_json::json!({"activated": true}))
}

async fn deactivate_voice(state: web::Data<AppState>) -> HttpResponse {
    let mut active = state.voice_active.lock().unwrap();
    *active = false;
    info!("Voice activation disabled");
    HttpResponse::Ok().json(serde_json::json!({"activated": false}))
}

async fn get_real_time_data(state: web::Data<AppState>) -> HttpResponse {
    let data = state.current_data.lock().unwrap();
    HttpResponse::Ok().json(data.clone())
}

async fn process_command(
    state: web::Data<AppState>,
    cmd: web::Json<VoiceCommand>,
) -> HttpResponse {
    let active = state.voice_active.lock().unwrap();
    
    if !*active {
        return HttpResponse::BadRequest().json(serde_json::json!({"error": "Voice not activated"}));
    }

    info!("Processing command: {}", cmd.text);
    
    let response = voice::process_voice_command(&cmd.text);
    HttpResponse::Ok().json(response)
}
