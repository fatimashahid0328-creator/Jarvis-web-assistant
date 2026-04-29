<script>
  import { onMount } from 'svelte';
  import VoiceActivator from './components/VoiceActivator.svelte';
  import DataDisplay from './components/DataDisplay.svelte';
  import CommandProcessor from './components/CommandProcessor.svelte';
  import { voiceStore, dataStore } from './stores';

  let voiceActive = false;
  let realTimeData = null;

  voiceStore.subscribe(value => {
    voiceActive = value.active;
  });

  dataStore.subscribe(value => {
    realTimeData = value;
  });

  onMount(() => {
    console.log('Jarvis Web Assistant initialized');
  });
</script>

<main>
  <div class="container">
    <header>
      <h1>🎤 Jarvis Web Assistant</h1>
      <p>Say "Jarvis" to activate • Real-time voice-enabled assistant</p>
    </header>

    <div class="grid">
      <VoiceActivator />
      <CommandProcessor />
      {#if realTimeData}
        <DataDisplay data={realTimeData} />
      {/if}
    </div>
  </div>
</main>

<style global>
  :global(body) {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    margin: 0;
    padding: 0;
    min-height: 100vh;
  }

  main {
    padding: 2rem;
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
  }

  header {
    text-align: center;
    color: white;
    margin-bottom: 3rem;
  }

  h1 {
    font-size: 2.5rem;
    margin: 0 0 0.5rem 0;
    font-weight: 700;
  }

  p {
    font-size: 1.1rem;
    opacity: 0.9;
    margin: 0;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
  }
</style>
