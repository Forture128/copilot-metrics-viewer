<template>
  <div class="rounded-lg border bg-card p-6 shadow-sm">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-semibold">Team Members</h3>
      <span class="text-sm text-muted-foreground"> {{ getAllMembers.length }} members </span>
    </div>

    <ScrollArea>
      <div class="grid grid-cols-2 gap-4">
        <div
          v-for="member in getAllMembers"
          :key="member.id"
          class="flex items-center space-x-3 p-2 rounded-lg hover:bg-accent/50"
        >
          <img
            :src="member.avatar_url"
            :alt="member.login"
            class="h-10 w-10 rounded-full border-2 border-border"
          />
          <div class="flex flex-col">
            <span class="text-sm font-medium">{{ member.name || member.login }}</span>
            <div class="flex space-x-2">
              <a
                :href="member.html_url"
                target="_blank"
                class="text-xs text-muted-foreground hover:underline"
              >
                Profile
              </a>
              <a
                :href="member.repos_url"
                target="_blank"
                class="text-xs text-muted-foreground hover:underline"
              >
                Repos
              </a>
            </div>
          </div>
        </div>
      </div>
    </ScrollArea>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { ScrollArea } from '@/components/ui/scroll-area'
import type { Members, SimpleUser } from '@/model/Members'

const props = defineProps<{
  members: Members[]
}>()

const getAllMembers = computed(() => {
  return props.members.reduce((acc: SimpleUser[], memberObj) => {
    return acc.concat(memberObj.members)
  }, [] as SimpleUser[])
})
</script>
