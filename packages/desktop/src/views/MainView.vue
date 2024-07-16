<script setup lang="ts">
import MessageInput from '@/components/MessageInput.vue'
import MessageItem from '@/components/MessageItem.vue';
</script>

<template>
  <div class="flex-1 h-[calc(100%-30px)] relative">
    <div class="w-auto h-[calc(100%-10px)] scrollbar overflow-y-scroll z-20 px-10 pb-[5rem] mt-[30px]">

      <MessageItem name="Acbox" :date="new Date()" />

    </div>
    <MessageInput class="min-w-[38rem] w-[50%] z-50 drawer absolute bottom-bar" />
  </div>
</template>

<style scoped>
.drawer {
  --p-toolbar-border-radius: 0;
  border-radius: var(--p-content-border-radius) var(--p-content-border-radius) 0
    0;
}

.bottom-bar {
  bottom: 0;
  left: 50%;
  overflow: none;
  transform: translateX(-50%);
}

.message, :deep(* >legend) {
  background: transparent !important;
}

</style>
