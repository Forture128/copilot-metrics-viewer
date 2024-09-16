//Make a call to the GitHub API to get Copilot Metrics, the API is https://api.github.com/orgs/toussaintt/copilot/usage
//Add the header Accept: application/vnd.github+json to the request
//Add also the Authorization: Bearer <token> header where <token> is hardcoded for now
//Also add X-GitHub-Api-Version: 2022-11-28 header
//Return the response from the API

import axios from "axios";

import { Metrics } from "../model/Metrics";
import organizationMockedResponse from '../assets/organization_response_sample.json';
import enterpriseMockedResponse from '../assets/enterprise_response_sample.json';
import teamMockedResponse from '../assets/teams_response.json';
import config from '../config';
import { Team } from "@/model/Teams";

export const getMetricsApi = async (): Promise<Metrics[]> => {

  let response;
  let metricsData;
  console.log("Config ", config);

  if (config.mockedData) {
    console.log("Using mock data. Check VUE_APP_MOCKED_DATA variable.");
    response = config.scope.type === "organization" ? organizationMockedResponse : enterpriseMockedResponse;
    metricsData = response.map((item: any) => new Metrics(item));
  } else {
    response = await axios.get(
      `${config.github.apiUrl}/copilot/usage`,
      {
        headers: {
          Accept: "application/vnd.github+json",
          Authorization: `Bearer ${config.github.token}`,
          "X-GitHub-Api-Version": "2022-11-28",
        },
      }
    );


    metricsData = response.data.map((item: any) => new Metrics(item));
  }
  return metricsData;
};

export const getTeams = async (): Promise<Team[]> => {
  console
  let response;
  let teamData;

  // If config mockdata is enabled, return a mocked response
  if (config.mockedData) {
    response = teamMockedResponse;
    // map with Team object
    teamData = response.map((item: any) => new Team(item));
  } else {
    response = await axios.get(`${config.github.apiUrl}/teams`, {
      headers: {
        Accept: 'application/vnd.github+json',
        Authorization: `Bearer ${config.github.token}`,
        'X-GitHub-Api-Version': '2022-11-28',
      },
    });
    teamData = response.data.map((item: any) => new Team(item));
  }
  return teamData;
}

export const getTeamMetricsApi = async (team_tag: string): Promise<Metrics[]> => {
  let response;
  let metricsData;

  if (config.mockedData) {
    response = config.scope.type === "organization" ? organizationMockedResponse : enterpriseMockedResponse;
    metricsData = response.map((item: any) => new Metrics(item));
  } else {
    try {
      response = await axios.get(
        `${config.github.apiUrl}/team/${team_tag}/copilot/usage`,
        {
          headers: {
            Accept: "application/vnd.github+json",
            Authorization: `Bearer ${config.github.token}`,
            "X-GitHub-Api-Version": "2022-11-28",
          },
        }
      );

      if (response.status === 200) {
        metricsData = response.data.map((item: any) => new Metrics(item));
      } else {
        console.error(`Error: Received status code ${response.status}`);
        metricsData = [];
      }
    } catch (error) {
      console.error('Error fetching team metrics:', error);
      metricsData = [];
    }
  }
  return metricsData;
};