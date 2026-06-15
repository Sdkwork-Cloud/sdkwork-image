export type Environment = "development" | "test" | "staging" | "production";
export type DeploymentMode = "saas" | "private" | "local" | "test";

export interface RuntimeEnvironment {
  environment: Environment;
  deploymentMode: DeploymentMode;
  apiBaseUrl: string;
  appBaseUrl: string;
}

export function resolveEnvironment(): RuntimeEnvironment {
  const env = (import.meta.env.VITE_ENVIRONMENT as Environment) ?? "development";
  return {
    environment: env,
    deploymentMode: env === "development" ? "local" : "saas",
    apiBaseUrl: import.meta.env.VITE_API_BASE_URL ?? "http://localhost:3000",
    appBaseUrl: import.meta.env.VITE_APP_BASE_URL ?? "http://localhost:5173",
  };
}
