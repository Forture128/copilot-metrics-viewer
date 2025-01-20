<template>
  <Card
    :class="[
      'col-span-12 sm:col-span-6 md:col-span-3',
      'group transition-all hover:-translate-y-1 hover:shadow-lg',
      'rounded-lg shadow-sm gradient-background'
    ]"
  >
    <CardHeader>
      <div class="flex items-center justify-between space-x-4">
        <CardTitle class="text-base font-semibold">
          {{ title }}
        </CardTitle>
        <div v-if="icon" class="rounded-full p-2 bg-primary/10">
          <component :is="getIconComponent(icon)" class="h-5 w-5 text-primary" />
        </div>
      </div>
      <CardDescription>{{ subtitle }}</CardDescription>
    </CardHeader>

    <CardContent>
      <div class="space-y-1">
        <div class="flex items-center justify-between">
          <span class="text-3xl font-bold tracking-tight text-emerald-500">{{
            formattedValue
          }}</span>
          <div
            v-if="trend"
            :class="[
              'flex items-center gap-1 text-sm',
              trend > 0 ? 'text-green-500' : 'text-red-500'
            ]"
          >
            <component :is="trend > 0 ? TrendingUp : TrendingDown" class="h-4 w-4" />
            {{ Math.abs(trend) }}%
          </div>
        </div>
        <p v-if="description" class="text-sm text-muted-foreground">
          {{ description }}
        </p>
      </div>
    </CardContent>
  </Card>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui/card'
import {
  ChartLine,
  Lightbulb,
  CheckCircle2,
  Code2,
  MessageSquare,
  Users,
  TrendingUp,
  TrendingDown,
  Clock,
  type LucideIcon
} from 'lucide-vue-next'

interface Props {
  title: string
  subtitle: string
  value: string | number
  icon?: string
  trend?: number // Optional trend percentage
  description?: string // Optional description text
}

const props = defineProps<Props>()

const getIconComponent = (iconName: string): LucideIcon => {
  const icons: Record<string, LucideIcon> = {
    'mdi-chart-areaspline': ChartLine,
    'mdi-lightbulb-outline': Lightbulb,
    'mdi-checkbox-marked-circle-outline': CheckCircle2,
    'mdi-code-tags': Code2,
    'mdi-message-text': MessageSquare,
    'mdi-account-group': Users,
    'mdi-account-multiple': Users,
    'mdi-account-cancel': Users,
    'mdi-account-clock': Users,
    'mdi-clock-start': Clock
  }
  return icons[iconName] || ChartLine
}

const formattedValue = computed(() => {
  if (typeof props.value === 'number') {
    return new Intl.NumberFormat().format(props.value)
  }
  return props.value
})
</script>

<style scoped>
.gradient-background {
  background: linear-gradient(145deg, #f5f7fa, #e6ebf0);
}
</style>
