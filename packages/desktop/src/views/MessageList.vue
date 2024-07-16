<template>
  <div v-for="message in props.messages" :key="message.id" class="flex flex-row w-full">
    <MessageItem :data="message" class="float-left m-1"></MessageItem>
  </div>
</template>

<script setup lang="ts">
import type { Message } from '../interfaces/message'
import MessageItem from './MessageItem.vue'

const props = defineProps<{
  messages: Message[]
}>()
</script>