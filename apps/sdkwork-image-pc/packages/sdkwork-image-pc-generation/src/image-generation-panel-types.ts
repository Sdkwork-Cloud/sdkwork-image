import type { MediaResourceLike } from '@sdkwork/assets-pc-commons';
import type {
  SdkworkGenerationModelBuckets,
  SdkworkGenerationSerializedAssetConfig,
} from './generation-asset-config';
import type { ImageGenerationModelOption, ImageReferenceMode } from './image-reference-capability';

export interface ImageGenerationReferenceImageInput {
  name: string;
  mimeType?: string;
  resource: MediaResourceLike;
  sizeBytes?: number;
}

export interface ImageGenerationModelGroup extends SdkworkGenerationModelBuckets<ImageGenerationModelOption> {
  id: string;
}

export interface ImageGenerationSubmitInput {
  prompt: string;
  selectedModality: 'image' | 'agent';
  targetType?: 'image';
  selectedModel?: string;
  generationConfig?: SdkworkGenerationSerializedAssetConfig;
  referenceImages?: ImageGenerationReferenceImageInput[];
  referenceMode?: ImageReferenceMode;
}

export interface ImageGenerationPanelProps {
  placeholderKey: string;
  modelGroups: ImageGenerationModelGroup[];
  selectedModelId: string;
  onSubmitGeneration: (input: ImageGenerationSubmitInput) => Promise<void>;
  submitting: boolean;
  submitError: string | null;
}
