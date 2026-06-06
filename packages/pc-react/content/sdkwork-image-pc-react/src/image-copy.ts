export type SdkworkImageLocale = "en-US" | "zh-CN";

export type SdkworkImageMessagesOverrides = DeepPartial<SdkworkImageMessages>;

export interface SdkworkImageMessages {
  empty: {
    noImagesDescription: string;
    noImagesTitle: string;
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
  presets: {
    all: string;
  };
  service: {
    loadWorkspaceFailed: string;
  };
  status: {
    all: string;
    queued: string;
    ready: string;
    rendering: string;
  };
  summary: {
    activeRenders: string;
    presets: string;
    readyImages: string;
    totalImages: string;
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

const EN_US_MESSAGES: SdkworkImageMessages = {
  empty: {
    noImagesDescription: "No images match the current filters.",
    noImagesTitle: "No images",
  },
  page: {
    description: "Track image presets, prompt jobs, and gallery-ready outputs from a reusable generation surface.",
    errorTitle: "Image workspace error",
    eyebrow: "Image generation",
    loading: "Loading image workspace...",
    searchLabel: "Search images",
    searchPlaceholder: "Search images",
    title: "Image Workspace",
  },
  presets: {
    all: "All presets",
  },
  service: {
    loadWorkspaceFailed: "Failed to load image workspace.",
  },
  status: {
    all: "All",
    queued: "Queued",
    ready: "Ready",
    rendering: "Rendering",
  },
  summary: {
    activeRenders: "Active renders",
    presets: "Presets",
    readyImages: "Ready images",
    totalImages: "Total images",
  },
};

const ZH_CN_MESSAGES: SdkworkImageMessages = {
  empty: {
    noImagesDescription: "\u5f53\u524d\u7b5b\u9009\u6761\u4ef6\u4e0b\u6ca1\u6709\u5339\u914d\u7684\u56fe\u50cf\u3002",
    noImagesTitle: "\u6682\u65e0\u56fe\u50cf",
  },
  page: {
    description: "\u7edf\u4e00\u67e5\u770b\u56fe\u50cf\u9884\u8bbe\u3001\u63d0\u793a\u8bcd\u4efb\u52a1\u548c\u53ef\u7528\u4e8e\u753b\u5eca\u7684\u8f93\u51fa\u7ed3\u679c\u3002",
    errorTitle: "\u56fe\u50cf\u5de5\u4f5c\u53f0\u5f02\u5e38",
    eyebrow: "\u56fe\u50cf\u751f\u6210",
    loading: "\u6b63\u5728\u52a0\u8f7d\u56fe\u50cf\u5de5\u4f5c\u53f0...",
    searchLabel: "\u641c\u7d22\u56fe\u50cf",
    searchPlaceholder: "\u641c\u7d22\u56fe\u50cf",
    title: "\u56fe\u50cf\u5de5\u4f5c\u53f0",
  },
  presets: {
    all: "\u5168\u90e8\u9884\u8bbe",
  },
  service: {
    loadWorkspaceFailed: "\u52a0\u8f7d\u56fe\u50cf\u5de5\u4f5c\u53f0\u5931\u8d25\u3002",
  },
  status: {
    all: "\u5168\u90e8",
    queued: "\u6392\u961f\u4e2d",
    ready: "\u5df2\u5c31\u7eea",
    rendering: "\u751f\u6210\u4e2d",
  },
  summary: {
    activeRenders: "\u6d3b\u8dc3\u6e32\u67d3",
    presets: "\u9884\u8bbe",
    readyImages: "\u5df2\u5c31\u7eea\u56fe\u50cf",
    totalImages: "\u56fe\u50cf\u603b\u6570",
  },
};

const SDKWORK_IMAGE_MESSAGES: Record<SdkworkImageLocale, SdkworkImageMessages> = {
  "en-US": EN_US_MESSAGES,
  "zh-CN": ZH_CN_MESSAGES,
};

export function normalizeSdkworkImageLocale(locale?: string | null): SdkworkImageLocale {
  const normalized = String(locale || "").trim().toLowerCase();
  if (normalized.startsWith("zh")) {
    return "zh-CN";
  }

  return "en-US";
}

export function createSdkworkImageMessages(
  locale?: string | null,
  overrides?: SdkworkImageMessagesOverrides,
): SdkworkImageMessages {
  return mergeDeep(
    SDKWORK_IMAGE_MESSAGES[normalizeSdkworkImageLocale(locale)],
    overrides,
  );
}
