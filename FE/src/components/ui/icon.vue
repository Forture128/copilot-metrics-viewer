<template>
  <component :is="iconComponent" class="lucide" :class="props.class" />
</template>

<script setup lang="ts">
import { computed } from 'vue'
import * as lucideIcons from 'lucide-vue-next'

const props = defineProps<{
  name: string
  class?: string
}>()

const iconComponent = computed(() => {
  const iconName = props.name
    .split('-')
    .map(part => part.charAt(0).toUpperCase() + part.slice(1))
    .join('')
  return lucideIcons[`${iconName}Icon` as keyof typeof lucideIcons]
})
</script>
