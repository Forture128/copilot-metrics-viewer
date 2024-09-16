<template>
  <!-- Main Layout -->
  <v-app>
    <!-- Sidebar -->
    <v-navigation-drawer app permanent color="indigo">
      <v-list>
        <v-list-item>
          <v-list-item-content>
            <v-list-item-title class="text-h6">
              Copilot Metrics Viewer
            </v-list-item-title>
            <v-list-item-subtitle>
              {{ capitalizedItemName }} : {{ displayedViewName }} {{ teamName }}
            </v-list-item-subtitle>
          </v-list-item-content>
        </v-list-item>

        <v-divider></v-divider>

        <!-- Sidebar Tab Items -->
        <v-list-item v-for="item in tabItems" :key="item" :value="item" :class="{ 'active-item': tab === item }"
          @click="tab = item">
          <template v-slot:prepend>
            <v-icon>{{ getIconForItem(item) }}</v-icon>
          </template>
          <v-list-item-title class="uppercase-text">{{
            item
            }}</v-list-item-title>
        </v-list-item>
        <!-- <v-list-item v-if="tabItems.includes('team metrics') && selectedTeam" :key="'team metrics'"
          :value="'team metrics'" :class="{ 'active-item': tab === 'team metrics' }" @click="tab = 'team metrics'">
          <template v-slot:prepend>
            <v-icon>{{ getIconForItem('team metrics') }}</v-icon>
          </template>
          <v-list-item-title class="uppercase-text">Team Metrics</v-list-item-title>
        </v-list-item> -->
      </v-list>
    </v-navigation-drawer>

    <!-- Main Content Area -->
    <v-main>
      <!-- Main Content with Metrics -->
      <v-container>
        <!-- API Error Message -->
        <div v-if="apiError" class="error-message" v-html="apiError"></div>

        <div v-if="!apiError">
          <v-progress-linear v-if="!metricsReady" indeterminate color="indigo"></v-progress-linear>

          <!-- Content Navigation Logic -->
          <v-window v-if="metricsReady" v-model="tab">
            <v-window-item v-for="item in tabItems" :key="item" :value="item">
              <v-card flat>
                <MetricsViewer v-if="item === itemName" :metrics="metrics" />
                <BreakdownComponent v-if="item === 'languages'" :metrics="metrics" :breakdownKey="'language'" />
                <BreakdownComponent v-if="item === 'editors'" :metrics="metrics" :breakdownKey="'editor'" />
                <CopilotChatViewer v-if="item === 'copilot chat'" :metrics="metrics" />
                <SeatsAnalysisViewer v-if="item === 'seat analysis'" :seats="seats" />
                <ApiResponse v-if="item === 'api response'" :metrics="metrics" :seats="seats" />
                <TeamMetricsViewer v-if="item === 'team metrics' && teamMetricsReady" :teams="teamList"
                  :team="teamList[0]" :metrics="teamMetrics" :breakdownKey="'team metrics'" />
              </v-card>
            </v-window-item>
          </v-window>
        </div>
      </v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent, onMounted, ref } from "vue";
import { getMetricsApi, getTeams } from "../api/GitHubApi";
import { getTeamMetricsApi } from "../api/GitHubApi";
import { getSeatsApi } from "../api/ExtractSeats";
import { Metrics, TeamMetrics } from "../model/Metrics";
import { Seat } from "../model/Seat";

//Components
import MetricsViewer from "./MetricsViewer.vue";
import BreakdownComponent from "./BreakdownComponent.vue";
import CopilotChatViewer from "./CopilotChatViewer.vue";
import SeatsAnalysisViewer from "./SeatsAnalysisViewer.vue";
import ApiResponse from "./ApiResponse.vue";
import config from "../config";
import TeamMetricsViewer from "./TeamMetricsViewer.vue";

