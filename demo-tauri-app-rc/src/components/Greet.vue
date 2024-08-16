<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const otherMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
  otherMsg.value = await invoke("other_function", {name: name.value});
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>
  <p>{{ otherMsg }}</p>
</template>
