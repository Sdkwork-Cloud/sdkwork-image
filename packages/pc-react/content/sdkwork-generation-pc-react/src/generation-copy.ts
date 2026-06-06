export type SdkworkGenerationLocale = "en-US" | "zh-CN";

export type SdkworkGenerationMessagesOverrides = DeepPartial<SdkworkGenerationMessages>;

export interface SdkworkGenerationMessages {
  actions: {
    refresh: string;
  };
  detail: {
    emptyDescription: string;
    emptyTitle: string;
    latencyLabel: string;
    tokensLabel: string;
  };
  empty: {
    noRunsDescription: string;
    noRunsTitle: string;
  };
  list: {
    openRunTemplate: string;
  };
  page: {
    description: string;
    errorTitle: string;
    eyebrow: string;
    loading: string;
    searchLabel: string;
    searchPlaceholder: string;
    title: string;
  };
  service: {
    loadWorkspaceFailed: string;
  };
  sort: {
    alphabetical: string;
    latency: string;
    recent: string;
  };
  status: {
    all: string;
    completed: string;
    failed: string;
    queued: string;
    running: string;
  };
  summary: {
    completed: string;
    failed: string;
    running: string;
    tokensUsed: string;
    totalRuns: string;
  };
}

type DeepPartial<T> = {
  [K in keyof T]?: T[K] extends (...args: never[]) => unknown
    ? T[K]
    : T[K] extends object
      ? DeepPartial<T[K]>
      : T[K];
};

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function mergeDeep<T>(base: T, overrides?: DeepPartial<T>): T {
  if (!overrides) {
    return base;
  }

  const output: Record<string, unknown> = {
    ...(base as Record<string, unknown>),
  };

  for (const [key, value] of Object.entries(overrides)) {
    if (value === undefined) {
      continue;
    }

    const baseValue = output[key];
    output[key] = isRecord(baseValue) && isRecord(value)
      ? mergeDeep(baseValue, value)
      : value;
  }

  return output as T;
}

export function formatSdkworkGenerationTemplate(
  template: string,
  replacements: Record<string, string>,
): string {
  return Object.entries(replacements).reduce(
    (current, [key, value]) => current.replaceAll(`{${key}}`, value),
    template,
  );
}

const EN_US_MESSAGES: SdkworkGenerationMessages = {
  actions: {
    refresh: "Refresh runs",
  },
  detail: {
    emptyDescription: "Select a generation run to inspect prompt preview, model, and usage details.",
    emptyTitle: "No selected run",
    latencyLabel: "Latency",
    tokensLabel: "Tokens",
  },
  empty: {
    noRunsDescription: "No generation runs match the current filters.",
    noRunsTitle: "No runs",
  },
  list: {
    openRunTemplate: "Open run {title}",
  },
  page: {
    description: "Track generation history, filter run status, and inspect prompt/result provenance through reusable content capability primitives.",
    errorTitle: "Generation workspace error",
    eyebrow: "Content generation",
    loading: "Loading generation workspace...",
    searchLabel: "Search generation runs",
    searchPlaceholder: "Search generation runs",
    title: "Generation Workspace",
  },
  service: {
    loadWorkspaceFailed: "Failed to load generation workspace.",
  },
  sort: {
    alphabetical: "A-Z",
    latency: "Latency",
    recent: "Recent",
  },
  status: {
    all: "All",
    completed: "Completed",
    failed: "Failed",
    queued: "Queued",
    running: "Running",
  },
  summary: {
    completed: "Completed",
    failed: "Failed",
    running: "Running",
    tokensUsed: "Tokens",
    totalRuns: "Runs",
  },
};

const ZH_CN_MESSAGES: SdkworkGenerationMessages = {
  actions: {
    refresh: "\u5237\u65b0\u4efb\u52a1",
  },
  detail: {
    emptyDescription: "\u9009\u62e9\u4e00\u6761\u751f\u6210\u4efb\u52a1\uff0c\u67e5\u770b\u63d0\u793a\u8bcd\u9884\u89c8\u3001\u6a21\u578b\u4e0e\u7528\u91cf\u8be6\u60c5\u3002",
    emptyTitle: "\u672a\u9009\u62e9\u4efb\u52a1",
    latencyLabel: "\u5ef6\u8fdf",
    tokensLabel: "Token",
  },
  empty: {
    noRunsDescription: "\u5f53\u524d\u7b5b\u9009\u6761\u4ef6\u4e0b\u6ca1\u6709\u5339\u914d\u7684\u751f\u6210\u4efb\u52a1\u3002",
    noRunsTitle: "\u6682\u65e0\u4efb\u52a1",
  },
  list: {
    openRunTemplate: "\u6253\u5f00\u4efb\u52a1 {title}",
  },
  page: {
    description: "\u96c6\u4e2d\u67e5\u770b\u751f\u6210\u5386\u53f2\u3001\u8fd0\u884c\u72b6\u6001\u4e0e\u63d0\u793a\u8bcd\u7ed3\u679c\u6765\u6e90\uff0c\u4f5c\u4e3a\u53ef\u590d\u7528\u7684\u5185\u5bb9\u751f\u6210\u5de5\u4f5c\u53f0\u3002",
    errorTitle: "\u751f\u6210\u5de5\u4f5c\u53f0\u5f02\u5e38",
    eyebrow: "\u5185\u5bb9\u751f\u6210",
    loading: "\u6b63\u5728\u52a0\u8f7d\u751f\u6210\u5de5\u4f5c\u53f0...",
    searchLabel: "\u641c\u7d22\u751f\u6210\u4efb\u52a1",
    searchPlaceholder: "\u641c\u7d22\u751f\u6210\u4efb\u52a1",
    title: "\u751f\u6210\u5de5\u4f5c\u53f0",
  },
  service: {
    loadWorkspaceFailed: "\u52a0\u8f7d\u751f\u6210\u5de5\u4f5c\u53f0\u5931\u8d25\u3002",
  },
  sort: {
    alphabetical: "A-Z",
    latency: "\u5ef6\u8fdf",
    recent: "\u6700\u65b0",
  },
  status: {
    all: "\u5168\u90e8",
    completed: "\u5df2\u5b8c\u6210",
    failed: "\u5931\u8d25",
    queued: "\u6392\u961f\u4e2d",
    running: "\u8fd0\u884c\u4e2d",
  },
  summary: {
    completed: "\u5df2\u5b8c\u6210",
    failed: "\u5931\u8d25",
    running: "\u8fd0\u884c\u4e2d",
    tokensUsed: "Token",
    totalRuns: "\u4efb\u52a1\u603b\u6570",
  },
};

const SDKWORK_GENERATION_MESSAGES: Record<SdkworkGenerationLocale, SdkworkGenerationMessages> = {
  "en-US": EN_US_MESSAGES,
  "zh-CN": ZH_CN_MESSAGES,
};

export function normalizeSdkworkGenerationLocale(locale?: string | null): SdkworkGenerationLocale {
  const normalized = String(locale || "").trim().toLowerCase();
  if (normalized.startsWith("zh")) {
    return "zh-CN";
  }

  return "en-US";
}

export function createSdkworkGenerationMessages(
  locale?: string | null,
  overrides?: SdkworkGenerationMessagesOverrides,
): SdkworkGenerationMessages {
  return mergeDeep(
    SDKWORK_GENERATION_MESSAGES[normalizeSdkworkGenerationLocale(locale)],
    overrides,
  );
}
