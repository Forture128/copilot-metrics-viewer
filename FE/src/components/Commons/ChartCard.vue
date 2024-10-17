<template>
  <v-col :cols="cols" :sm="sm" :md="md">
    <v-card class="chart-card" elevation="4">
      <v-card-title class="text-center">{{ title }}</v-card-title>
      <v-card-item>
        <div class="chart-container">
          <component :is="chartComponent" :data="data" :options="options" />
        </div>
      </v-card-item>
    </v-card>
  </v-col>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { Line, Bar, Pie, Doughnut } from "vue-chartjs";

export default defineComponent({
  name: "ChartCard",
  components: {
    Line,
    Bar,
    Pie,
    Doughnut,
  },
  props: {
    title: {
      type: String,
      required: true,
    },
    data: {
      type: Object as () => { datasets: any[] },
      required: true,
    },
    options: {
      type: Object,
      required: true,
    },
    chartType: {
      type: String,
      default: "line",
      validator: (value: string) => ["line", "bar", "pie", "doughnut"].includes(value),
    },
    cols: {
      type: Number,
      default: 12,
    },
    sm: {
      type: Number,
      default: 12,
    },
    md: {
      type: Number,
      default: 12,
    },
  },
  computed: {
    chartComponent() {
      switch (this.chartType) {
        case "bar":
          return "Bar";
        case "pie":
          return "Pie";
        case "doughnut":
          return "Doughnut";
        default:
          return "Line";
      }
    },
  },
});
</script>

<style scoped>
.chart-card {
  padding: 15px;
  border-radius: 12px;
  background-color: #ffffff;
  transition: transform 0.2s ease-in-out, box-shadow 0.2s ease-in-out;
  margin-bottom: 25px; /* Increase margin between cards */
}

.chart-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
}

.chart-container {
  height: 350px; /* Increased height for larger charts */
  width: 100%; /* Ensure the chart takes full width */
  padding: 20px; /* Keep minimal padding */
  display: flex;
  justify-content: center;
  align-items: center;
}

.v-card-title {
  font-size: 1.3em;
  font-weight: bold;
  margin-bottom: 10px;
}
</style>
