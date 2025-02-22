<!-- SearchHeaderComponent.vue -->
<template>
  <header class="header">
    <div class="search-container">
      <input
          v-model="searchText"
          @input="handleInput"
          type="search"
          placeholder="Buscar..."
          class="search-input"
      />
    </div>
  </header>
</template>

<script setup>
import { ref } from 'vue';

const searchText = ref('');
const emit = defineEmits(['search']);

const handleInput = () => {
  emit('search', searchText.value);
};
</script>

<style scoped>
.header {
  width: 100%;
  height: 64px;
  background-color: #383838;
  display: flex;
  align-items: center;
  padding: 0;
}

.search-container {
  width: 100%;
  height: 100%;
  padding: 0.5rem;
  box-sizing: border-box;
}

.search-input {
  width: 100%;
  height: 100%;
  padding: 0 1.5rem;
  border: none;
  background-color: #2d2d2d;
  color: #e0e0e0;
  font-size: 1rem;
  outline: none;
  box-sizing: border-box;
}

.search-input::placeholder {
  color: #888;
}

.search-input:focus {
  background-color: #333333;
}
</style>