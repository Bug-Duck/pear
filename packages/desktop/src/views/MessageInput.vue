<template>
  <Toolbar>
    <template #start>
      <Button icon="pi pi-plus" class="mr-2" severity="secondary" text />
      <Button icon="pi pi-images" class="mr-2" severity="secondary" text />
      <Button icon="pi pi-upload" severity="secondary" text />
    </template>

    <template #center>
      <Textarea rows="1" cols="35" :auto-resize="false" placeholder="Message" :resizable="false" />
    </template>

    <template #end>
      <SplitButton label="Send" :model="items"></SplitButton>
    </template>
  </Toolbar>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import Toolbar from 'primevue/toolbar'
import IconField from 'primevue/iconfield'
import InputIcon from 'primevue/inputicon'
import Badge from 'primevue/badge'
import Button from 'primevue/button'
import SplitButton from 'primevue/splitbutton'
import Textarea from 'primevue/textarea'
import FloatLabel from 'primevue/floatlabel'

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