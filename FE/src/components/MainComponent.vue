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
                <DepartmentsMetricsViewer v-if="item === 'departments'" :teamMetrics="teamMetrics" />
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
import { defineComponent, onMounted } from "vue";
import { useStore } from "vuex";
import { mapState, mapActions } from "vuex";
import config from "../config";

//Components
import MetricsViewer from "./MetricsViewer.vue";
import BreakdownComponent from "./BreakdownComponent.vue";
import CopilotChatViewer from "./CopilotChatViewer.vue";
import SeatsAnalysisViewer from "./SeatsAnalysisViewer.vue";
import ApiResponse from "./ApiResponse.vue";
import TeamMetricsViewer from "./TeamMetricsViewer.vue";
import DoraDashboard from "./DoraDashboard.vue";
import DepartmentsMetricsViewer from "./DepartmentsMetricsViewer.vue";

export default defineComponent({
  name: "MainComponent",
  components: {
    MetricsViewer,
    DepartmentsMetricsViewer,
    BreakdownComponent,
    CopilotChatViewer,
    SeatsAnalysisViewer,
    ApiResponse,
    TeamMetricsViewer,
    DoraDashboard,
  },
  computed: {
    ...mapState({
      metricsReady: (state: any) => state.CopilotUsage.metricsReady,
      metrics: (state: any) => state.CopilotUsage.metrics,
      seatsReady: (state: any) => state.CopilotUsage.seatsReady,
      seats: (state: any) => state.CopilotUsage.seats,
      teamList: (state: any) => state.CopilotUsage.teamList,
      teamMetrics: (state: any) => state.CopilotUsage.teamMetrics,
      teamMetricsReady: (state: any) => state.CopilotUsage.teamMetricsReady,
      apiError: (state: any) => state.CopilotUsage.apiError,
    }),
    // ...mapGetters("CopilotUsage", ["gitHubOrgName", "itemName", "capitalizedItemName", "displayedViewName", "isScopeOrganization"]),
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
    ...mapActions("CopilotUsage", ["fetchMetrics", "fetchSeats", "fetchAllTeamMetrics"]),
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
    const store = useStore();

    // Fetch data for all teams once on mount
    onMounted(async () => {
      try {
        await store.dispatch("CopilotUsage/fetchMetrics");
        await store.dispatch("CopilotUsage/fetchSeats");
        await store.dispatch("CopilotUsage/fetchAllTeamMetrics");
      } catch (error) {
        console.error(error);
      }
    });

    return {
      store,
    };
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