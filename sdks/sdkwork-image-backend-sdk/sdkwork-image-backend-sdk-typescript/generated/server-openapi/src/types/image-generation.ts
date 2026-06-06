import type { DriveSyncStatus } from './drive-sync-status';
import type { ImageGenerationOutput } from './image-generation-output';
import type { ImageGenerationStatus } from './image-generation-status';

export interface ImageGeneration {
  generationId: string;
  status: ImageGenerationStatus;
  scene: string;
  providerCode?: string | null;
  providerTaskId?: string | null;
  providerStatus?: string | null;
  driveSpaceId?: string | null;
  driveSyncStatus?: DriveSyncStatus;
  outputAssetCount?: number;
  outputs: ImageGenerationOutput[];
  metadata?: Record<string, unknown>;
}
