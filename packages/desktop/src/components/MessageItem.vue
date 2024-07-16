<script setup lang="ts">
import Fieldset from 'primevue/fieldset'

defineProps<{
  name: string
  date: Date
}>()

const emit = defineEmits<{
  onDetails: []
}>()
</script>

<template>
  <Fieldset :legend="name" class="message select-none">
    <slot></slot>
    <p class="text-xs font-mono text-gray-600 mt-2 select-none">
      {{ date.getUTCFullYear() }}-{{ date.getUTCMonth().toString().padStart(2, '0') }}-{{
        date.getUTCDay().toString().padStart(2, '0')
      }}
      {{ date.getUTCHours().toString().padStart(2, '0') }}:{{ date.getUTCMinutes().toString().padStart(2, '0') }}:{{
        date.getUTCSeconds().toString().padStart(2, '0')
      }} (UTC+0000)
      <a class="hover:underline hover:cursor-pointer" @click="emit('onDetails')"
        >[Details]</a
      >
    </p>
  </Fieldset>
</template>

<style scoped>
.message,
:deep(* > legend) {
  background: transparent !important;
}
</style>
