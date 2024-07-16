<script setup lang="ts">
import { ref } from 'vue'
import MessageInput from './MessageInput.vue'
import MessageList from './MessageList.vue'
import TopMenubar from './TopMenubar.vue'
import FriendsList from './FriendsList.vue'
</script>

<template>
  <div class="flex-1 h-full relative">
    <div class="h-full w-auto max-h-full overflow-y-scroll scrollbar z-20">
      <div class="mb-16">
        <MessageList :messages="[
          {
            name: '野兽先辈',
            id: '114514',
            date: '1145.1.4',
            time: '11:45',
            content: '啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊'
          },
          {
            name: '野兽先辈',
            id: '114514',
            date: '1145.1.4',
            time: '11:45',
            content: '啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊'
          },
          {
            name: '野兽先辈',
            id: '114514',
            date: '1145.1.4',
            time: '11:45',
            content: '啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊'
          },
          {
            name: '野兽先辈',
            id: '114514',
            date: '1145.1.4',
            time: '11:45',
            content: '啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊'
          },
          {
            name: '野兽先辈',
            id: '114514',
            date: '1145.1.4',
            time: '11:45',
            content: '啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊'
          },
          {
            name: '野兽先辈',
            id: '114514',
            date: '1145.1.4',
            time: '11:45',
            content: '啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊'
          },
          {
            name: '野兽先辈',
            id: '114514',
            date: '1145.1.4',
            time: '11:45',
            content: '啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊'
          },
          {
            name: '野兽先辈',
            id: '114514',
            date: '1145.1.4',
            time: '11:45',
            content: '啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊'
          },
        ]"/>
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
  width: 8px;
}

.scrollbar::-webkit-scrollbar-thumb {
  background: #ffffffa0;
  border-radius: 10px;
}

.scrollbar::-webkit-scrollbar-track {
  background: #ffffff0a;
  border-radius: 10px;
}
</style>
