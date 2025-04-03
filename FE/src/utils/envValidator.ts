/**
 * Environment variable validation utility
 * This module ensures all required environment variables are properly set
 * and contain valid values before the application starts.
 */

interface EnvConfig {
  mockedData: boolean
  scope: 'enterprise' | 'organization'
  githubOrg: string
  githubEnt?: string
  githubTeam?: string
}

/**
 * Validates the environment configuration
 * @throws {Error} If any required environment variables are missing or invalid
 */
export function validateEnv(): EnvConfig {
  const errors: string[] = []

  // Check if we're using Vite or Vue environment variables
  const isVite = import.meta.env !== undefined
  const env = isVite ? import.meta.env : process.env

  // Validate mocked data setting
  const mockedData = env.VITE_APP_MOCKED_DATA || env.VUE_APP_MOCKED_DATA
  if (mockedData === undefined) {
    errors.push('MOCKED_DATA environment variable is not set')
  }

  // Validate scope
  const scope = env.VITE_APP_SCOPE || env.VUE_APP_SCOPE
  if (!scope || !['enterprise', 'organization'].includes(scope)) {
    errors.push('SCOPE must be either "enterprise" or "organization"')
  }

  // Validate GitHub organization
  const githubOrg = env.VITE_APP_GITHUB_ORG || env.VUE_APP_GITHUB_ORG
  if (!githubOrg || githubOrg === 'your_organization_name') {
    errors.push('GITHUB_ORG must be set to your actual organization name')
  }

  // Validate enterprise name if using enterprise scope
  if (scope === 'enterprise') {
    const githubEnt = env.VITE_APP_GITHUB_ENT || env.VUE_APP_GITHUB_ENT
    if (!githubEnt) {
      errors.push('GITHUB_ENT must be set when using enterprise scope')
    }
  }

  // If there are any errors, throw them
  if (errors.length > 0) {
    throw new Error(
      `Environment validation failed:\n${errors.join('\n')}\n\n` +
        'Please check your .env file and ensure all required variables are set correctly.'
    )
  }

  // Return validated configuration
  return {
    mockedData: mockedData === 'true',
    scope: scope as 'enterprise' | 'organization',
    githubOrg,
    githubEnt: env.VITE_APP_GITHUB_ENT || env.VUE_APP_GITHUB_ENT,
    githubTeam: env.VITE_APP_GITHUB_TEAM || env.VUE_APP_GITHUB_TEAM
  }
}

/**
 * Type guard to check if a value is a valid environment configuration
 */
export function isValidEnvConfig(config: unknown): config is EnvConfig {
  if (!config || typeof config !== 'object') return false

  const c = config as EnvConfig
  return (
    typeof c.mockedData === 'boolean' &&
    ['enterprise', 'organization'].includes(c.scope) &&
    typeof c.githubOrg === 'string' &&
    (!c.githubEnt || typeof c.githubEnt === 'string') &&
    (!c.githubTeam || typeof c.githubTeam === 'string')
  )
}
