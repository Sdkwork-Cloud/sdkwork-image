import type { MediaResourceLike } from '@sdkwork/assets-pc-commons';
import type { SdkworkGenerationSerializedAssetConfig } from './generation-asset-config';
import type { ImageGenerationModelOption, ImageReferenceMode } from './image-reference-capability';

export interface ImageGenerationReferenceImageInput {
  name: string;
  mimeType?: string;
  resource: MediaResourceLike;
  sizeBytes?: number;
}

export interface ImageGenerationModelGroup {
  id: string;
  llms: ImageGenerationModelOption[];
  images: ImageGenerationModelOption[];
  videos: ImageGenerationModelOption[];
  audios: ImageGenerationModelOption[];
  music: ImageGenerationModelOption[];
  sfx: ImageGenerationModelOption[];
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
