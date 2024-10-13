import axios from 'axios';
import config from '../config';



const GitHubService = {
  async getDeployments(owner: string, repo: string) {
    let response;
    let deploymentsData;

    if (config.mockedData) {
      // Handle mocked data if necessary
      deploymentsData = []; // Replace with actual mocked data if available
    } else {
      try {
        response = await axios.get(`${config.github.baseUrl}/repos/${owner}/${repo}/deployments`, {
          headers: {
            Accept: 'application/vnd.github+json',
            Authorization: `Bearer ${config.github.token}`,
            'X-GitHub-Api-Version': '2022-11-28',
          },
        });

        if (response.status === 200) {
          deploymentsData = response.data;
        } else {
          console.error(`Error: Received status code ${response.status}`);
          deploymentsData = [];
        }
      } catch (error) {
        console.error('Error fetching deployments:', error);
        deploymentsData = [];
      }
    }
    return deploymentsData;
  },

  // Fetch Pull Requests for Lead Time for Changes
  async getPullRequests(owner: string, repo: string, params: Record<string, any> = {}) {
    let response;
    let pullRequestsData;

    if (config.mockedData) {
      // Handle mocked data if necessary
      pullRequestsData = []; // Replace with actual mocked data if available
    } else {
      try {
        response = await axios.get(`${config.github.baseUrl}/repos/${owner}/${repo}/pulls`, {
          headers: {
            Accept: 'application/vnd.github+json',
            Authorization: `Bearer ${config.github.token}`,
            'X-GitHub-Api-Version': '2022-11-28',
          },
          params,
        });

        if (response.status === 200) {
          pullRequestsData = response.data;
        } else {
          console.error(`Error: Received status code ${response.status}`);
          pullRequestsData = [];
        }
      } catch (error) {
        console.error('Error fetching pull requests:', error);
        pullRequestsData = [];
      }
    }
    return pullRequestsData;
  },

  // Fetch Deployment Statuses for Change Failure Rate
  async getDeploymentStatuses(owner: string, repo: string, deploymentId: number) {
    let response;
    let statusesData;

    if (config.mockedData) {
      // Handle mocked data if necessary
      statusesData = []; // Replace with actual mocked data if available
    } else {
      try {
        response = await axios.get(`${config.github.baseUrl}/repos/${owner}/${repo}/deployments/${deploymentId}/statuses`, {
          headers: {
            Accept: 'application/vnd.github+json',
            Authorization: `Bearer ${config.github.token}`,
            'X-GitHub-Api-Version': '2022-11-28',
          },
        });

        if (response.status === 200) {
          statusesData = response.data;
        } else {
          console.error(`Error: Received status code ${response.status}`);
          statusesData = [];
        }
      } catch (error) {
        console.error('Error fetching deployment statuses:', error);
        statusesData = [];
      }
    }
    return statusesData;
  },

  // Fetch Commits for Lead Time for Changes
  async getCommits(owner: string, repo: string) {
    let response;
    let commitsData;

    if (config.mockedData) {
      // Handle mocked data if necessary
      commitsData = []; // Replace with actual mocked data if available
    } else {
      try {
        response = await axios.get(`${config.github.baseUrl}/repos/${owner}/${repo}/commits`, {
          headers: {
            Accept: 'application/vnd.github+json',
            Authorization: `Bearer ${config.github.token}`,
            'X-GitHub-Api-Version': '2022-11-28',
          },
        });

        if (response.status === 200) {
          commitsData = response.data;
        } else {
          console.error(`Error: Received status code ${response.status}`);
          commitsData = [];
        }
      } catch (error) {
        console.error('Error fetching commits:', error);
        commitsData = [];
      }
    }
    return commitsData;
  },

  // Fetch Workflow Runs for Deployment Frequency
  async getWorkflowRuns(owner: string, repo: string) {
    let response;
    let workflowRunsData;

    if (config.mockedData) {
      // Handle mocked data if necessary
      workflowRunsData = []; // Replace with actual mocked data if available
    } else {
      try {
        response = await axios.get(`${config.github.baseUrl}/repos/${owner}/${repo}/actions/runs`, {
          headers: {
            Accept: 'application/vnd.github+json',
            Authorization: `Bearer ${config.github.token}`,
            'X-GitHub-Api-Version': '2022-11-28',
          },
        });

        if (response.status === 200) {
          workflowRunsData = response.data;
        } else {
          console.error(`Error: Received status code ${response.status}`);
          workflowRunsData = [];
        }
      } catch (error) {
        console.error('Error fetching workflow runs:', error);
        workflowRunsData = [];
      }
    }
    return workflowRunsData;
  },

  // Fetch Issues for Mean Time to Restore (MTTR)
  async getIssues(owner: string, repo: string) {
    let response;
    let issuesData;

    if (config.mockedData) {
      // Handle mocked data if necessary
      issuesData = []; // Replace with actual mocked data if available
    } else {
      try {
        response = await axios.get(`${config.github.baseUrl}/repos/${owner}/${repo}/issues`, {
          headers: {
            Accept: 'application/vnd.github+json',
            Authorization: `Bearer ${config.github.token}`,
            'X-GitHub-Api-Version': '2022-11-28',
          },
        });

        if (response.status === 200) {
          issuesData = response.data;
        } else {
          console.error(`Error: Received status code ${response.status}`);
          issuesData = [];
        }
      } catch (error) {
        console.error('Error fetching issues:', error);
        issuesData = [];
      }
    }
    return issuesData;
  },

  async getTimePullRequestReviews(owner: string, repo: string, pull_number: number) {
    try {
      const response = await axios.get(`${config.github.baseUrl}/repos/${owner}/${repo}/pulls/${pull_number}/reviews`, {
        headers: {
          Accept: 'application/vnd.github+json',
          Authorization: `Bearer ${config.github.token}`,
        },
      });

      const reviews = response.data;
      if (reviews.length > 0) {
        const firstReview = reviews[0];
        const approvedReview = reviews.find((review: { state: string }) => review.state === 'APPROVED');
        const lastReview = reviews[reviews.length - 1];

        return {
          first_review_at: firstReview.submitted_at,
          approved_at: approvedReview ? approvedReview.submitted_at : null,
          review_completed_at: lastReview.submitted_at,
        };
      }
      return null;
    } catch (error) {
      console.error('Error fetching PR reviews:', error);
      return null;
    }
  },


  // Get repository information
  async getRepository(owner: string, repo: string) {
    let response;
    let repositoryData;

    if (config.mockedData) {
      // Handle mocked data if necessary
      repositoryData = {}; // Replace with actual mocked data if available
    } else {
      try {
        response = await axios.get(`${config.github.baseUrl}/repos/${owner}/${repo}`, {
          headers: {
            Accept: 'application/vnd.github+json',
            Authorization: `Bearer ${config.github.token}`,
            'X-GitHub-Api-Version': '2022-11-28',
          },
        });

        if (response.status === 200) {
          repositoryData = response.data;
        } else {
          console.error(`Error: Received status code ${response.status}`);
          repositoryData = {};
        }
      } catch (error) {
        console.error('Error fetching repository:', error);
        repositoryData = {};
      }
    }
    return repositoryData;
  },

  // Get List repos
  // Get List repos with sorting and pagination
  async getRepositories(page = 1) {
    let response;
    let reposData = [];

    if (config.mockedData) {
      // Handle mocked data if necessary
      reposData = []; // Replace with actual mocked data if available
    } else {
      try {
        response = await axios.get(`${config.github.baseUrl}/user/repos`, {
          headers: {
            Accept: 'application/vnd.github+json',
            Authorization: `Bearer ${config.github.token}`,
            'X-GitHub-Api-Version': '2022-11-28',
          },
          params: {
            sort: 'updated',
            per_page: 50,
            page: page,
          },
        });

        if (response.status === 200) {
          reposData = response.data;
        } else {
          console.error(`Error: Received status code ${response.status}`);
        }
      } catch (error) {
        console.error('Error fetching repos:', error);
      }
    }
    return reposData;
  },
};

export default GitHubService;