<script setup lang="ts">
import { ref } from 'vue'
import MessageInput from './MessageInput.vue'

</script>

<template>
  <div class="flex-1 h-[calc(100%-30px)] relative ">
    <div class="w-auto h-[calc(100%-10px)] max-[] scrollbar overflow-y-scroll z-20 px-10 pb-[5rem] mt-[30px]">
      <!-- <div>
        a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>a<br>
      </div> -->


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

.scrollbar {
}

.scrollbar::-webkit-scrollbar {
  background: transparent;
  width: 7px;
}

.scrollbar::-webkit-scrollbar-thumb {
  background: #ffffffa0;
  border-radius: 10px;
  cursor: grab;
}

.scrollbar::-webkit-scrollbar-track {
  background: transparent;
  border-radius: 10px;
}
</style>
