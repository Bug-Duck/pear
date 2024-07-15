<script setup lang="ts">
import { ref } from 'vue'
import Menubar from 'primevue/menubar'
import InputText from 'primevue/inputtext'
import Avatar from 'primevue/avatar'
import Toolbar from 'primevue/toolbar'
import IconField from 'primevue/iconfield'
import InputIcon from 'primevue/inputicon'
import Badge from 'primevue/badge'
import Button from 'primevue/button'
import SplitButton from 'primevue/splitbutton'
import Textarea from 'primevue/textarea'
import FloatLabel from 'primevue/floatlabel'

const items2 = ref([
  {
    label: 'Update',
    icon: 'pi pi-refresh',
  },
  {
    label: 'Delete',
    icon: 'pi pi-times',
  },
])

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

<template>
  <div>
    <Menubar :model="items" class="w-full">
      <template #start>
        <Avatar
          image="https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png"
          shape="circle"
        />
        <p class="ml-3 text-lg">Arona</p>
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

    <Toolbar>
      <template #start>
        <Button icon="pi pi-plus" class="mr-2" severity="secondary" text />
        <Button icon="pi pi-images" class="mr-2" severity="secondary" text />
        <Button icon="pi pi-upload" severity="secondary" text />
      </template>

      <template #center>
        <Textarea
          rows="1"
          cols="35"
          :auto-resize="false"
          placeholder="Message"
          :resizable="false"
        />
      </template>

      <template #end>
        <SplitButton label="Send" :model="items2"></SplitButton
      ></template>
    </Toolbar>
  </div>
</template>
