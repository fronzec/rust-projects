<!-- CardListComponent.vue -->
<script setup>
import { ref } from 'vue';
import CardDisplayComponent from './CardDisplayComponent.vue';

const cards = ref([
  {
    title: "Tarjeta de ejemplo 1",
    description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
    imagen: "https://picsum.photos/400/200?random=1"
  },
  {
    title: "Tarjeta de ejemplo 2",
    description: "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
    imagen: "https://picsum.photos/400/200?random=2"
  },
  {
    title: "Tarjeta de ejemplo 3",
    description: "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.",
    imagen: "https://picsum.photos/400/200?random=3"
  }
]);
</script>

<template>
  <div class="cards-container">
    <div class="cards-list">
      <CardDisplayComponent
          v-for="(card, index) in cards"
          :key="index"
          :title="card.title"
          :description="card.description"
          :imagen="card.imagen"
      />
    </div>
  </div>
</template>

<!-- CardListComponent.vue -->
<style scoped>
.cards-container {
  width: 100%;
  box-sizing: border-box;
}

.cards-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  width: 100%;
  max-width: 800px;
  margin: 0 auto;
  padding-bottom: 1rem;
}
</style>
