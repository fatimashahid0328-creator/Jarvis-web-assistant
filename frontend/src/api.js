import axios from 'axios';

const API_URL = 'http://localhost:8080/api';

const api = axios.create({
  baseURL: API_URL,
  timeout: 5000,
});

export async function activateVoice() {
  const response = await api.post('/voice/activate');
  return response.data;
}

export async function deactivateVoice() {
  const response = await api.post('/voice/deactivate');
  return response.data;
}

export async function getRealTimeData() {
  const response = await api.get('/data');
  return response.data;
}

export async function processCommand(commandText) {
  const response = await api.post('/command', {
    text: commandText,
    timestamp: new Date().toISOString()
  });
  return response.data;
}

export async function healthCheck() {
  const response = await api.get('/health');
  return response.data;
}
