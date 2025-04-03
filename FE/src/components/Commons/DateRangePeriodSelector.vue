<template>
  <div class="flex items-center space-x-4">
    <CustomizeSelect
      v-model="selectedPeriodValue"
      class="w-32"
      @update:model-value="updatePeriodData"
    >
      <SelectTrigger>
        <SelectValue :placeholder="selectedPeriodValue" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="weekly">Weekly</SelectItem>
        <SelectItem value="monthly">Monthly</SelectItem>
        <SelectItem value="yearly">Yearly</SelectItem>
      </SelectContent>
    </CustomizeSelect>

    <Popover>
      <PopoverTrigger as-child>
        <CustomButton
          id="date"
          variant="outline"
          :class="
            cn(
              'w-[280px] justify-start text-left font-normal',
              !dateRange && 'text-muted-foreground'
            )
          "
        >
          <CalendarIcon class="mr-2 h-4 w-4" />
          <span>
            {{
              dateRange.start
                ? dateRange.end
                  ? `${format(dateRange.start, 'LLL dd, y')} - ${format(
                      dateRange.end,
                      'LLL dd, y'
                    )}`
                  : format(dateRange.start, 'LLL dd, y')
                : 'Pick a date'
            }}
          </span>
        </CustomButton>
      </PopoverTrigger>
      <PopoverContent class="w-auto p-0" align="start">
        <Calendar v-model.range="dateRange" :columns="2" @update:model-value="updateDateRange" />
      </PopoverContent>
    </Popover>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import ToastService from '@/services/ToastService'
import {
  Select as CustomizeSelect,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select'
import { Button as CustomButton } from '@/components/ui/button'
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover'
import { Calendar } from '@/components/ui/v-calendar'
import { cn } from '@/lib/utils'
import { subMonths, addDays, format } from 'date-fns'
import { CalendarIcon } from 'lucide-vue-next'

const props = defineProps<{
  modelValue: {
    start: Date
    end: Date
  }
  selectedPeriod: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: { start: Date; end: Date }]
  'update:selectedPeriod': [value: string]
  dateRangeChanged: []
}>()

const dateRange = ref(props.modelValue)
const selectedPeriodValue = ref(props.selectedPeriod)

const updateDateRange = () => {
  if (!dateRange.value.start || !dateRange.value.end) return

  if (dateRange.value.start > dateRange.value.end) {
    ToastService.error('Start date cannot be after end date')
    dateRange.value = {
      start: subMonths(new Date(), 1),
      end: new Date()
    }
    return
  }

  emit('update:modelValue', dateRange.value)
  emit('dateRangeChanged')
}

const updatePeriodData = (period: string) => {
  selectedPeriodValue.value = period
  emit('update:selectedPeriod', period)

  const now = new Date()
  switch (period) {
    case 'weekly':
      dateRange.value = {
        start: addDays(now, -7),
        end: now
      }
      break
    case 'monthly':
      dateRange.value = {
        start: subMonths(now, 1),
        end: now
      }
      break
    case 'yearly':
      dateRange.value = {
        start: new Date(now.getFullYear() - 1, now.getMonth(), now.getDate()),
        end: now
      }
      break
  }

  emit('update:modelValue', dateRange.value)
  emit('dateRangeChanged')
}

watch(
  () => props.modelValue,
  newVal => {
    dateRange.value = newVal
  }
)

watch(
  () => props.selectedPeriod,
  newVal => {
    selectedPeriodValue.value = newVal
  }
)
</script>
