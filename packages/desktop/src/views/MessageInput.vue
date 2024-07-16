<template>
  <Toolbar>
    <template #start>
      <Button icon="pi pi-plus" class="mr-2" severity="secondary" text />
      <Button icon="pi pi-images" class="mr-2" severity="secondary" text />
      <Button icon="pi pi-upload" severity="secondary" text />
    </template>

    <template #center>
      <Textarea rows="1" cols="35" placeholder="Message" class="resize-none" />
    </template>

    <template #end>
      <Button icon="pi pi-send" label="Send"></Button>
    </template>
  </Toolbar>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import Toolbar from 'primevue/toolbar'
import Button from 'primevue/button'
import Textarea from 'primevue/textarea'
const items = ref([
  {
    label: 'Update',
    icon: 'pi pi-refresh',
  },
  {
    label: 'Delete',
    icon: 'pi pi-times',
  },
])
</script>
