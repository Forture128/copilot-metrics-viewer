<template>
  <!-- Main Layout -->
  <v-app>
    <v-toolbar color="indigo" elevation="4">
      <v-btn icon>
        <v-icon>mdi-github</v-icon>
      </v-btn>

      <v-toolbar-title class="toolbar-title">Copilot Metrics Viewer | {{ capitalizedItemName }} : {{ displayedViewName
        }} {{ teamName }}

      </v-toolbar-title>
      <h2 class="error-message"> {{ mockedDataMessage }} </h2>
      <v-spacer></v-spacer>

      <template v-slot:extension>

        <v-tabs v-model="tab" align-tabs="title">
          <v-tab v-for="item in tabItems" :key="item" :value="item">
            {{ item }}
          </v-tab>
        </v-tabs>

      </template>

    </v-toolbar>

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
                <MetricsViewer v-if="item === itemName" :metrics="metrics" :teamMetrics="teamMetrics" />
                <BreakdownComponent v-if="item === 'languages'" :metrics="metrics" :breakdownKey="'language'" />
                <BreakdownComponent v-if="item === 'editors'" :metrics="metrics" :breakdownKey="'editor'" />
                <CopilotChatViewer v-if="item === 'copilot chat'" :metrics="metrics" />
                <SeatsAnalysisViewer v-if="item === 'seat analysis'" :seats="seats" />
                <ApiResponse v-if="item === 'api response'" :metrics="metrics" :seats="seats" />
                <TeamMetricsViewer v-if="item === 'team metrics' && teamMetricsReady" :teams="teamList"
                  :team="teamList[0]" :metrics="teamMetrics" :breakdownKey="'team metrics'" />
                <DoraDashboard v-if="item === 'dora dashboard'" />
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
import { getMetricsApi, getTeamMembers, getTeams } from "../api/GitHubApi";
import { getTeamMetricsApi } from "../api/GitHubApi";
import { getSeatsApi } from "../api/ExtractSeats";
import { MembersMetrics, Metrics, TeamMetrics } from "../model/Metrics";
import { Seat } from "../model/Seat";

//Components
import MetricsViewer from "./MetricsViewer.vue";
import BreakdownComponent from "./BreakdownComponent.vue";
import CopilotChatViewer from "./CopilotChatViewer.vue";
import SeatsAnalysisViewer from "./SeatsAnalysisViewer.vue";
import ApiResponse from "./ApiResponse.vue";
import config from "../config";
import TeamMetricsViewer from "./TeamMetricsViewer.vue";
import DoraDashboard from "./DoraDashboard.vue";

export default defineComponent({
  name: "MainComponent",
  components: {
    MetricsViewer,
    BreakdownComponent,
    CopilotChatViewer,
    SeatsAnalysisViewer,
    ApiResponse,
    TeamMetricsViewer,
    DoraDashboard,
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
      tabItems: ["team metrics", "languages", "editors", "copilot chat", "api response", "seat analysis", "dora dashboard"],
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
        "dora dashboard": "mdi mdi-monitor-dashboard",
      };
      return icons[item] || "mdi-chevron-right";
    },
    navigateTo(item: string) {
      this.tab = item;
      if (item === 'dora dashboard') {
        this.$router.push({ name: 'DoraDashboard' });
      } else {
        this.$router.push({ name: 'Home' });
      }
    },
  },
  created() {
    this.tabItems.unshift(this.itemName);
    if (config.scope.type === "organization") {
      // get the last item in the array,which is 'api response'
      //and add 'seat analysis' before it
      let lastItem = this.tabItems.pop();
      // this.tabItems.push("seat analysis");
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
    teamList.value = ["cloud_transfer_reviewers", "neo-ap", "rci-mfv", "attendance_dev_reviewers", "mf_connected_db_developers", "payroll_dev_reviewers", "payroll_kotlin_reviewers", "social_insurance_dev_reviewers", "tax_adjustment_dev_mfj_reviewers"];
    const membersMetrics = ref<MembersMetrics[]>([]);
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
    // getTeams()
    //   .then((data) => {
    //     console.log("Team List: ", data);
    //     // convert Array[Team] to array name of teams
    //     teamList.value = data.map((team) => team.slug);
    //     // teamTagList.value = data.map((team) => team.slug);
    //     console.log("Team List: ", teamList.value);
    //   })
    //   .catch((error) => handleApiError(error, apiError));

    // Fetch data for all teams once on mount
    onMounted(async () => {
      try {
        // const teams = await getTeams();
        // cloud_transfer_reviewers | neo-ap | rci-mfv
        // Create a list of team names for the dropdown
        // team
        // teamList.value = teams.map((team) => team.slug);
        // teamList.value = ["All Teams", "cloud_transfer_reviewers", "neo-ap", "rci-mfv"];
        // teamList.value.unshift("All Teams");

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
            const metrics = await getTeamMetricsApi(team);
            const members = await getTeamMembers(team);
            console.log("Team members: ", members);
            const teamData: MembersMetrics = {
              team_tag: team,
              metrics: metrics,
              members: members,
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
