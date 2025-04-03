import { createStore } from 'vuex'
import DoraData from './DoraData'
import authModule from './auth.module'
import type { AuthState } from './auth.module'
import CopilotUsageModule from './CopilotUsage.module'
import type { CopilotUsageState } from './CopilotUsage.module'
import DeveloperMetricsModule from './DeveloperMetrics.module'
import type { DeveloperMetricsState } from './DeveloperMetrics.module'

// Root state interface
export interface RootState {
  auth?: AuthState
  CopilotUsage?: CopilotUsageState
  DeveloperMetrics?: DeveloperMetricsState
  // Add other module states as needed
}

export default createStore<RootState>({
  state: {
    // Root state properties go here
  },

  getters: {
    // Root getters go here
  },

  mutations: {
    // Root mutations go here
  },

  actions: {
    // Root actions go here
  },

  modules: {
    DoraData,
    auth: authModule,
    CopilotUsage: CopilotUsageModule,
    DeveloperMetrics: DeveloperMetricsModule
  }
})
