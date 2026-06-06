import type { SdkworkMediaResource } from "@sdkwork/image-contracts";

export type SdkworkImageJobStatus = "queued" | "ready" | "rendering";

export interface SdkworkImageAsset {
  id: string;
  presetId: string;
  prompt: string;
  resource: SdkworkMediaResource;
  resolution: string;
  status: SdkworkImageJobStatus;
  style: string;
  title: string;
  updatedAt: string;
}

export interface SdkworkImagePreset {
  id: string;
  itemCount: number;
  title: string;
}

export interface SdkworkImageDigest {
  activeRenders: number;
  presetCount: number;
  readyImages: number;
  totalImages: number;
}

export interface SdkworkImageWorkspaceData {
  digest: SdkworkImageDigest;
  images: SdkworkImageAsset[];
  isAuthenticated: boolean;
  presets: SdkworkImagePreset[];
}

export interface SdkworkImageRouteIntent {
  focusWindow: boolean;
  imageId?: string;
  presetId?: string;
  route: string;
  source: "image-workspace";
  type: "image-route-intent";
}

export interface CreateImageRouteIntentOptions {
  basePath?: string;
  focusWindow?: boolean;
  imageId?: string;
  presetId?: string;
}

export interface SdkworkImageWorkspaceManifest {
  capability: "image";
  description: string;
  host?: string;
  id: string;
  packageNames: string[];
  routePath: string;
  theme?: string;
  title: string;
}

export interface CreateImageWorkspaceManifestOptions extends Partial<Omit<SdkworkImageWorkspaceManifest, "capability" | "routePath">> {
  routePath?: string;
}

export interface CreateEmptySdkworkImageWorkspaceOptions {
  images?: readonly SdkworkImageAsset[];
  isAuthenticated?: boolean;
  presets?: readonly SdkworkImagePreset[];
}

export const imagePackageMeta = {
  architecture: "pc-react",
  domain: "content",
  package: "@sdkwork/image-pc-react",
  status: "ready",
} as const;

function normalizeBasePath(basePath: string | undefined): string {
  const normalized = (basePath ?? "/image").trim();
  if (!normalized || normalized === "/") {
    return "/image";
  }

  return normalized.endsWith("/") ? normalized.slice(0, -1) : normalized;
}

function toTimestamp(value: string): number {
  const timestamp = Date.parse(value);
  return Number.isFinite(timestamp) ? timestamp : 0;
}

function sortSdkworkImages(images: readonly SdkworkImageAsset[]): SdkworkImageAsset[] {
  return [...images].sort(
    (left, right) => toTimestamp(right.updatedAt) - toTimestamp(left.updatedAt) || left.title.localeCompare(right.title),
  );
}

function createGeneratedImageResource(
  id: string,
  title: string,
  prompt: string,
  resolution: string,
  style: string,
): SdkworkMediaResource {
  const [width, height] = resolution.split("x").map((part) => Number.parseInt(part, 10));
  return {
    ai: {
      generationTaskId: id,
      provenance: "generated",
      moderationStatus: "unknown",
    },
    altText: prompt,
    id: `node-ai-generated-${id}`,
    kind: "image",
    metadata: {
      scene: style,
      style,
      drive: {
        nodeId: `node-ai-generated-${id}`,
        spaceId: "space-ai-generated-user-demo",
        spaceType: "ai_generated",
      },
    },
    source: "drive",
    title,
    uri: `drive://spaces/space-ai-generated-user-demo/nodes/node-ai-generated-${id}`,
    ...(Number.isFinite(width) ? { width } : {}),
    ...(Number.isFinite(height) ? { height } : {}),
  };
}

export function createDefaultSdkworkImagePresets(): SdkworkImagePreset[] {
  return [
    { id: "studio-product", itemCount: 2, title: "Studio Product" },
    { id: "brand-illustration", itemCount: 1, title: "Brand Illustration" },
    { id: "launch-key-visual", itemCount: 1, title: "Launch Key Visual" },
  ];
}

export function createDefaultSdkworkImages(): SdkworkImageAsset[] {
  const images = [
    { id: "image-device-beauty", presetId: "studio-product", prompt: "Premium device beauty shot", resolution: "2048x2048", status: "ready", style: "studio", title: "Device Beauty", updatedAt: "2026-04-03T04:20:00.000Z" },
    { id: "image-desktop-closeup", presetId: "studio-product", prompt: "Operator desk closeup", resolution: "1792x1024", status: "rendering", style: "editorial", title: "Desktop Closeup", updatedAt: "2026-04-02T04:20:00.000Z" },
    { id: "image-brand-scene", presetId: "brand-illustration", prompt: "Brand scene illustration", resolution: "1536x1536", status: "ready", style: "illustration", title: "Brand Scene", updatedAt: "2026-04-01T04:20:00.000Z" },
    { id: "image-launch-poster", presetId: "launch-key-visual", prompt: "Launch campaign poster", resolution: "1024x1536", status: "queued", style: "campaign", title: "Launch Poster", updatedAt: "2026-03-31T04:20:00.000Z" },
  ] as const;

  return images.map((image) => ({
    ...image,
    resource: createGeneratedImageResource(image.id, image.title, image.prompt, image.resolution, image.style),
  }));
}

export function summarizeSdkworkImageWorkspace(
  images: readonly SdkworkImageAsset[],
  presets: readonly SdkworkImagePreset[],
): SdkworkImageDigest {
  return {
    activeRenders: images.filter((image) => image.status === "rendering" || image.status === "queued").length,
    presetCount: presets.length,
    readyImages: images.filter((image) => image.status === "ready").length,
    totalImages: images.length,
  };
}

export function createImageWorkspaceManifest({
  description = "Image workspace for generation presets, prompt tracking, and gallery-ready result filtering.",
  host,
  id = "sdkwork-image",
  packageNames = [
    "@sdkwork/image-pc-react",
    "@sdkwork/media-pc-react",
  ],
  routePath = "/image",
  theme,
  title = "Image Workspace",
}: CreateImageWorkspaceManifestOptions = {}): SdkworkImageWorkspaceManifest {
  return {
    capability: "image",
    description,
    ...(host ? { host } : {}),
    id,
    packageNames: [...packageNames],
    routePath: normalizeBasePath(routePath),
    ...(theme ? { theme } : {}),
    title,
  };
}

export function createImageRouteIntent(options: CreateImageRouteIntentOptions = {}): SdkworkImageRouteIntent {
  const basePath = normalizeBasePath(options.basePath);
  const params = new URLSearchParams();

  if (options.presetId) {
    params.set("presetId", options.presetId);
  }
  if (options.imageId) {
    params.set("imageId", options.imageId);
  }

  return {
    focusWindow: options.focusWindow !== false,
    ...(options.imageId ? { imageId: options.imageId } : {}),
    ...(options.presetId ? { presetId: options.presetId } : {}),
    route: params.toString() ? `${basePath}?${params.toString()}` : basePath,
    source: "image-workspace",
    type: "image-route-intent",
  };
}

export function createEmptySdkworkImageWorkspace(
  options: CreateEmptySdkworkImageWorkspaceOptions = {},
): SdkworkImageWorkspaceData {
  const presets = options.presets?.length ? [...options.presets] : createDefaultSdkworkImagePresets();
  const images = sortSdkworkImages(options.images?.length ? options.images : createDefaultSdkworkImages());

  return {
    digest: summarizeSdkworkImageWorkspace(images, presets),
    images,
    isAuthenticated: Boolean(options.isAuthenticated),
    presets,
  };
}
