<script setup>
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";

const greetMsg = ref("");
const name = ref("");

// contiene el valor actual mostrado en el display
const currentOutput = ref("0");

// (opcional, si quieres mantenerla) referencia a la operación actual
const operation = ref(null);

/* funciones para controlar el display y botones */
function clear() {
  currentOutput.value = "0";
  operation.value = null;
}

function appendNumber(number) {
  if (currentOutput.value === "0" || operation.value === "result") {
    currentOutput.value = String(number);
  } else {
    currentOutput.value += String(number);
  }
  operation.value = null;
}

function appendDot() {
  // evitar múltiples puntos seguidos en la última porción numérica
  // si el último token numérico ya tiene '.', no añadir otro
  // enfoque simple: evitar más de un '.' en toda la cadena si prefieres
  // una solución más robusta requiere parsing.
  const lastChar = currentOutput.value.slice(-1);
  if (["+", "-", "*", "/"].includes(lastChar)) {
    return;
  }
  const parts = currentOutput.value.split(/[\+\-\*\/]/);
  const lastNum = parts[parts.length - 1];
  if (!lastNum.includes(".")) {
    currentOutput.value += ".";
  }
}

function setOperator(op) {
  const last = currentOutput.value.slice(-1);
  if (["+", "-", "*", "/"].includes(last)) {
    // reemplaza el operador final por el nuevo
    currentOutput.value = currentOutput.value.slice(0, -1) + op;
  } else {
    currentOutput.value += op;
  }
}

async function evaluate() {
  try {
    // envía la expresión completa al comando Rust 'calculate'
    const result = await invoke("calculate", { operation: currentOutput.value });
    currentOutput.value = String(result);
    operation.value = "result";
  } catch (e) {
    // muestra error simple en el display
    currentOutput.value = "Error";
    operation.value = "result";
    console.error(e);
  }
}

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main class="container">
    <!-- Calculadora: display enlazado y botones con handlers -->
    <section class="calculator" style="margin-top: 2rem; display: inline-block; text-align: center;">
      <div class="calc-display" style="margin-bottom: 0.75rem;">
        <input id="calc-display" type="text" :value="currentOutput" readonly
               style="width: 14rem; padding: 0.6rem 0.8rem; font-size: 1.25rem; text-align: right; border-radius: 8px; border: 1px solid #ccc; background: #0f0f0f;" />
      </div>

      <div class="calc-grid" style="display: grid; grid-template-columns: repeat(4, 3.5rem); gap: 0.5rem; justify-content: center;">
        <button type="button" class="btn clear" style="grid-column: span 2;" @click="clear()">C</button>
        <button type="button" class="btn op" @click="setOperator('/')">/</button>
        <button type="button" class="btn op" @click="setOperator('*')">*</button>

        <button type="button" class="btn num" @click="appendNumber(7)">7</button>
        <button type="button" class="btn num" @click="appendNumber(8)">8</button>
        <button type="button" class="btn num" @click="appendNumber(9)">9</button>
        <button type="button" class="btn op" @click="setOperator('-')">-</button>

        <button type="button" class="btn num" @click="appendNumber(4)">4</button>
        <button type="button" class="btn num" @click="appendNumber(5)">5</button>
        <button type="button" class="btn num" @click="appendNumber(6)">6</button>
        <button type="button" class="btn op" @click="setOperator('+')">+</button>

        <button type="button" class="btn num" @click="appendNumber(1)">1</button>
        <button type="button" class="btn num" @click="appendNumber(2)">2</button>
        <button type="button" class="btn num" @click="appendNumber(3)">3</button>
        <button type="button" class="btn equals" style="grid-row: span 2; height: calc(3.5rem * 2 + 0.5rem);" @click="evaluate()">=</button>

        <button type="button" class="btn num" style="grid-column: span 2;" @click="appendNumber(0)">0</button>
        <button type="button" class="btn num" @click="appendDot()">.</button>
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
