<template>
    <v-col :cols="cols" :sm="sm" :md="md">
      <v-card elevation="6" class="metric-card">
        <v-card-item class="content-wrapper">
          <div class="icon-container">
            <v-icon v-if="icon" large>{{ icon }}</v-icon>
          </div>
          <div class="text-container">
            <div class="metric-title">{{ title }}</div>
            <div class="metric-subtitle">{{ subtitle }}</div>
            <p class="metric-value">{{ value }}</p>
          </div>
        </v-card-item>
      </v-card>
    </v-col>
  </template>
  
  <script lang="ts">
  import { defineComponent } from "vue";
  
  export default defineComponent({
    name: "MetricCard",
    props: {
      title: {
        type: String,
        required: true,
      },
      subtitle: {
        type: String,
        required: true,
      },
      value: {
        type: [String, Number],
        required: true,
      },
      cols: {
        type: Number,
        default: 12,
      },
      sm: {
        type: Number,
        default: 6,
      },
      md: {
        type: Number,
        default: 3,
      },
      icon: {
        type: String,
        default: "", // Optionally pass in an icon name from Material Design icons
      },
    },
  });
  </script>
  
  <style scoped>
  .metric-card {
    padding: 20px;
    text-align: center;
    border-radius: 12px;
    background: linear-gradient(145deg, #f5f7fa, #e6ebf0); /* Soft gradient background */
    box-shadow: 0px 4px 12px rgba(0, 0, 0, 0.1); /* Subtle shadow for depth */
    transition: all 0.3s ease-in-out; /* Smooth transitions */
  }
  
  .metric-card:hover {
    transform: translateY(-4px); /* Lift on hover */
    box-shadow: 0px 8px 24px rgba(0, 0, 0, 0.15); /* Stronger shadow on hover */
  }
  
  .content-wrapper {
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .icon-container {
    margin-right: 10px;
    color: #42a5f5; /* Add color to the icon for more vibrancy */
  }
  
  .text-container {
    flex-grow: 1;
  }
  
  .metric-title {
    font-size: 18px;
    font-weight: bold;
    color: #2c3e50;
  }
  
  .metric-subtitle {
    font-size: 14px;
    color: #7f8c8d;
  }
  
  .metric-value {
    font-size: 32px;
    font-weight: bold;
    color: #1abc9c;
    margin-top: 10px;
  }
  </style>