# Copilot Metrics Viewer Frontend

A Vue.js application for visualizing GitHub Copilot metrics and managing RBAC (Role-Based Access Control).

## Project Overview

This frontend application provides a user interface for:

- Visualizing GitHub Copilot usage metrics across teams and organizations
- Analyzing developer productivity and code quality metrics
- Tracking DORA (DevOps Research and Assessment) metrics
- Managing users, roles, and departments (RBAC)

## Project Structure

```
src/
├── assets/                # Static assets (images, fonts, etc.)
├── components/            # Reusable UI components
│   ├── common/            # Generic UI components (buttons, inputs, etc.)
│   ├── dashboard/         # Dashboard-specific components
│   │   └── dora/          # DORA metrics components
│   │       ├── DoraDashboard.vue
│   │       ├── CycleTimeCard.vue
│   │       └── ...
│   └── rbac/              # RBAC management components
├── layouts/               # Layout components
│   ├── AppLayout.vue      # Main application layout
│   └── AuthLayout.vue     # Layout for authentication pages
├── router/                # Vue Router configuration
├── services/              # API and other services
│   └── HorusService.ts    # Service for Horus API communication
├── store/                 # Vuex store modules
│   ├── auth.module.ts     # Authentication store module
│   ├── CopilotUsage.ts    # Copilot usage store module
│   ├── DoraData.ts        # DORA metrics store module
│   └── index.ts           # Root store
├── types/                 # TypeScript type definitions
│   └── horus-api.types.ts # Type definitions for API responses
└── views/                 # Page/screen components
    ├── admin/             # Admin-specific views
    ├── analysis/          # Data analysis views
    ├── auth/              # Authentication-related views
    │   └── LoginView.vue
    ├── dashboard/         # Dashboard-related views
    │   └── DashboardView.vue
    ├── departments/       # Department management views
    ├── metrics/           # Metrics-related views
    │   ├── DoraMetricsView.vue
    │   └── DeveloperMetricsView.vue
    ├── organization/      # Organization management views
    ├── rbac/              # RBAC management views
    └── teams/             # Team management views
```

## Implementation Status

### Completed Features

- Authentication flow with login/logout functionality
- Dashboard view with navigation to different metric pages
- DORA metrics visualization components
- Application layout and navigation structure
- Type-safe API service for backend communication

### In Progress

- Developer metrics visualization
- Organization and team views
- RBAC management interface
- Data analysis views

## Getting Started

### Prerequisites

- Node.js 14.x or higher
- npm 7.x or higher

### Installation

```bash
# Install dependencies
npm install

# Serve with hot reload at localhost:8080
npm run serve

# Build for production
npm run build
```

### Configuration

The application can be configured via environment variables:

- `VUE_APP_API_URL`: URL of the backend API
- `VUE_APP_USE_MOCK_DATA`: Set to 'true' to use mock data instead of calling the API

## Contributing

1. Ensure you follow the established project structure
2. Create feature branches from `develop`
3. Use TypeScript for type safety
4. Follow the Vue.js style guide
5. Write unit tests for new components and services

This application is designed to provide clear visualizations of various metrics related to GitHub Copilot for your GitHub Organization or Enterprise Account. It utilizes the [GitHub Copilot Metrics API](https://docs.github.com/en/enterprise-cloud@latest/rest/copilot/copilot-usage?apiVersion=2022-11-28) to gather the necessary data.

## Key Metrics

The GitHub Copilot Metrics Viewer provides the following key metrics visualized in charts:

1. **Acceptance Rate:** This metric represents the ratio of accepted lines to the total lines suggested by GitHub Copilot. It indicates the relevance and usefulness of Copilot's suggestions.

2. **Total Suggestions:** This chart illustrates the total number of code suggestions made by GitHub Copilot over time, providing insights into the tool's activity and engagement with users.

3. **Total Acceptances:** This visualization focuses on the total number of suggestions accepted by users, showcasing the level of acceptance and adoption.

4. **Total Lines Suggested:** This chart showcases the total number of lines of code suggested by GitHub Copilot, giving an idea of the volume of code generation and assistance provided.

5. **Total Lines Accepted:** This metric represents the total lines of code accepted by users, offering insights into how much of the suggested code is actually being utilized and incorporated into the codebase.

6. **Total Active Users:** This chart represents the number of active users engaging with GitHub Copilot, helping to understand the user base growth and adoption rate.

## Languages Breakdown Analysis

The GitHub Copilot Metrics Viewer also provides a breakdown analysis of the top 5 languages by accepted prompts and acceptance rate. Pie charts are displayed at the top, giving a visual representation of the language distribution.

Additionally, a table is shown, presenting the Accepted Prompts, Accepted Lines of Code, and Acceptance Rate (%) for each language over the past 28 days. The entries are sorted by the number of accepted lines of code in descending order.

## Copilot Chat Metrics

The Copilot Chat Metrics section provides insights into the interactions between users and Copilot:

1. **Cumulative Number of Turns:** This metric represents the total number of turns (interactions) with Copilot over the past 28 days. A 'turn' includes both user inputs and Copilot's responses.

2. **Cumulative Number of Acceptances:** This metric shows the total number of lines of code suggested by Copilot that have been accepted by users over the past 28 days.

3. **Total Turns | Total Acceptances Count:** This chart displays the total number of turns and acceptances.

4. **Total Active Copilot Chat Users:** This bar chart illustrates the total number of users who have actively interacted with Copilot over the past 28 days.

## Seat Analysis

The Seat Analysis section provides insights into the utilization of Copilot seats within your organization or enterprise:

1. **Total Assigned:** This metric represents the total number of Copilot seats assigned within the current organization/enterprise.

2. **Assigned But Never Used:** This metric shows seats that were assigned but never used within the current organization/enterprise. The assigned timestamp is also displayed in the chart.

3. **No Activity in the Last 7 days:** This section displays seats that have either never been used or have had no activity in the past 7 days.

4. **No Activity in the Last 7 days (including never used seats):** This table displays seats that have had no activity in the past 7 days, ordered by the date of last activity. Seats that were used earlier are displayed at the top.

## Setup Instructions

To set up the GitHub Copilot Metrics Viewer, follow these steps:

1. Create a `.env` file in the root directory of the project.

2. Configure the following environment variables in the `.env` file:

   - `VUE_APP_SCOPE`: Set this variable to either 'enterprise' or 'organization' to determine the scope of the API calls made by the application.

   - `VUE_APP_GITHUB_ORG`: If `VUE_APP_SCOPE` is set to 'organization', provide the name of your GitHub Organization.

   - `VUE_APP_GITHUB_ENT`: If `VUE_APP_SCOPE` is set to 'enterprise', provide the name of your GitHub Enterprise account.

   - `VUE_APP_GITHUB_TEAM`: Set this variable to filter metrics for a specific GitHub team within your organization or enterprise. If not needed, leave it empty.

   - `VUE_APP_MOCKED_DATA`: Set this boolean variable to `false` to access actual Copilot metrics from the last 28 days via the API and display real data.

   - `VUE_APP_GITHUB_TOKEN`: Specify your GitHub Personal Access Token with the necessary scopes for API requests.

3. Install the project dependencies by running `npm install` in the terminal.

4. Compile and run the application using `npm run serve`.

5. Optionally, you can build and run the application using Docker. Build the Docker image with `docker build -t copilot-metrics-viewer .` and run it with `docker run -p 8080:80 --env-file ./.env copilot-metrics-viewer`.

The GitHub Copilot Metrics Viewer will be accessible at http://localhost:8080.

---

### New update with RBAC
