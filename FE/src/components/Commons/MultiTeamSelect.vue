<template>
  <div class="w-full rounded-lg border bg-card p-6 shadow-sm">
    <div class="flex items-center justify-between mb-4">
      <Label class="text-lg font-semibold">{{ label }}</Label>
      <Button variant="ghost" size="sm" @click="toggleAllTeams">
        {{ allTeamsSelected ? 'Deselect All' : 'Select All' }}
      </Button>
    </div>

    <ScrollArea class="h-[600px] w-full pr-4">
      <div class="space-y-2">
        <div
          v-for="team in props.teams"
          :key="team.id"
          class="flex items-center space-x-2 p-2 rounded-lg hover:bg-accent/50 cursor-pointer"
          @click="toggleTeam(team)"
        >
          <Checkbox :id="team.id" :checked="isSelected(team)" @update:checked="toggleTeam(team)" />
          <Label :for="team.id" class="text-sm cursor-pointer flex-1">
            {{ team.name }}
          </Label>
        </div>
      </div>
    </ScrollArea>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Checkbox } from '@/components/ui/checkbox'
import { Label } from '@/components/ui/label'
import { Button } from '@/components/ui/button'
import { ScrollArea } from '@/components/ui/scroll-area'

const props = defineProps<{
  modelValue: any[]
  teams: { id: string; name: string }[]
  label: string
}>()

const emit = defineEmits(['update:modelValue'])

const selected = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value)
})

const allTeamsSelected = computed(
  () => props.teams.length > 0 && selected.value.length === props.teams.length
)

const isSelected = (team: { id: string; name: string }) =>
  selected.value.some(t => t.id === team.id)

const toggleAllTeams = () => {
  if (allTeamsSelected.value) {
    selected.value = []
  } else {
    selected.value = [...props.teams]
  }
}

const toggleTeam = (team: { id: string; name: string }) => {
  const currentSelected = [...selected.value]
  const index = currentSelected.findIndex(t => t.id === team.id)

  if (index === -1) {
    currentSelected.push(team)
  } else {
    currentSelected.splice(index, 1)
  }

  selected.value = currentSelected
}
</script>
