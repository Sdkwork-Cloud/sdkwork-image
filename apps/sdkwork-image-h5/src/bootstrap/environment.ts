export type Environment = "development" | "test" | "staging" | "production";

export interface RuntimeEnvironment {
  environment: Environment;
  apiBaseUrl: string;
}

export function resolveEnvironment(): RuntimeEnvironment {
  const env = (import.meta.env.VITE_ENVIRONMENT as Environment) ?? "development";
  return {
    environment: env,
    apiBaseUrl: import.meta.env.VITE_API_BASE_URL ?? "http://localhost:3000",
  };
}
