use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCommand {
    pub text: String,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RealTimeData {
    pub timestamp: String,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub temperature: f64,
    pub network_status: String,
    pub active_processes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResponse {
    pub command: String,
    pub response: String,
    pub data: Option<RealTimeData>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceResponse {
    pub text: String,
    pub audio_url: Option<String>,
}
