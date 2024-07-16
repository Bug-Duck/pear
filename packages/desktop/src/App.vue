<script setup lang="ts">
import { RouterLink, RouterView, useRouter } from 'vue-router'
import Button from 'primevue/button'
import { computed } from 'vue';

const router = useRouter()
const routeName = computed(() => router.currentRoute.value.name)
console.log(router.currentRoute, routeName)
</script>

<template>
  <div class="w-full h-[20px] absolute top-0 left-0 drag-region"></div>
  <div class="flex w-full h-full items-start">
    <div
      class="navbar w-[4rem] bg-[#1d1d1d] h-full flex justify-between flex-col py-4 sidebar-border"
    >
      <div class="w-full flex flex-col items-center bg-[transparent]">
        <div class="gap-2 flex flex-col">
          <Button as="router-link" icon="pi pi-home" aria-label="Home" text v-tooltip="'Home'" to="/" :severity="routeName != 'home' ? 'secondary' : null" size="large" />
          <Button as="router-link" icon="pi pi-address-book" aria-label="Contact" text v-tooltip="'Contact'" :severity="routeName != 'contact' ? 'secondary' : null" size="large" />
          <Button as="router-link" icon="pi pi-bookmark" aria-label="Bookmark" text v-tooltip="'Bookmark'" :severity="routeName != 'bookmark' ? 'secondary' : null" size="large" />
        </div>
      </div>
      <div class="w-full flex flex-col items-center">
        <Button
        as="router-link" icon="pi pi-cog"
          aria-label="Settings"
          text
          v-tooltip="'Settings'"
          to="/settings"
size="large"
          :severity="routeName != 'settings' ? 'secondary' : null"
        />
      </div>
    </div>
    <div class="page-container flex-1 h-full overflow-scroll-y">
      <RouterView />
    </div>
  </div>
</template>

<style scoped>
.sidebar-border {
  border: 0px solid var(--p-toolbar-border-color);
  border-right-width: 1px;
}

.drag-region {
  -webkit-app-region: drag;
}
</style>
