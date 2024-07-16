<template>
  <Menubar :model="items" class="w-full">
    <template #start>
      <!-- <Avatar image="https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png" shape="circle" />
      <p class="ml-3 text-lg">Arona</p> -->
    </template>
    <template #item="{ item, props, hasSubmenu, root }">
      <a v-ripple class="flex items-center" v-bind="props.action">
        <span :class="item.icon" />
      </a>
    </template>
    <template #end>
      <InputText placeholder="Search" type="text" class="w-32 sm:w-auto" />
    </template>
  </Menubar>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import Menubar from 'primevue/menubar'
import InputText from 'primevue/inputtext'
import Avatar from 'primevue/avatar'

const items = ref([
  {
    icon: 'pi pi-home',
  },
  {
    icon: 'pi pi-star',
  },
  {
    icon: 'pi pi-search'
  },
  {
    icon: 'pi pi-envelope',
  },
])
</script>