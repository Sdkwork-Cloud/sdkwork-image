export type SdkworkGenerationStatus = "completed" | "failed" | "queued" | "running";
export type SdkworkGenerationSortBy = "alphabetical" | "latency" | "recent";

export interface SdkworkGenerationRun {
  id: string;
  latencyMs: number;
  model: string;
  promptPreview: string;
  status: SdkworkGenerationStatus;
  title: string;
  tokensUsed: number;
  updatedAt: string;
}

export interface SdkworkGenerationDigest {
  completedRuns: number;
  failedRuns: number;
  runningRuns: number;
  totalRuns: number;
  totalTokensUsed: number;
}

export interface SdkworkGenerationWorkspaceData {
  digest: SdkworkGenerationDigest;
  isAuthenticated: boolean;
  runs: SdkworkGenerationRun[];
}

export interface SdkworkGenerationCapabilityManifest {
  description: string;
  host?: string;
  id: string;
  packageNames: string[];
  theme?: string;
  title: string;
}

export interface SdkworkGenerationWorkspaceManifest extends SdkworkGenerationCapabilityManifest {
  capability: "generation";
  routePath: string;
}

export interface CreateGenerationWorkspaceManifestOptions
  extends Partial<
    Pick<SdkworkGenerationCapabilityManifest, "description" | "host" | "id" | "packageNames" | "theme" | "title">
  > {
  routePath?: string;
}

export interface SdkworkGenerationRouteIntent {
  focusWindow: boolean;
  route: string;
  runId?: string;
  source: "generation-workspace";
  type: "generation-route-intent";
}

export interface CreateGenerationRouteIntentOptions {
  basePath?: string;
  focusWindow?: boolean;
  runId?: string;
}

export interface CreateEmptySdkworkGenerationWorkspaceOptions {
  includeSampleRuns?: boolean;
  isAuthenticated?: boolean;
  runs?: readonly SdkworkGenerationRun[];
}

function normalizeBasePath(basePath: string | undefined): string {
  const normalized = (basePath ?? "/generation").trim();
  if (!normalized || normalized === "/") {
    return "/generation";
  }

  return normalized.endsWith("/") ? normalized.slice(0, -1) : normalized;
}

function toTimestamp(value: string): number {
  const timestamp = Date.parse(value);
  return Number.isFinite(timestamp) ? timestamp : 0;
}

export function createDefaultSdkworkGenerationRuns(): SdkworkGenerationRun[] {
  return [
    {
      id: "run-desktop-hero",
      latencyMs: 1320,
      model: "gpt-5.4-mini",
      promptPreview: "Create a premium desktop hero section with strong typographic hierarchy.",
      status: "completed",
      title: "Desktop Hero Draft",
      tokensUsed: 4860,
      updatedAt: "2026-04-03T02:30:00.000Z",
    },
    {
      id: "run-workflow-canvas",
      latencyMs: 2410,
      model: "gpt-5.4",
      promptPreview: "Generate workflow canvas node labels and deterministic edge mapping.",
      status: "running",
      title: "Workflow Canvas Expansion",
      tokensUsed: 2150,
      updatedAt: "2026-04-03T01:40:00.000Z",
    },
    {
      id: "run-release-summary",
      latencyMs: 1880,
      model: "gpt-5.4-mini",
      promptPreview: "Summarize release verification logs and unresolved risks.",
      status: "failed",
      title: "Release Summary",
      tokensUsed: 3010,
      updatedAt: "2026-04-02T23:20:00.000Z",
    },
  ];
}

export function sortSdkworkGenerationRuns(
  runs: readonly SdkworkGenerationRun[],
  sortBy: SdkworkGenerationSortBy = "recent",
): SdkworkGenerationRun[] {
  return [...runs].sort((left, right) => {
    if (sortBy === "alphabetical") {
      return left.title.localeCompare(right.title);
    }
    if (sortBy === "latency") {
      return left.latencyMs - right.latencyMs
        || toTimestamp(right.updatedAt) - toTimestamp(left.updatedAt);
    }
    return toTimestamp(right.updatedAt) - toTimestamp(left.updatedAt)
      || left.title.localeCompare(right.title);
  });
}

export function summarizeSdkworkGenerationWorkspace(
  runs: readonly SdkworkGenerationRun[],
): SdkworkGenerationDigest {
  return {
    completedRuns: runs.filter((run) => run.status === "completed").length,
    failedRuns: runs.filter((run) => run.status === "failed").length,
    runningRuns: runs.filter((run) => run.status === "running").length,
    totalRuns: runs.length,
    totalTokensUsed: runs.reduce((total, run) => total + run.tokensUsed, 0),
  };
}

export function createEmptySdkworkGenerationWorkspace(
  options: CreateEmptySdkworkGenerationWorkspaceOptions = {},
): SdkworkGenerationWorkspaceData {
  const hasExplicitRuns = options.runs !== undefined;
  const runs = sortSdkworkGenerationRuns(
    hasExplicitRuns
      ? options.runs ?? []
      : options.includeSampleRuns === false
        ? []
        : createDefaultSdkworkGenerationRuns(),
    "recent",
  );

  return {
    digest: summarizeSdkworkGenerationWorkspace(runs),
    isAuthenticated: Boolean(options.isAuthenticated),
    runs,
  };
}

export function createGenerationWorkspaceManifest({
  description = "Generation capability for deterministic run history, filterable status views, and route intents.",
  host,
  id = "sdkwork-generation",
  packageNames = [
    "@sdkwork/image-pc-generation",
    "@sdkwork/canvas-pc-react",
  ],
  routePath = "/generation",
  theme,
  title = "Generation Workspace",
}: CreateGenerationWorkspaceManifestOptions = {}): SdkworkGenerationWorkspaceManifest {
  return {
    capability: "generation",
    description,
    ...(host ? { host } : {}),
    id,
    packageNames: [...packageNames],
    routePath: normalizeBasePath(routePath),
    ...(theme ? { theme } : {}),
    title,
  };
}

export function createGenerationRouteIntent(
  options: CreateGenerationRouteIntentOptions = {},
): SdkworkGenerationRouteIntent {
  const basePath = normalizeBasePath(options.basePath);
  const params = new URLSearchParams();
  if (options.runId) {
    params.set("runId", options.runId);
  }

  return {
    focusWindow: options.focusWindow !== false,
    route: params.toString() ? `${basePath}?${params.toString()}` : basePath,
    ...(options.runId ? { runId: options.runId } : {}),
    source: "generation-workspace",
    type: "generation-route-intent",
  };
}

export const generationPackageMeta = {
  architecture: "pc-react",
  domain: "content",
  package: "@sdkwork/image-pc-generation",
  status: "ready",
} as const;

export type GenerationPackageMeta = typeof generationPackageMeta;
