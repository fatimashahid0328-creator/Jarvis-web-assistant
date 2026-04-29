<script>
  export let data;

  function formatValue(value, unit = '') {
    if (typeof value === 'number') {
      return value.toFixed(1) + unit;
    }
    return value + unit;
  }
</script>

<div class="card">
  <h2>📊 Real-Time System Data</h2>
  
  <div class="data-grid">
    <div class="data-item">
      <span class="label">CPU Usage</span>
      <span class="value">{formatValue(data.cpu_usage, '%')}</span>
      <div class="progress-bar">
        <div class="progress" style="width: {data.cpu_usage}%"></div>
      </div>
    </div>

    <div class="data-item">
      <span class="label">Memory Usage</span>
      <span class="value">{formatValue(data.memory_usage, '%')}</span>
      <div class="progress-bar">
        <div class="progress" style="width: {data.memory_usage}%"></div>
      </div>
    </div>

    <div class="data-item">
      <span class="label">Temperature</span>
      <span class="value">{formatValue(data.temperature, '°C')}</span>
      <div class="progress-bar">
        <div class="progress temp" style="width: {(data.temperature / 100) * 100}%"></div>
      </div>
    </div>

    <div class="data-item">
      <span class="label">Active Processes</span>
      <span class="value">{data.active_processes}</span>
    </div>

    <div class="data-item">
      <span class="label">Network Status</span>
      <span class="value status">{data.network_status}</span>
    </div>

    <div class="data-item">
      <span class="label">Last Updated</span>
      <span class="value time">{new Date(data.timestamp).toLocaleTimeString()}</span>
    </div>
  </div>
</div>

<style>
  .card {
    background: white;
    border-radius: 12px;
    padding: 2rem;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
  }

  h2 {
    margin: 0 0 1.5rem 0;
    color: #333;
  }

  .data-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1.5rem;
  }

  .data-item {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .label {
    font-size: 0.85rem;
    color: #666;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .value {
    font-size: 1.8rem;
    font-weight: 700;
    color: #667eea;
  }

  .value.status {
    color: #2ed573;
    font-size: 1.1rem;
  }

  .value.time {
    font-size: 0.95rem;
    color: #999;
  }

  .progress-bar {
    width: 100%;
    height: 8px;
    background: #f0f0f0;
    border-radius: 4px;
    overflow: hidden;
  }

  .progress {
    height: 100%;
    background: linear-gradient(90deg, #667eea, #764ba2);
    transition: width 0.3s ease;
  }

  .progress.temp {
    background: linear-gradient(90deg, #2ed573, #ff9800, #ff4757);
  }
</style>
