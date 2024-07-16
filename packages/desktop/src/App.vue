<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import Button from 'primevue/button'
import Avatar from 'primevue/avatar'
</script>

<template>
  <div class="flex w-full h-full items-start">
    <div
      class="navbar w-[4rem] bg-[#ffffff0a] h-full flex justify-between flex-col py-4 sidebar-border"
    >
      <div class="w-full flex flex-col items-center">
        <div class="gap-2 flex flex-col">
          <Button as="router-link" icon="pi pi-home" aria-label="Home" text v-tooltip="'Home'" to="/" />
          <Button as="router-link" icon="pi pi-address-book" aria-label="Contact" text v-tooltip="'Contact'" />
          <Button as="router-link" icon="pi pi-bookmark" aria-label="Bookmark" text v-tooltip="'Bookmark'" />
        </div>
      </div>
      <div class="w-full flex flex-col items-center">
        <Button
        as="router-link" icon="pi pi-cog"
          aria-label="Settings"
          text
          v-tooltip="'Settings'"
          to="/settings"
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
</style>
