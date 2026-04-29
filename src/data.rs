use crate::models::RealTimeData;
use chrono::Utc;

pub fn get_system_data() -> RealTimeData {
    RealTimeData {
        timestamp: Utc::now().to_rfc3339(),
        cpu_usage: rand::random::<f64>() * 100.0,
        memory_usage: 50.0 + rand::random::<f64>() * 30.0,
        temperature: 40.0 + rand::random::<f64>() * 30.0,
        network_status: "Connected".to_string(),
        active_processes: (50 + (rand::random::<u32>() % 200)) as u32,
    }
}
