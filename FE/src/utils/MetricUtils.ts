// utils/metricsUtils.ts
import { Metrics } from "../model/Metrics";

export const calculateTeamAcceptanceRate = (teamMetrics: Metrics[]): number => {
  const totalSuggestions = teamMetrics.reduce(
    (sum, m) => sum + m.total_suggestions_count,
    0
  );
  const totalAcceptances = teamMetrics.reduce(
    (sum, m) => sum + m.total_acceptances_count,
    0
  );
  return totalSuggestions > 0
    ? (totalAcceptances / totalSuggestions) * 100
    : 0;
};

export const calculateTeamCumulativeMetrics = (teamMetricsData: any[]) => {
  let cumulativeNumberSuggestions = 0;
  let cumulativeNumberAcceptances = 0;
  let cumulativeNumberLOCAccepted = 0;

  teamMetricsData.forEach((team) => {
    team.metrics.forEach((m: Metrics) => {
      cumulativeNumberSuggestions += m.total_suggestions_count;
      cumulativeNumberAcceptances += m.total_acceptances_count;
      cumulativeNumberLOCAccepted += m.total_lines_accepted;
    });
  });

  return {
    cumulativeNumberSuggestions,
    cumulativeNumberAcceptances,
    cumulativeNumberLOCAccepted,
    acceptanceRateAverage: (cumulativeNumberAcceptances / cumulativeNumberSuggestions) * 100,
  };
};

export const calculateCumulativeMetrics = (metrics: Metrics[]) => {
    let cumulativeNumberSuggestions = 0;
    let cumulativeNumberAcceptances = 0;
    let cumulativeNumberLOCAccepted = 0;
  
    metrics.forEach((m) => {
      cumulativeNumberSuggestions += m.total_suggestions_count;
      cumulativeNumberAcceptances += m.total_acceptances_count;
      cumulativeNumberLOCAccepted += m.total_lines_accepted;
    });
  
    return {
      cumulativeNumberSuggestions,
      cumulativeNumberAcceptances,
      cumulativeNumberLOCAccepted,
      acceptanceRateAverage: (cumulativeNumberAcceptances / cumulativeNumberSuggestions) * 100,
    };
  };