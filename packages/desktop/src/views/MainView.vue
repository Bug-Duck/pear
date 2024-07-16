<script setup lang="ts">
import { ref } from 'vue'
import MessageInput from './MessageInput.vue'

</script>

<template>
  <div class="flex-1 h-full relative overflow-hidden">
    <div class="w-auto max-h-full overflow-y-scroll scrollbar z-20 px-10 pb-[7rem] mt-[30px]">
      <div></div>
      <div>
        a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>
      </div>

      <MessageInput class="w-[37.5rem] z-50 drawer absolute bottom-bar" />
    </div>
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

.scrollbar {
}

.scrollbar::-webkit-scrollbar {
  background: #ffffff0a;
  width: 7px;
}

.scrollbar::-webkit-scrollbar-thumb {
  background: #ffffffa0;
  border-radius: 10px;
  cursor: grab;
}

.scrollbar::-webkit-scrollbar-track {
  background: #ffffff0a;
  border-radius: 10px;
}
</style>
