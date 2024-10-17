<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12">
        <v-card class="full-chart-card" elevation="4">
          <v-card-title v-if="title" class="text-center">{{ title }}</v-card-title>
          <v-card-item>
            <div class="full-chart-container">
              <component :is="chartComponent" :data="data" :options="options" />
            </div>
          </v-card-item>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { Line, Bar, Pie, Doughnut } from "vue-chartjs";
import { ChartOptions} from "chart.js";

export default defineComponent({
  name: "FullWidthChart",
  components: {
    Line,
    Bar,
    Pie,
    Doughnut,
  },
  props: {
    title: {
      type: String,
      required: false,
    },
    data: {
      type: Object as () => { datasets: any[] },
      required: true,
    },
    options: {
      type: Object as () => ChartOptions,
      required: true,
    },
    chartType: {
      type: String,
      default: "line",
      validator: (value: string) => ["line", "bar", "pie", "doughnut"].includes(value),
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
.full-chart-card {
  padding: 30px;
  border-radius: 12px;
  background-color: #ffffff;
  transition: transform 0.2s ease-in-out, box-shadow 0.2s ease-in-out;
  width: 100%;
}

.full-chart-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
}

.full-chart-container {
  padding: 30px;
}

.v-card-title {
  font-size: 1.4em;
  font-weight: bold;
  margin-bottom: 20px;
}
</style>