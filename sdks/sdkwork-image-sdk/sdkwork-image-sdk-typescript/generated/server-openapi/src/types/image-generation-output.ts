import type { DriveSyncStatus } from './drive-sync-status';
import type { MediaKind } from './media-kind';
import type { MediaResource } from './media-resource';

export interface ImageGenerationOutput {
  outputIndex: number;
  mediaKind: MediaKind;
  scene: string;
  providerCode?: string;
  providerAssetId?: string | null;
  providerUri?: string | null;
  driveSpaceId?: string | null;
  driveNodeId?: string | null;
  driveUri?: string | null;
  objectBlobId?: string | null;
  syncStatus: DriveSyncStatus;
  resource?: MediaResource;
  errorCode?: string | null;
  errorMessage?: string | null;
}
