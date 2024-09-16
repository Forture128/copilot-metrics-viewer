<template>
  <div>
    <!-- Dashboard Container for Metrics -->
    <v-container fluid class="dashboard-container">
      <v-row justify="center" align="center">
        <!-- Total Assigned Card -->
        <v-col cols="12" sm="6" md="4">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">Total Assigned</div>
                <div class="metric-subtitle">Currently assigned seats</div>
                <p class="metric-value">{{ totalSeats.length }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>

        <!-- Assigned But Never Used Card -->
        <v-col cols="12" sm="6" md="4">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">Assigned But Never Used</div>
                <div class="metric-subtitle">No show seats</div>
                <p class="metric-value">{{ NoshowSeats.length }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>

        <!-- No Activity in the Last 7 days Card -->
        <v-col cols="12" sm="6" md="4">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">No Activity in the Last 7 days</div>
                <div class="metric-subtitle">No use in the last 7 days</div>
                <p class="metric-value">{{ unusedSeats.length }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>
      </v-row>
    </v-container>

    <!-- Seats Table Section -->
    <v-container fluid class="seats-table-container">
      <v-row>
        <v-col cols="12">
          <h2 class="text-center">All assigned seats</h2>
          <v-card class="table-card">
            <v-data-table
              :headers="headers"
              :items="totalSeats"
              :items-per-page="10"
              class="elevation-2"
            >
              <template v-slot:item="{ item }">
                <tr>
                  <td>{{ item.login }}</td>
                  <td>{{ item.id }}</td>
                  <td>{{ item.team }}</td>
                  <td>{{ item.created_at }}</td>
                  <td>{{ item.last_activity_at }}</td>
                  <td>{{ item.last_activity_editor }}</td>
                </tr>
              </template>
            </v-data-table>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </div>
</template>
  
<script lang="ts">
import { defineComponent, ref, watchEffect } from 'vue';
import { Seat } from '../model/Seat';

export default defineComponent({
  name: 'SeatsAnalysisViewer',
  props: {
    seats: {
      type: Array as () => Seat[],
      required: true,
      default: () => []  
    }
  },
  setup(props) {
    const totalSeats = ref<Seat[]>([]);
    const NoshowSeats = ref<Seat[]>([]);
    const unusedSeats = ref<Seat[]>([]);

    const headers = [
      { title: 'Login', key: 'login' },
      { title: 'GitHub ID', key: 'id' },
      { title: 'Assigning team', key: 'team' },
      { title: 'Assigned time', key: 'created_at' },
      { title: 'Last Activity At', key: 'last_activity_at' },
      { title: 'Last Activity Editor', key: 'last_activity_editor' },
    ];

    watchEffect(() => {
      if (props.seats && Array.isArray(props.seats)) {
        totalSeats.value = props.seats;

        const oneWeekAgo = new Date();
        oneWeekAgo.setDate(oneWeekAgo.getDate() - 7);

        NoshowSeats.value = props.seats.filter(seat => seat.last_activity_at == null);

        unusedSeats.value = totalSeats.value.filter(seat => {
          if (seat.last_activity_at === null) {
            return true;
          }

          const lastActivityDate = new Date(seat.last_activity_at);
          return lastActivityDate < oneWeekAgo;
        });

        unusedSeats.value.sort((a, b) => {
          if (a.last_activity_at === null) {
            return -1;
          }
          if (b.last_activity_at === null) {
            return 1;
          }
          return new Date(a.last_activity_at) > new Date(b.last_activity_at) ? 1 : -1;
        });
      } else {
        throw new Error('Invalid number of seats');
      }
    });

    return {
      totalSeats,
      NoshowSeats,
      unusedSeats,
      headers
    };
  }
});
</script>
