<!-- FooterNavigation.vue -->
<script setup>
const menuItems = [
  { icon: '🏠', label: 'Inicio' },
  { icon: '🔍', label: 'Explorar' },
  { icon: '➕', label: 'Nuevo' },
  { icon: '❤️', label: 'Favoritos' },
  { icon: '👤', label: 'Perfil' }
];
</script>

<template>
  <nav class="footer-nav">
    <button
      v-for="(item, index) in menuItems"
      :key="index"
      class="nav-item"
    >
      <span class="icon">{{ item.icon }}</span>
      <span class="label">{{ item.label }}</span>
    </button>
  </nav>
</template>

<style scoped>
.footer-nav {
  height: 60px;
  display: flex;
  justify-content: space-around;
  align-items: center;
  padding: 0.5rem 0;
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  background: none;
  border: none;
  color: #666;
  cursor: pointer;
  padding: 0.25rem;
  transition: color 0.3s ease;
}

.nav-item:hover {
  color: #007AFF;
}

.icon {
  font-size: 1.5rem;
}

.label {
  font-size: 0.7rem;
  margin-top: 0.2rem;
}

@media (prefers-color-scheme: dark) {
  .nav-item {
    color: #999;
  }

  .nav-item:hover {
    color: #0A84FF;
  }
}
</style>