export type SdkworkMediaKind =
  | "archive"
  | "audio"
  | "document"
  | "image"
  | "model"
  | "other"
  | "video"
  | "voice";
export type SdkworkMediaSource =
  | "data_url"
  | "drive"
  | "external_url"
  | "generated"
  | "provider_asset";
export type SdkworkMediaAiProvenance = "edited" | "generated" | "imported" | "uploaded";

export interface SdkworkMediaAccess {
  expiresAt?: string;
  visibility?: "organization" | "private" | "public" | "signed" | "tenant";
}

export interface SdkworkMediaChecksum {
  algorithm: "md5" | "sha1" | "sha256";
  value: string;
}

export interface SdkworkMediaResource {
  access?: SdkworkMediaAccess;
  ai?: {
    generationTaskId?: string;
    model?: string;
    moderationStatus?: "approved" | "blocked" | "pending" | "rejected" | "unknown";
    provenance: SdkworkMediaAiProvenance;
    provider?: string;
    promptId?: string;
    safetyLabels?: string[];
    seed?: string;
    sourceMediaIds?: string[];
  };
  altText?: string;
  checksum?: SdkworkMediaChecksum;
  durationSeconds?: number;
  fileName?: string;
  height?: number;
  id?: string;
  kind: SdkworkMediaKind;
  metadata?: Record<string, unknown>;
  mimeType?: string;
  objectBlobId?: string;
  poster?: SdkworkMediaResource;
  publicUrl?: string;
  sizeBytes?: string;
  source: SdkworkMediaSource;
  thumbnails?: SdkworkMediaResource[];
  title?: string;
  uri?: string;
  url?: string;
  variants?: SdkworkMediaResource[];
  width?: number;
}

export function getSdkworkMediaDeliveryUrl(
  resource: Pick<SdkworkMediaResource, "publicUrl" | "url"> | null | undefined,
): string | undefined {
  return normalizeOptionalText(resource?.publicUrl)
    || normalizeOptionalText(resource?.url);
}

function normalizeOptionalText(value: unknown): string | undefined {
  const normalized = typeof value === "string" ? value.trim() : "";
  return normalized || undefined;
}
