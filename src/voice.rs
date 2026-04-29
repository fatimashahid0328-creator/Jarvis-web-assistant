use crate::models::{CommandResponse, RealTimeData};
use chrono::Local;

pub fn process_voice_command(command: &str) -> CommandResponse {
    let command_lower = command.to_lowercase();
    
    let response = match command_lower.as_str() {
        _ if command_lower.contains("time") => {
            handle_time_command()
        },
        _ if command_lower.contains("status") => {
            handle_status_command()
        },
        _ if command_lower.contains("system") => {
            handle_system_command()
        },
        _ => {
            CommandResponse {
                command: command.to_string(),
                response: "Command not recognized. Try 'time', 'status', or 'system'".to_string(),
                data: None,
                success: false,
            }
        }
    };
    
    response
}

fn handle_time_command() -> CommandResponse {
    let now = Local::now();
    CommandResponse {
        command: "time".to_string(),
        response: format!("The current time is {}", now.format("%H:%M:%S")),
        data: None,
        success: true,
    }
}

fn handle_status_command() -> CommandResponse {
    let data = RealTimeData {
        timestamp: chrono::Utc::now().to_rfc3339(),
        cpu_usage: 45.2,
        memory_usage: 62.8,
        temperature: 52.5,
        network_status: "Connected".to_string(),
        active_processes: 127,
    };
    
    CommandResponse {
        command: "status".to_string(),
        response: format!("System status: CPU {}%, Memory {}%, Temp {}°C", 
            data.cpu_usage, data.memory_usage, data.temperature),
        data: Some(data),
        success: true,
    }
}

fn handle_system_command() -> CommandResponse {
    let data = RealTimeData {
        timestamp: chrono::Utc::now().to_rfc3339(),
        cpu_usage: 38.5,
        memory_usage: 71.2,
        temperature: 58.3,
        network_status: "Connected".to_string(),
        active_processes: 156,
    };
    
    CommandResponse {
        command: "system".to_string(),
        response: "System information retrieved".to_string(),
        data: Some(data),
        success: true,
    }
}
