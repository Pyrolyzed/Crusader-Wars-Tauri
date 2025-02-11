<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  let attilaPath = $state("");
  let ck3Path = $state("");
  let isValid = $state("");
  let isCkRunning = $state("");
  let isAttilaRunning = $state("");

  async function verifyGamePaths() {
    if (attilaPath == "" || ck3Path == "") return false;

    isValid = await invoke("verify_game_paths", { ckStr: ck3Path, attilaStr: attilaPath });
    return isValid;
  }

  async function getAttilaRunning() {
    let isRunning = await invoke("is_attila_running", { });
    isAttilaRunning = isRunning;
    return isRunning;
  }

  async function getCkRunning() {
    let isRunning = await invoke("is_ck_running", { });
    isCkRunning = isRunning;
    return isRunning;
  }

  async function ck3PathButtonPressed(event: Event) {
    event.preventDefault();
    ck3Path = await openFileSelectorDialog();
    verifyGamePaths();
  }

  async function attilaPathButtonPressed(event: Event) {
    event.preventDefault();
    attilaPath = await openFileSelectorDialog();
    verifyGamePaths();
  }

  async function openFileSelectorDialog() {
    const result = await open({
      multiple: false,
      directory: false,
    });
    return result;
  }

  async function playButtonPressed(event: Event) {
    if (!verifyGamePaths()) {
      return;
    }

    if (getAttilaRunning()) {
      await invoke("close_attila", { });
    }

    let isRunning = await getCkRunning();
    if (!isRunning) {
      await invoke("launch_crusader_kings", { });
    }

    await invoke("wait_for_battle_start", { });
  }
</script>

<main class="container">
  <h1>Crusader Wars Launcher</h1>
  <h2>{isValid}</h2>
  <button onclick={ck3PathButtonPressed}>CK3 Path</button>
  <button onclick={attilaPathButtonPressed}>Attila Path</button>
  <button onclick={playButtonPressed}>Play</button>
  <p>Attila Path: {attilaPath}</p>
  <p>CK Path: {ck3Path}</p>
  {#await getCkRunning()}
	  <p> waiting </p>
  {:then data}
    <p>CK Running: {data}</p>
  {:catch error}
     <p style="color: red">{error.message}</p>
  {/await}
  {#await getAttilaRunning()}
	  <p> waiting </p>
  {:then data}
    <p>Attila Running: {data}</p>
  {:catch error}
     <p style="color: red">{error.message}</p>
  {/await}
</main>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.row {
  display: flex;
  justify-content: center;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
