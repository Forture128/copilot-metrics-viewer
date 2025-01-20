<template>
  <div class="w-64 border-r h-full p-4 space-y-4">
    <div class="space-y-2">
      <Label class="text-sm font-medium">Department</Label>
      <Select v-model="selectedDept" @update:modelValue="handleDepartmentChange">
        <SelectTrigger>
          <SelectValue :placeholder="selectedDept || 'Select Department'" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem v-for="dept in departments" :key="dept" :value="dept">
            {{ dept }}
          </SelectItem>
        </SelectContent>
      </Select>
    </div>

    <div class="space-y-2">
      <div class="flex items-center justify-between">
        <Label class="text-sm font-medium">Teams</Label>
        <Button variant="ghost" size="sm" @click="toggleAllTeams">
          {{ allTeamsSelected ? 'Deselect All' : 'Select All' }}
        </Button>
      </div>

      <ScrollArea class="h-[400px] w-full rounded-md border p-2">
        <div class="space-y-2">
          <div v-for="team in availableTeams" :key="team.id" class="flex items-center space-x-2">
            <Checkbox
              :id="team.id"
              :checked="isTeamSelected(team)"
              @update:checked="toggleTeam(team)"
            />
            <Label :for="team.id" class="text-sm">{{ team.name }}</Label>
          </div>
        </div>
      </ScrollArea>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select'
import { Label } from '@/components/ui/label'
import { Checkbox } from '@/components/ui/checkbox'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Button } from '@/components/ui/button'

const props = defineProps<{
  departments: string[]
  selectedDept: string
  selectedTeams: { id: string; name: string }[]
  availableTeams: { id: string; name: string }[]
}>()

const emit = defineEmits(['update:selectedDept', 'update:selectedTeams'])

const allTeamsSelected = computed(() => props.selectedTeams.length === props.availableTeams.length)

const isTeamSelected = (team: { id: string; name: string }) =>
  props.selectedTeams.some(t => t.id === team.id)

const toggleAllTeams = () => {
  if (allTeamsSelected.value) {
    emit('update:selectedTeams', [])
  } else {
    emit('update:selectedTeams', [...props.availableTeams])
  }
}

const toggleTeam = (team: { id: string; name: string }) => {
  const currentSelected = [...props.selectedTeams]
  const index = currentSelected.findIndex(t => t.id === team.id)

  if (index === -1) {
    currentSelected.push(team)
  } else {
    currentSelected.splice(index, 1)
  }

  emit('update:selectedTeams', currentSelected)
}

const handleDepartmentChange = (dept: string) => {
  emit('update:selectedDept', dept)
}
</script>
