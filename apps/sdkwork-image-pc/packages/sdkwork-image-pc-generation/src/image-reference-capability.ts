import type { SdkworkGenerationPricedModel } from './generation-asset-config';

export type ImageReferenceMode = 'text_to_image' | 'image_to_image' | 'multi_reference';

export interface ImageGenerationModelOption extends SdkworkGenerationPricedModel {
  id: string;
  catalogKey: string;
  model: string;
  name: string;
  displayName: string;
  capabilities: string[];
  inputModalities: string[];
  outputModalities: string[];
  apiFormat?: string;
}

export interface ImageReferenceCapability {
  enabled: boolean;
  maxImages: number;
  supportedModes: ImageReferenceMode[];
}

export interface ImageReferenceModeUpload {
  maxFiles: number;
}

export const IMAGE_REFERENCE_MODE_ORDER: readonly ImageReferenceMode[] = [
  'text_to_image',
  'image_to_image',
  'multi_reference',
];

export function resolveImageReferenceCapability(
  model: ImageGenerationModelOption | null | undefined,
): ImageReferenceCapability {
  if (!model) {
    return createTextOnlyImageReferenceCapability();
  }

  const capabilityTokens = createReferenceImageCapabilityTokenSet(model.capabilities);
  const inputTokens = createReferenceImageCapabilityTokenSet(model.inputModalities);
  const outputTokens = createReferenceImageCapabilityTokenSet(model.outputModalities);
  const descriptorTokens = createReferenceImageCapabilityTokenSet([
    model.apiFormat,
    model.catalogKey,
    model.displayName,
    model.id,
    model.model,
    model.name,
  ]);
  const allTokens = new Set([
    ...capabilityTokens,
    ...inputTokens,
    ...outputTokens,
    ...descriptorTokens,
  ]);

  const canOutputImage = outputTokens.has('image')
    || hasAnyReferenceImageToken(capabilityTokens, IMAGE_OUTPUT_CAPABILITY_TOKENS);
  if (!canOutputImage) {
    return createTextOnlyImageReferenceCapability();
  }

  const supportsSingleReference = inputTokens.has('image')
    || hasAnyReferenceImageToken(capabilityTokens, SINGLE_REFERENCE_IMAGE_CAPABILITY_TOKENS);
  const supportsMultiReference = supportsSingleReference && (
    hasAnyReferenceImageToken(allTokens, MULTI_REFERENCE_IMAGE_CAPABILITY_TOKENS)
    || hasKnownMultiReferenceImageModel(descriptorTokens)
  );

  const supportedModes: ImageReferenceMode[] = ['text_to_image'];
  if (supportsSingleReference) {
    supportedModes.push('image_to_image');
  }
  if (supportsMultiReference) {
    supportedModes.push('multi_reference');
  }

  const maxImages = supportsMultiReference
    ? 4
    : supportsSingleReference
      ? 1
      : 0;

  return {
    enabled: maxImages > 0,
    maxImages,
    supportedModes,
  };
}

export function resolveImageReferenceModeUpload(
  capability: ImageReferenceCapability,
  mode: ImageReferenceMode,
): ImageReferenceModeUpload {
  switch (mode) {
    case 'text_to_image':
      return { maxFiles: 0 };
    case 'image_to_image':
      return { maxFiles: capability.maxImages > 0 ? 1 : 0 };
    case 'multi_reference':
      return { maxFiles: capability.maxImages };
    default:
      return { maxFiles: 0 };
  }
}

function createTextOnlyImageReferenceCapability(): ImageReferenceCapability {
  return {
    enabled: false,
    maxImages: 0,
    supportedModes: ['text_to_image'],
  };
}

function createReferenceImageCapabilityTokenSet(values: readonly (string | null | undefined)[]): Set<string> {
  return new Set(values.flatMap((value) => normalizeReferenceImageCapabilityTokens(value)));
}

function normalizeReferenceImageCapabilityTokens(value: string | null | undefined): string[] {
  const normalized = value
    ?.trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '_')
    .replace(/^_+|_+$/g, '');
  if (!normalized) {
    return [];
  }
  return [normalized, ...normalized.split('_').filter(Boolean)];
}

function hasAnyReferenceImageToken(tokens: ReadonlySet<string>, expectedTokens: ReadonlySet<string>): boolean {
  return Array.from(expectedTokens).some((expectedToken) => tokens.has(expectedToken));
}

function hasKnownMultiReferenceImageModel(tokens: ReadonlySet<string>): boolean {
  return Array.from(tokens).some((token) => (
    token.startsWith('gpt_image_')
    || token.startsWith('gemini_')
    || token.startsWith('doubao_seedream_')
    || token.startsWith('seedream_')
    || token.startsWith('flux_kontext_')
    || token.startsWith('flux_kontext')
    || token.startsWith('black_forest_labs_flux_kontext')
  ));
}

const IMAGE_OUTPUT_CAPABILITY_TOKENS = new Set([
  'image',
  'image_generation',
  'image_edit',
  'image_editing',
  'image_to_image',
]);

const SINGLE_REFERENCE_IMAGE_CAPABILITY_TOKENS = new Set([
  'image_edit',
  'image_editing',
  'image_reference',
  'image_to_image',
  'image_variation',
  'reference_image',
  'vision',
]);

const MULTI_REFERENCE_IMAGE_CAPABILITY_TOKENS = new Set([
  'image_edit_multi',
  'image_reference_multi',
  'multi_image',
  'multi_image_reference',
  'multi_reference_image',
  'multiple_image_reference',
]);
