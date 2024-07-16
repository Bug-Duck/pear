<template>
  <Toolbar>
    <template #start>
      <Button icon="pi pi-plus" class="mr-2" severity="secondary" text v-tooltip.top="'Editor'" />
      <Button icon="pi pi-images" class="mr-2" severity="secondary" text v-tooltip.top="'Picture'" />
      <Button icon="pi pi-upload" severity="secondary" text v-tooltip.top="'File'"  />

    </template>

    <template #center>
      <InputText placeholder="Message" class="resize-none flex-1" wrap="off"/>
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
import InputText from 'primevue/inputtext'
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


<style scoped>
.p-toolbar:deep(.p-toolbar-center) {
  flex: 1;
}
</style>