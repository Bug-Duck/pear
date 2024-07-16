<template>
  <div class="w-full">
    <Avatar class="float-left h-6 w-6 mx-3 mt-2" />
    <div class="float-left">
      <span class=" text-[10px]">
        {{ data.name }}
      </span>
      <br/>
      <Card class="float-right w-full">
        <template #content>
          {{ data.content }}
        </template>
      </Card>
    </div>
  </div>
</template>

<script setup lang="ts">
import Avatar from 'primevue/avatar'
import Card from 'primevue/card'
import type { Message } from '../interfaces/message'

const props = defineProps<{
  data: Message
}>()
</script>