<template>
  <Card class="w-full transition-all hover:-translate-y-1 hover:shadow-lg">
    <CardContent>
      <div class="p-6">
        <component :is="chartComponent" :data="data" :options="chartOptions" />
      </div>
    </CardContent>
  </Card>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Line, Bar, Pie, Doughnut } from 'vue-chartjs'
import { type ChartOptions } from 'chart.js'
import { Card, CardContent } from '@/components/ui/card'

interface Props {
  // eslint-disable-next-line vue/require-default-prop
  title?: string
  data: {
    datasets: any[]
  }
  options: ChartOptions
  chartType?: 'line' | 'bar' | 'pie' | 'doughnut'
}

const props = withDefaults(defineProps<Props>(), {
  chartType: 'line'
})

const chartComponent = computed(() => {
  switch (props.chartType) {
    case 'bar':
      return Bar
    case 'pie':
      return Pie
    case 'doughnut':
      return Doughnut
    default:
      return Line
  }
})

const chartOptions = computed(() => ({
  ...props.options,
  responsive: true,
  maintainAspectRatio: true,
  plugins: {
    legend: {
      display: true,
      position: 'top' as const
    },
    title: {
      display: !!props.title,
      text: props.title,
      font: {
        size: 16,
        weight: 700
      }
    }
  }
}))
</script>
