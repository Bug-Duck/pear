<script setup lang="ts">
import Menu from 'primevue/menu'
import { ref } from 'vue'

const menus = ref([
  {
    icon: 'pi pi-envelope',
  },
])
</script>

<template>
  <div class="w-1">
    <Menu :model="menus" class="w-10"></Menu>
  </div>
</template>

<style scoped></style>
