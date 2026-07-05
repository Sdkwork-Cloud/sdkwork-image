export type {
  MediaAccess as SdkworkMediaAccess,
  MediaAccessVisibility,
  MediaAiProvenanceKind as SdkworkMediaAiProvenance,
  MediaChecksum as SdkworkMediaChecksum,
  MediaKind as SdkworkMediaKind,
  MediaModerationStatus,
  MediaResource as SdkworkMediaResource,
  MediaSource as SdkworkMediaSource,
} from '@sdkwork/assets-core';

export {
  readMediaResourceThumb,
  readMediaResourceUrl,
} from '@sdkwork/assets-core';

import type { MediaResource } from '@sdkwork/assets-core';
import { readMediaResourceUrl } from '@sdkwork/assets-core';

/** Delivery URL helper retained for image PC consumers. */
export function getSdkworkMediaDeliveryUrl(
  resource: Pick<MediaResource, 'publicUrl' | 'url' | 'uri' | 'id'> | null | undefined,
): string | undefined {
  const url = readMediaResourceUrl(resource);
  return url || undefined;
}
