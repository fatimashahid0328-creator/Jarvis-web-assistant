# MVVM Architecture Guide

## Overview

The Jarvis Web Assistant implements the **Model-View-ViewModel (MVVM)** pattern for clean separation of concerns and reactive data binding.

## Architecture Layers

### 1. **View Layer** (Svelte Components)

Responsible for rendering UI and user interactions:

- `VoiceActivator.svelte` - Voice control UI
- `DataDisplay.svelte` - Real-time metrics display
- `CommandProcessor.svelte` - Command input and history
- `App.svelte` - Main container component

**Characteristics:**
- Pure presentation logic
- No business logic
- Reactive to store changes
- User input handling

### 2. **ViewModel Layer** (Svelte Stores)

Manages application state and business logic:

```javascript
// src/stores.js
export const voiceStore = writable({
  active: false,
  listening: false,
  transcript: ''
});

export const dataStore = writable(null);

export const commandStore = writable({
  history: [],
  current: null
});
```

**Responsibilities:**
- State management
- Data transformation
- Command handling
- Reactive updates

### 3. **Model Layer** (Backend API + API Client)

Handles data and business logic:

**Backend (Rust):**
- Voice command processing
- System data collection
- Real-time data updates
- Command execution

**API Client (src/api.js):**
- HTTP communication
- Data formatting
- Error handling

## Data Flow

```
User Interaction
        ↓
    View (Svelte Component)
        ↓
    ViewModel (Store)
        ↓
    Model (API Client)
        ↓
    Backend (Rust)
        ↓
    Model Response
        ↓
    ViewModel Update
        ↓
    View Re-render
```

## Example: Voice Command Execution

### Step 1: View Interaction
```svelte
<!-- CommandProcessor.svelte -->
<form on:submit={handleCommand}>
  <input bind:value={commandInput} />
  <button>Send</button>
</form>
```

### Step 2: ViewModel Processing
```javascript
// src/stores.js
async function handleCommand(e) {
  const response = await processCommand(commandInput);
  commandStore.update(s => ({
    ...s,
    history: [response, ...s.history]
  }));
}
```

### Step 3: Model Communication
```javascript
// src/api.js
export async function processCommand(commandText) {
  const response = await api.post('/command', {
    text: commandText
  });
  return response.data;
}
```

### Step 4: Backend Processing
```rust
// src/voice.rs
pub fn process_voice_command(command: &str) -> CommandResponse {
  // Process command
  // Return response
}
```

### Step 5: Update View
```svelte
<!-- CommandProcessor.svelte -->
commandStore.subscribe(value => {
  commandHistory = value.history;
});
```

## Benefits of MVVM

✅ **Separation of Concerns** - Clean boundaries between layers
✅ **Testability** - Each layer can be tested independently
✅ **Reusability** - Components and stores are reusable
✅ **Maintainability** - Changes in one layer don't affect others
✅ **Scalability** - Easy to add new features
✅ **Reactive Binding** - Automatic UI updates with Svelte

## Store Subscription Pattern

```javascript
// Subscribe to store changes
voiceStore.subscribe(value => {
  console.log('Voice state changed:', value);
});

// Update store
voiceStore.update(v => ({ ...v, active: true }));

// Set entire store
voiceStore.set({ active: true, listening: true, transcript: '' });
```

## Adding New Features

### Adding a new command type:

1. **Update ViewModel** (stores.js):
```javascript
export const newFeatureStore = writable({ /* initial state */ });
```

2. **Create View Component** (components/):
```svelte
<script>
  import { newFeatureStore } from '../stores';
</script>
```

3. **Add API Endpoint** (src/main.rs):
```rust
.route("/api/new-endpoint", web::post().to(handler))
```

4. **Update API Client** (src/api.js):
```javascript
export async function newFeature() {
  const response = await api.post('/new-endpoint');
  return response.data;
}
```

## State Management Best Practices

1. **Keep stores focused** - One responsibility per store
2. **Immutable updates** - Always create new objects
3. **Normalize data** - Avoid nested structures
4. **Subscribe wisely** - Unsubscribe when components unmount
5. **Type safety** - Use JSDoc or TypeScript for stores

## Performance Optimization

- Use `$` auto-subscription in Svelte
- Memoize expensive computations
- Batch store updates
- Lazy load components
- Debounce frequent updates

---

For more Svelte store patterns, see: https://svelte.dev/docs/svelte-store