export default defineComponent({
  name: "MainComponent",
  components: {
    MetricsViewer,
    BreakdownComponent,
    CopilotChatViewer,
    SeatsAnalysisViewer,
    ApiResponse,
    TeamMetricsViewer
  },
  computed: {
    gitHubOrgName() {
      return config.github.org;
    },
    itemName() {
      return config.scope.type;
    },
    capitalizedItemName(): string {
      return this.itemName.charAt(0).toUpperCase() + this.itemName.slice(1);
    },
    displayedViewName(): string {
      return config.scope.name;
    },
    isScopeOrganization() {
      return config.scope.type === "organization";
    },
    teamName() {
      var teamName;
      if (config.github.team && config.github.team.trim() !== "") {
        teamName = "| Team : " + config.github.team;
      } else {
        teamName = "";
      }
      return teamName;
    },
    mockedDataMessage() {
      return config.mockedData
        ? "Using mock data - see README if unintended"
        : "";
    },
  },
  data() {
    return {
      tabItems: ["languages", "team metrics", "editors", "copilot chat", "api response",],
      tab: "",
    };
  },
  methods: {
    getIconForItem(item: string): string {
      // Add icons for each menu item
      const icons: { [key: string]: string } = {
        languages: "mdi-code-tags",
        editors: "mdi-pencil",
        "copilot chat": "mdi-chat",
        "seat analysis": "mdi-account-group",
        "api response": "mdi-api",
        "team metrics": "mdi-account-group",
        organization: "mdi-domain",
        enterprise: "mdi-office-building",
      };
      return icons[item] || "mdi-chevron-right";
    },
  },
  created() {
    this.tabItems.unshift(this.itemName);
    if (config.scope.type === "organization") {
      // get the last item in the array,which is 'api response'
      //and add 'seat analysis' before it
      let lastItem = this.tabItems.pop();
      this.tabItems.push("seat analysis");
      if (lastItem) {
        this.tabItems.push(lastItem);
      }
    }
  },
  setup() {
    const metricsReady = ref(false);
    const metrics = ref<Metrics[]>([]);

    const seatsReady = ref(false);
    const seats = ref<Seat[]>([]);

    const teamList = ref<string[]>([]);
    const teamMetrics = ref<TeamMetrics[]>([]);
    const teamMetricsReady = ref(false);
    // API Error Message
    const apiError = ref<string | undefined>(undefined);

    // if (config.github.team && config.github.team.trim() !== "") {
    //   getTeamMetricsApi()
    //     .then((data) => {
    //       metrics.value = data;

    //       // Set metricsReady to true after the call completes.
    //       metricsReady.value = true;
    //     })
    //     .catch((error) => handleApiError(error, apiError));
    // }

    if (metricsReady.value === false) {
      getMetricsApi()
        .then((data) => {
          metrics.value = data;

          // Set metricsReady to true after the call completes.
          metricsReady.value = true;
        })
        .catch((error) => handleApiError(error, apiError));
    }

    getSeatsApi()
      .then((data) => {
        seats.value = data;

        // Set seatsReady to true after the call completes.
        seatsReady.value = true;
      })
      .catch((error) => handleApiError(error, apiError));

    // Fetch teams for the dropdown
    // Add console.log to see the team list
    getTeams()
      .then((data) => {
        console.log("Team List: ", data);
        // convert Array[Team] to array name of teams
        teamList.value = data.map((team) => team.slug);
        // teamTagList.value = data.map((team) => team.slug);
      })
      .catch((error) => handleApiError(error, apiError));

    // Fetch data for all teams once on mount
    onMounted(async () => {
      try {
        const teams = await getTeams();
        teamList.value = teams.map((team) => team.slug);
        teamList.value.unshift("All Teams");

        // Add default "All Teams" entry with overall metrics
        const allTeamsData: TeamMetrics = {
          team_tag: "All Teams",
          metrics: metrics.value,
        };
        teamMetrics.value.push(allTeamsData);

        // Fetch team metrics for all teams
        // Loop through the teamList and fetch metrics for each team
        teamList.value.forEach(async (team) => {
          if (team !== "All Teams") {
            const data = await getTeamMetricsApi(team);
            const teamData: TeamMetrics = {
              team_tag: team,
              metrics: data,
            };
            teamMetrics.value.push(teamData);
          }
        });
        teamMetricsReady.value = true;
      } catch (error) {
        handleApiError(error, apiError);
      }
    });
    // Reusable Error Handling
    const handleApiError = (error: any, errorRef: any) => {
      console.error(error);
      if (error.response && error.response.status) {
        switch (error.response.status) {
          case 401:
            errorRef.value =
              "401 Unauthorized access - check if your token in the .env file is correct.";
            break;
          case 404:
            errorRef.value = `404 Not Found - is the ${config.scope.type} '${config.scope.name}' correct?`;
            break;
          default:
            errorRef.value = error.message;
            break;
        }
      } else {
        errorRef.value = error.message;
      }
      errorRef.value +=
        " <br> If .env file is modified, restart the app for the changes to take effect.";
    };
    return { metricsReady, metrics, seatsReady, seats, teamList, teamMetrics, teamMetricsReady, apiError };
  },
});
</script>

<style scoped>
.toolbar-title {
  white-space: nowrap;
  overflow: visible;
  text-overflow: clip;
}

.error-message {
  color: red;
}
</style>
