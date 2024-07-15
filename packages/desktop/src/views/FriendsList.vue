<template>
  <TieredMenu :model="items" class="h-full"></TieredMenu>
</template>

<script setup lang="ts">
import TieredMenu from 'primevue/tieredmenu'
import { ref } from 'vue'

interface Friend {
  name: string
}

const friends = ref<Array<Friend>>([
  {
    name: 'Test'
  }
])

const items = friends.value.map((friend) => {
  return {
    label: friend.name
  }
})
</script>
