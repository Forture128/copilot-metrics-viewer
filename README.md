This application is designed to provide clear visualizations of various metrics related to GitHub Copilot for your GitHub Organization or Enterprise Account. It utilizes the [GitHub Copilot Metrics API](https://docs.github.com/en/enterprise-cloud@latest/rest/copilot/copilot-usage?apiVersion=2022-11-28) to gather the necessary data.

## Video

You can watch a video demonstration of the GitHub Copilot Metrics Viewer [here](https://github.com/github-copilot-resources/copilot-metrics-viewer/assets/3329307/bc7e2a16-cc73-43c4-887a-b50809c08533).

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

## License

This project is licensed under the terms of the MIT open source license. Please refer to the [LICENSE.txt](./LICENSE.txt) file for the full terms.

## Support

This project is not an official GitHub product and is maintained by dedicated contributors. Support is provided through [GitHub Issues](https://github.com/github-copilot-resources/copilot-metrics-viewer/issues). Please note that responses may not be immediate, but critical issues will be prioritized.

## Fork Information

This repository is a fork of the original project by [@martedesco](https://github.com/martedesco). It includes custom metrics and layouts for personal and study purposes.
