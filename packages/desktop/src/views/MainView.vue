<script setup lang="ts">
import { ref } from 'vue'
import MessageInput from './MessageInput.vue'
import TopMenubar from './TopMenubar.vue'
import FriendsList from './FriendsList.vue'
</script>

<template>
  <header class="sticky mt-0">
    <TopMenubar></TopMenubar>
  </header>
  <div>
    <div class="flex h-full float-left w-[25%] bg-gary-100">
      <FriendsList class="float-left w-full h-full"/>
    </div>
    <div class="float-right min-w-[75%]">
      <div class="bg-gary-100">
        www
      </div>
      <MessageInput class="absolute bottom-0 w-[75%]"/>
    </div>
  </div>
</template>
