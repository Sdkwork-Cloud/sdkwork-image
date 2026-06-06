import type { MediaAiProvenance } from './media-ai-provenance';
import type { MediaKind } from './media-kind';
import type { MediaSource } from './media-source';

export interface MediaResource {
  id?: string;
  kind: MediaKind;
  source: MediaSource;
  url?: string;
  publicUrl?: string;
  uri?: string | null;
  objectBlobId?: string | null;
  fileName?: string | null;
  mimeType?: string | null;
  sizeBytes?: string | null;
  width?: number | null;
  height?: number | null;
  durationSeconds?: number | null;
  ai?: MediaAiProvenance;
  metadata?: Record<string, unknown>;
}
