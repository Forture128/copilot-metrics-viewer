/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_APP_MOCKED_DATA: string
  readonly VITE_APP_SCOPE: string
  readonly VITE_APP_GITHUB_ORG: string
  readonly VITE_APP_GITHUB_ENT: string
  readonly VITE_APP_GITHUB_TOKEN: string
  readonly VITE_APP_GITHUB_TEAM: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
