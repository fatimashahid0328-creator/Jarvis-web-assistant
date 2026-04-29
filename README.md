# 🎤 Jarvis Web Assistant

**Offline voice assistant that respects your privacy. MVVM web app with real-time data.**

## Features

✨ **Voice Activation** - Say "Jarvis" to activate  
🎯 **MVVM Architecture** - Svelte frontend with reactive stores  
⚡ **Real-Time Data** - Live system metrics (CPU, Memory, Temperature)  
🔒 **Privacy-First** - Offline processing with local voice recognition  
🚀 **High Performance** - Rust backend + Svelte frontend  
📊 **Command Processor** - Execute custom commands with voice  

## Tech Stack

**Backend:**
- Rust + Actix-web
- Real-time data processing
- Voice command API

**Frontend:**
- Svelte 4 + Vite
- MVVM pattern with stores
- Web Speech API
- Responsive design

## Project Structure

```
Jarvis-web-assistant/
├── src/
│   ├── main.rs           # Actix server setup
│   ├── models.rs         # Data structures
│   ├── voice.rs          # Voice command processing
│   └── data.rs           # System data collection
├── frontend/
│   ├── src/
│   │   ├── App.svelte    # Main app component
│   │   ├── stores.js     # MVVM stores
│   │   ├── api.js        # API client
│   │   └── components/   # Svelte components
│   └── index.html
├── Cargo.toml
└── README.md
```

## Installation

### Backend Setup

```bash
cd Jarvis-web-assistant
cargo build --release
cargo run
```

Server starts on `http://localhost:8080`

### Frontend Setup

```bash
cd frontend
npm install
npm run dev
```

Frontend runs on `http://localhost:5173`

## Usage

1. **Start the backend:**
   ```bash
   cargo run
   ```

2. **Start the frontend:**
   ```bash
   cd frontend && npm run dev
   ```

3. **Open in browser:** `http://localhost:5173`

4. **Activate voice:** Click "Activate Jarvis" button or say "Jarvis"

5. **Send commands:**
   - "time" - Get current time
   - "status" - System status
   - "system" - Full system info

## API Endpoints

- `POST /api/voice/activate` - Activate voice assistant
- `POST /api/voice/deactivate` - Deactivate voice assistant
- `POST /api/command` - Process voice command
- `GET /api/data` - Get real-time system data
- `GET /api/health` - Health check

## Command Format

```json
{
  "text": "status",
  "timestamp": "2026-04-29T10:30:00Z"
}
```

## Response Format

```json
{
  "command": "status",
  "response": "System status: CPU 45.2%, Memory 62.8%, Temp 52.5°C",
  "data": {
    "cpu_usage": 45.2,
    "memory_usage": 62.8,
    "temperature": 52.5,
    "network_status": "Connected",
    "active_processes": 127
  },
  "success": true
}
```

## MVVM Architecture

The Svelte frontend implements MVVM pattern:

- **View**: Svelte components (VoiceActivator, DataDisplay, CommandProcessor)
- **ViewModel**: Svelte stores (voiceStore, dataStore, commandStore)
- **Model**: API client communicating with Rust backend

## Development

### Add new voice command:

1. Edit `src/voice.rs`
2. Add pattern matching in `process_voice_command()`
3. Implement handler function

### Add new component:

1. Create file in `frontend/src/components/`
2. Import in `App.svelte`
3. Add to layout grid

## License

MIT License - Open source and free to use

## Contributing

Contributions welcome! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation

---

**Made with ❤️ by fatimashahid0328**
