import {
  createEmptySdkworkGenerationWorkspace,
  type SdkworkGenerationRun,
  type SdkworkGenerationWorkspaceData,
} from "./generation.ts";

export type {
  SdkworkGenerationRun,
  SdkworkGenerationStatus,
  SdkworkGenerationWorkspaceData,
} from "./generation.ts";

export interface CreateSdkworkGenerationServiceOptions {
  getSessionTokens?: () => {
    authToken?: string;
  };
  includeSampleRuns?: boolean;
  listRuns?: () => Promise<readonly SdkworkGenerationRun[]>;
  runs?: readonly SdkworkGenerationRun[];
}

export interface SdkworkGenerationService {
  getEmptyWorkspace(): SdkworkGenerationWorkspaceData;
  getWorkspace(): Promise<SdkworkGenerationWorkspaceData>;
}

function normalizeText(value: string | undefined): string {
  return (value ?? "").trim().toLowerCase();
}

function resolveSettledValue<T>(
  result: PromiseSettledResult<T>,
  fallback: T,
): T {
  return result.status === "fulfilled" ? result.value : fallback;
}

function readDefaultGenerationSessionTokens(): { authToken?: string } {
  return {};
}

export function createSdkworkGenerationService(
  options: CreateSdkworkGenerationServiceOptions = {},
): SdkworkGenerationService {
  const getSessionTokens = options.getSessionTokens ?? readDefaultGenerationSessionTokens;
  const fallbackWorkspace = createEmptySdkworkGenerationWorkspace({
    includeSampleRuns: options.includeSampleRuns,
    runs: options.runs,
  });

  return {
    getEmptyWorkspace() {
      return createEmptySdkworkGenerationWorkspace({
        includeSampleRuns: options.includeSampleRuns,
        isAuthenticated: Boolean(normalizeText(getSessionTokens().authToken)),
        runs: options.runs ?? fallbackWorkspace.runs,
      });
    },

    async getWorkspace() {
      const isAuthenticated = Boolean(normalizeText(getSessionTokens().authToken));
      if (!options.listRuns) {
        return createEmptySdkworkGenerationWorkspace({
          includeSampleRuns: options.includeSampleRuns,
          isAuthenticated,
          runs: options.runs ?? fallbackWorkspace.runs,
        });
      }

      const results = await Promise.allSettled([options.listRuns()]);
      const runs = resolveSettledValue(results[0], options.runs ?? fallbackWorkspace.runs);

      return createEmptySdkworkGenerationWorkspace({
        includeSampleRuns: options.includeSampleRuns,
        isAuthenticated,
        runs,
      });
    },
  };
}

export const sdkworkGenerationService = createSdkworkGenerationService();
