<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

/* contains the current output value, initialized with 0*/
const currentOutput = ref("0");

/* contains the operation */
const operation= ref(null);



async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main class="container">
    <!-- Basic calculator UI (static, no bindings) -->
    <section class="calculator" style="margin-top: 2rem; display: inline-block; text-align: center;">
      <div class="calc-display" style="margin-bottom: 0.75rem;">
        <input id="calc-display" type="text" value="0" readonly
               style="width: 14rem; padding: 0.6rem 0.8rem; font-size: 1.25rem; text-align: right; border-radius: 8px; border: 1px solid #ccc; background: #0f0f0f;" />
      </div>

      <div class="calc-grid" style="display: grid; grid-template-columns: repeat(4, 3.5rem); gap: 0.5rem; justify-content: center;">
        <button type="button" class="btn clear" style="grid-column: span 2;">C</button>
        <button type="button" class="btn op">/</button>
        <button type="button" class="btn op">*</button>

        <button type="button" class="btn num">7</button>
        <button type="button" class="btn num">8</button>
        <button type="button" class="btn num">9</button>
        <button type="button" class="btn op">-</button>

        <button type="button" class="btn num">4</button>
        <button type="button" class="btn num">5</button>
        <button type="button" class="btn num">6</button>
        <button type="button" class="btn op">+</button>

        <button type="button" class="btn num">1</button>
        <button type="button" class="btn num">2</button>
        <button type="button" class="btn num">3</button>
        <button type="button" class="btn equals" style="grid-row: span 2; height: calc(3.5rem * 2 + 0.5rem);">=</button>

        <button type="button" class="btn num" style="grid-column: span 2;">0</button>
        <button type="button" class="btn num">.</button>
        <!-- equals button spans the last column above -->
      </div>
    </section>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
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

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
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

#greet-input {
  margin-right: 5px;
}

.calculator {
  margin-top: 2rem;
  display: inline-block;
  text-align: center;
}

.calc-display {
  margin-bottom: 0.75rem;
}

.calc-display input {
  width: 14rem;
  padding: 0.6rem 0.8rem;
  font-size: 1.25rem;
  text-align: right;
  border-radius: 8px;
  border: 1px solid #ccc;
  background: #fff;
}

.calc-grid {
  display: grid;
  grid-template-columns: repeat(4, 3.5rem);
  gap: 0.5rem;
  justify-content: center;
}

.btn {
  padding: 0.6rem;
  font-size: 1.125rem;
  border-radius: 8px;
  border: 1px solid transparent;
  background-color: #3f3f3f;
  transition: background-color 0.25s, transform 0.1s;
}

.btn:hover {
  background-color: #e0e0e0;
}

.btn:active {
  transform: scale(0.98);
}

.btn.clear {
  grid-column: span 2;
}

.btn.equals {
  grid-row: span 2;
  height: calc(3.5rem * 2 + 0.5rem);
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
