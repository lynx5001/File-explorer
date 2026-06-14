import { invoke } from '@tauri-apps/api/core';

/**
 * Initializes the frontend application.
 * This function runs once the DOM is fully loaded.
 */
async function setup() {
  console.log("Frontend initialized. Attempting to call Rust backend...");

  try {
    // Invoke a simple 'greet' command from the Rust backend to confirm IPC is working.
    const message = await invoke<string>('greet', { name: 'World' });
    console.log(message);
  } catch (error) {
    console.error("Error invoking 'greet' command from Rust:", error);
  }
}
document.addEventListener("DOMContentLoaded", setup);