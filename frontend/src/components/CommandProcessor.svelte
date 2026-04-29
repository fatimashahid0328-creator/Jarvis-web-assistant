<script>
  import { commandStore, dataStore } from '../stores';
  import { processCommand } from '../api';

  let commandInput = '';
  let commandHistory = [];
  let isProcessing = false;

  commandStore.subscribe(value => {
    commandHistory = value.history;
  });

  async function handleCommand(e) {
    e.preventDefault();
    if (!commandInput.trim()) return;

    isProcessing = true;
    try {
      const response = await processCommand(commandInput);
      
      commandStore.update(s => ({
        ...s,
        history: [{ command: commandInput, response: response.response, success: response.success }, ...s.history],
        current: response
      }));

      if (response.data) {
        dataStore.set(response.data);
      }

      commandInput = '';
    } catch (error) {
      console.error('Command processing error:', error);
      commandStore.update(s => ({
        ...s,
        history: [{ command: commandInput, response: 'Error processing command', success: false }, ...s.history]
      }));
    } finally {
      isProcessing = false;
    }
  }
</script>

<div class="card">
  <h2>💬 Command Processor</h2>
  
  <form on:submit={handleCommand}>
    <input
      type="text"
      placeholder="Enter command (e.g., 'status', 'time', 'system')"
      bind:value={commandInput}
      disabled={isProcessing}
    />
    <button type="submit" disabled={isProcessing}>
      {isProcessing ? '⏳ Processing...' : '📤 Send'}
    </button>
  </form>

  {#if commandHistory.length > 0}
    <div class="history">
      <h3>Command History</h3>
      {#each commandHistory.slice(0, 5) as item, i}
        <div class="history-item" class:error={!item.success}>
          <span class="cmd">➜ {item.command}</span>
          <span class="res">{item.response}</span>
        </div>
      {/each}
    </div>
  {/if}
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

  form {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
  }

  input {
    flex: 1;
    padding: 0.75rem 1rem;
    border: 2px solid #e0e0e0;
    border-radius: 8px;
    font-size: 0.95rem;
    transition: border-color 0.3s ease;
  }

  input:focus {
    outline: none;
    border-color: #667eea;
  }

  input:disabled {
    background: #f5f5f5;
    color: #ccc;
  }

  button {
    padding: 0.75rem 1.5rem;
    background: #667eea;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 600;
    transition: all 0.3s ease;
  }

  button:hover:not(:disabled) {
    background: #764ba2;
    transform: translateY(-2px);
  }

  button:disabled {
    background: #ccc;
    cursor: not-allowed;
  }

  .history {
    border-top: 1px solid #e0e0e0;
    padding-top: 1.5rem;
  }

  h3 {
    margin: 0 0 1rem 0;
    font-size: 0.9rem;
    color: #666;
    text-transform: uppercase;
  }

  .history-item {
    padding: 0.75rem;
    background: #f9f9f9;
    border-radius: 6px;
    margin-bottom: 0.5rem;
    border-left: 3px solid #667eea;
  }

  .history-item.error {
    border-left-color: #ff4757;
  }

  .cmd {
    display: block;
    font-weight: 600;
    color: #333;
    font-size: 0.9rem;
    margin-bottom: 0.25rem;
  }

  .res {
    display: block;
    font-size: 0.85rem;
    color: #666;
  }
</style>
