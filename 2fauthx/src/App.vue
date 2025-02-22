<script setup>
import { ref } from "vue";
import CardListComponent from "./components/CardListComponent.vue";
import SearchHeaderComponent from "./components/SearchHeaderComponent.vue";
import FooterNavigationComponent from "./components/FooterNavigationComponent.vue";

const searchQuery = ref("");

const handleSearch = (query) => {
  searchQuery.value = query;
  // Aquí puedes implementar la lógica de búsqueda
};
</script>
<!-- App.vue -->
<template>
  <div class="app-container">
    <div class="header-fixed">
      <SearchHeaderComponent @search="handleSearch" />
    </div>

    <div class="content-wrapper">
      <main class="main-content">
        <CardListComponent />
      </main>
    </div>

    <div class="footer-fixed">
      <FooterNavigationComponent />
    </div>
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: #2d2d2d;
  position: fixed;
  width: 100%;
  top: 0;
  left: 0;
}

.header-fixed {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: 64px;
  z-index: 1000;
  background-color: #2d2d2d;
}

.content-wrapper {
  flex: 1;
  overflow: hidden;
  margin-top: 64px;
  margin-bottom: 60px;
  position: relative;
}

.main-content {
  height: 100%;
  overflow-y: auto;
  -webkit-overflow-scrolling: touch;
  padding: 1rem;
  box-sizing: border-box;
}

.footer-fixed {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  height: 60px;
  z-index: 1000;
  background-color: #2d2d2d;
}
</style>
