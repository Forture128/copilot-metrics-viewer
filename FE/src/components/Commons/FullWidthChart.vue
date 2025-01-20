<template>
  <Card class="w-full transition-all hover:-translate-y-1 hover:shadow-lg">
    <CardContent>
      <component :is="chartComponent" :data="data" :options="chartOptions" />
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
    tooltip: {
      enabled: true,
      mode: 'index',
      intersect: false,
      callbacks: {
        label: function (context: any) {
          const label = context.dataset.label || ''
          const value = context.parsed.y

          if (label) {
            if (label === 'Acceptance Rate' || label === 'Lines Acceptance Rate') {
              return `${label}: ${value.toFixed(3)}%`
            }

            return `${label}: ${value.toLocaleString()}`
          }

          return ''
        }
      }
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
