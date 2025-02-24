<!-- CardDisplayComponent.vue -->
<template>
  <div class="card">
    <img :src="imagen" :alt="title" class="card-image">
    <div class="card-content">
      <h3 class="card-title">{{ title }}</h3>
      <p class="card-description">{{ description }}</p>
    </div>
  </div>
</template>

<script setup>
defineProps({
  title: String,
  description: String,
  imagen: {
    type: String,
    default: 'https://via.placeholder.com/100x100?text=Icon' // Placeholder image
  }
});
</script>

<style scoped>
.card {
  width: 100%;
  height: 100px; /* Adjusted height */
  background: #383838;
  border-radius: 12px;
  overflow: hidden;
  display: flex;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

.card-image {
  width: 100px; /* Adjusted width */
  height: 100px; /* Adjusted height */
  object-fit: cover;
}

.card-content {
  flex: 1;
  padding: 1rem;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}

.card-title {
  margin: 0;
  color: #e0e0e0;
  font-size: 1rem; /* Adjusted font size */
  font-weight: 600;
}

.card-description {
  margin: 0.5rem 0 0 0;
  color: #b0b0b0;
  font-size: 0.8rem; /* Adjusted font size */
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2; /* Adjusted line clamp */
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>