<script>
  import { voiceStore } from '../stores';
  import { activateVoice, deactivateVoice } from '../api';

  let isActive = false;

  async function toggleVoice() {
    try {
      if (isActive) {
        await deactivateVoice();
        isActive = false;
        voiceStore.update(s => ({ ...s, active: false }));
      } else {
        await activateVoice();
        isActive = true;
        voiceStore.update(s => ({ ...s, active: true }));
        startListening();
      }
    } catch (error) {
      console.error('Voice toggle error:', error);
    }
  }

  function startListening() {
    const recognition = new (window.SpeechRecognition || window.webkitSpeechRecognition)();
    recognition.continuous = true;
    recognition.interimResults = true;

    recognition.onstart = () => {
      voiceStore.update(s => ({ ...s, listening: true }));
    };

    recognition.onresult = (event) => {
      let transcript = '';
      for (let i = event.resultIndex; i < event.results.length; i++) {
        transcript += event.results[i][0].transcript;
      }
      voiceStore.update(s => ({ ...s, transcript }));
    };

    recognition.onerror = (event) => {
      console.error('Speech recognition error', event.error);
      voiceStore.update(s => ({ ...s, listening: false }));
    };

    if (isActive) {
      recognition.start();
    }
  }
</script>

<div class="card">
  <h2>Voice Activation</h2>
  <button 
    class="toggle-btn" 
    class:active={isActive}
    on:click={toggleVoice}
  >
    {#if isActive}
      🔴 Listening...
    {:else}
      🎤 Activate Jarvis
    {/if}
  </button>
  
  {#if isActive}
    <div class="status-indicator">
      <span class="pulse"></span>
      Voice is active - Say your command
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

  .toggle-btn {
    width: 100%;
    padding: 1.5rem;
    font-size: 1.2rem;
    border: none;
    border-radius: 8px;
    background: #667eea;
    color: white;
    cursor: pointer;
    transition: all 0.3s ease;
    font-weight: 600;
  }

  .toggle-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 5px 15px rgba(102, 126, 234, 0.4);
  }

  .toggle-btn.active {
    background: #ff4757;
    animation: pulse-animation 2s infinite;
  }

  .status-indicator {
    margin-top: 1rem;
    padding: 1rem;
    background: #f0f9ff;
    border-left: 4px solid #667eea;
    border-radius: 4px;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.95rem;
    color: #333;
  }

  .pulse {
    width: 12px;
    height: 12px;
    background: #ff4757;
    border-radius: 50%;
    animation: pulse 1.5s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% {
      opacity: 1;
      transform: scale(1);
    }
    50% {
      opacity: 0.5;
      transform: scale(1.2);
    }
  }

  @keyframes pulse-animation {
    0%, 100% {
      box-shadow: 0 0 0 0 rgba(255, 71, 87, 0.7);
    }
    50% {
      box-shadow: 0 0 0 10px rgba(255, 71, 87, 0);
    }
  }
</style>
