const PROPS = ['MOCKED_DATA', 'SCOPE', 'GITHUB_ORG', 'GITHUB_ENT', 'GITHUB_TEAM', 'GITHUB_TOKEN']
const env: any = {}
PROPS.forEach(prop => {
  const propName = `VITE_APP_${prop}`
  if (import.meta.env.PROD) {
    env[propName] = (window as any)['_ENV_'][`VUE_APP_${prop}`]
  } else {
    env[propName] = import.meta.env[propName]
  }
})

const VALID_SCOPE = ['organization', 'enterprise']

let scopeType
if (VALID_SCOPE.includes(env.VITE_APP_SCOPE)) {
  scopeType = env.VITE_APP_SCOPE as 'enterprise' | 'organization'
}

let apiUrl: string
const baseUrl = 'https://api.github.com'
const githubOrgName = env.VITE_APP_GITHUB_ORG
const githubEntName = env.VITE_APP_GITHUB_ENT

let scopeName: string
if (scopeType === 'organization') {
  scopeName = githubOrgName
  apiUrl = `https://api.github.com/orgs/${githubOrgName}`
} else if (scopeType === 'enterprise') {
  scopeName = githubEntName
  apiUrl = `https://api.github.com/enterprises/${githubEntName}`
} else {
  throw new Error(
    `Invalid VITE_APP_SCOPE value: ${env.VITE_APP_SCOPE}. Valid values: ${VALID_SCOPE.join(', ')}`
  )
}

const config: Config = {
  mockedData: env.VITE_APP_MOCKED_DATA === 'true',
  scope: {
    type: scopeType,
    name: scopeName
  },
  github: {
    org: githubOrgName,
    ent: githubEntName,
    team: env.VITE_APP_GITHUB_TEAM,
    token: env.VITE_APP_GITHUB_TOKEN,
    apiUrl,
    baseUrl
  }
}
if (!config.mockedData && !config.github.token) {
  throw new Error('VITE_APP_GITHUB_TOKEN environment variable must be set.')
}
export default config

interface Config {
  mockedData: boolean
  scope: {
    type: 'organization' | 'enterprise'
    name: string
  }
  github: {
    org: string
    ent: string
    team: string
    token: string
    apiUrl: string
    baseUrl: string
  }
}
