export type SdkworkMediaKind = "audio" | "file" | "image" | "text" | "video";
export type SdkworkMediaSource =
  | "data_url"
  | "external_url"
  | "generated"
  | "object_storage"
  | "provider_asset"
  | "remote"
  | "upload";
export type SdkworkMediaAiProvenance = "edited" | "generated" | "human" | "unknown";

export interface SdkworkMediaAccess {
  expiresAt?: string;
  policy?: string;
  visibility?: "private" | "public" | "tenant";
}

export interface SdkworkMediaChecksum {
  algorithm: "md5" | "sha1" | "sha256";
  value: string;
}

export interface SdkworkMediaResource {
  access?: SdkworkMediaAccess;
  ai?: {
    model?: string;
    provenance: SdkworkMediaAiProvenance;
    promptHash?: string;
  };
  altText?: string;
  checksum?: SdkworkMediaChecksum;
  deliveryUrl?: string;
  durationSeconds?: number;
  fileSizeBytes?: number;
  fileName?: string;
  height?: number;
  id?: string;
  kind: SdkworkMediaKind;
  metadata?: Record<string, unknown>;
  mimeType?: string;
  objectKey?: string;
  objectBlobId?: string;
  objectVersion?: string;
  poster?: SdkworkMediaResource;
  publicUrl?: string;
  source: SdkworkMediaSource;
  thumbnails?: SdkworkMediaResource[];
  title?: string;
  uri?: string;
  url?: string;
  variants?: SdkworkMediaResource[];
  width?: number;
}

export function getSdkworkMediaDeliveryUrl(
  resource: Pick<SdkworkMediaResource, "deliveryUrl" | "publicUrl" | "url"> | null | undefined,
): string | undefined {
  return normalizeOptionalText(resource?.deliveryUrl)
    || normalizeOptionalText(resource?.publicUrl)
    || normalizeOptionalText(resource?.url);
}

function normalizeOptionalText(value: unknown): string | undefined {
  const normalized = typeof value === "string" ? value.trim() : "";
  return normalized || undefined;
}
