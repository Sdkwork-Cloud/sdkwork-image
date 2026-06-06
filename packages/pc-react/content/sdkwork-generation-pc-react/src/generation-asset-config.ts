export type SdkworkGenerationAssetModality = "image";
export type SdkworkGenerationAssetAspectRatio = "1:1" | "16:9" | "9:16";
export type SdkworkGenerationAssetQuality = "high" | "standard";

export interface SdkworkGenerationImageModeConfig {
  aspectRatio: "auto" | "1:1" | "16:9" | "21:9" | "2:3" | "3:2" | "3:4" | "4:3" | "9:16";
  count: number;
  quality: "1k" | "2k";
}

export interface SdkworkGenerationAssetConfig {
  aspectRatio: SdkworkGenerationAssetAspectRatio;
  durationSeconds: number;
  imageCount: number;
  imageMode?: SdkworkGenerationImageModeConfig;
  quality: SdkworkGenerationAssetQuality;
}

export interface SdkworkGenerationSerializedAssetConfig {
  aspectRatio?: SdkworkGenerationAssetAspectRatio;
  durationSeconds?: number;
  imageCount?: number;
  imageMode?: SdkworkGenerationImageModeConfig;
  quality?: SdkworkGenerationAssetQuality;
}

export type SdkworkGenerationModelBucket = "llms" | "images";

export interface SdkworkGenerationReferencePrice {
  currency: string;
  unitPrice: string;
  usageMeter: string;
}

export interface SdkworkGenerationPriceAvailability {
  status: "reference" | "unavailable";
  reason?: string | null;
}

export interface SdkworkGenerationPricedModel {
  officialReferenceCurrency?: string | null;
  officialReferencePrices: readonly SdkworkGenerationReferencePrice[];
  officialReferenceUnitPrice?: string | null;
  priceAvailability: SdkworkGenerationPriceAvailability;
}

export type SdkworkGenerationModelBuckets<TModel> = {
  [Bucket in SdkworkGenerationModelBucket]: readonly TModel[];
};

export interface SdkworkGenerationCreditEstimate {
  points: number | null;
  detail: string;
  reference: boolean;
}

export interface EstimateSdkworkGenerationCreditsInput<TModel extends SdkworkGenerationPricedModel> {
  config: SdkworkGenerationAssetConfig;
  modality: SdkworkGenerationAssetModality;
  model: TModel | null | undefined;
  pointsPerUsd?: number;
  unavailableDetail?: string;
}

const DEFAULT_SDKWORK_GENERATION_POINTS_PER_USD = 10;
const DEFAULT_SDKWORK_GENERATION_COST_UNAVAILABLE_DETAIL = "sdkwork.generation.cost.unavailable";

export const DEFAULT_SDKWORK_GENERATION_IMAGE_MODE_CONFIG: SdkworkGenerationImageModeConfig = {
  aspectRatio: "auto",
  count: 2,
  quality: "1k",
};

export function getDefaultSdkworkGenerationDurationSeconds(
  modality: SdkworkGenerationAssetModality,
): number {
  switch (modality) {
    case "image":
      return 1;
  }
}

export function createDefaultSdkworkGenerationAssetConfig(
  modality: SdkworkGenerationAssetModality,
): SdkworkGenerationAssetConfig {
  const imageMode = { ...DEFAULT_SDKWORK_GENERATION_IMAGE_MODE_CONFIG };
  return {
    aspectRatio: normalizeImageAspectRatio(imageMode.aspectRatio, "1:1"),
    durationSeconds: getDefaultSdkworkGenerationDurationSeconds(modality),
    imageCount: imageMode.count,
    imageMode,
    quality: imageMode.quality === "2k" ? "high" : "standard",
  };
}

export function reconcileSdkworkGenerationAssetConfig(
  config: SdkworkGenerationAssetConfig,
  modality: SdkworkGenerationAssetModality,
): SdkworkGenerationAssetConfig {
  const defaultConfig = createDefaultSdkworkGenerationAssetConfig(modality);
  const next = {
    ...defaultConfig,
    ...config,
    durationSeconds: config.durationSeconds || defaultConfig.durationSeconds,
  };
  const imageMode = next.imageMode ?? { ...DEFAULT_SDKWORK_GENERATION_IMAGE_MODE_CONFIG };

  return {
    ...next,
    aspectRatio: normalizeImageAspectRatio(imageMode.aspectRatio, next.aspectRatio),
    imageCount: imageMode.count,
    imageMode,
    quality: imageMode.quality === "2k" ? "high" : "standard",
  };
}

export function serializeSdkworkGenerationAssetConfig(
  config: SdkworkGenerationAssetConfig,
  modality: SdkworkGenerationAssetModality,
): SdkworkGenerationSerializedAssetConfig {
  const reconciled = reconcileSdkworkGenerationAssetConfig(config, modality);
  return {
    aspectRatio: reconciled.aspectRatio,
    durationSeconds: reconciled.durationSeconds,
    imageCount: reconciled.imageCount,
    imageMode: reconciled.imageMode,
    quality: reconciled.quality,
  };
}

export function createSdkworkGenerationAssetConfigFromSerialized(
  serialized: SdkworkGenerationSerializedAssetConfig | undefined,
  modality: SdkworkGenerationAssetModality,
): SdkworkGenerationAssetConfig {
  const defaultConfig = createDefaultSdkworkGenerationAssetConfig(modality);
  if (!serialized) {
    return defaultConfig;
  }

  const imageMode = serialized.imageMode
    ? { ...serialized.imageMode }
    : {
      ...DEFAULT_SDKWORK_GENERATION_IMAGE_MODE_CONFIG,
      aspectRatio: serialized.aspectRatio ?? DEFAULT_SDKWORK_GENERATION_IMAGE_MODE_CONFIG.aspectRatio,
      count: serialized.imageCount ?? DEFAULT_SDKWORK_GENERATION_IMAGE_MODE_CONFIG.count,
      quality: serialized.quality === "high" ? "2k" : DEFAULT_SDKWORK_GENERATION_IMAGE_MODE_CONFIG.quality,
    };

  return reconcileSdkworkGenerationAssetConfig({
    ...defaultConfig,
    aspectRatio: serialized.aspectRatio ?? defaultConfig.aspectRatio,
    durationSeconds: serialized.durationSeconds ?? defaultConfig.durationSeconds,
    imageCount: serialized.imageCount ?? imageMode.count,
    imageMode,
    quality: serialized.quality ?? (imageMode.quality === "2k" ? "high" : "standard"),
  }, modality);
}

export function updateSdkworkGenerationImageModeConfig(
  config: SdkworkGenerationAssetConfig,
  imageMode: SdkworkGenerationImageModeConfig,
): SdkworkGenerationAssetConfig {
  return reconcileSdkworkGenerationAssetConfig({
    ...config,
    imageMode,
  }, "image");
}

export function getSdkworkGenerationModelBucket(
  modality: SdkworkGenerationAssetModality,
): Exclude<SdkworkGenerationModelBucket, "llms"> {
  switch (modality) {
    case "image":
      return "images";
  }
}

export function findSdkworkGenerationModelById<TModel extends { id: string }>(
  groups: readonly SdkworkGenerationModelBuckets<TModel>[],
  modelId: string,
): TModel | null {
  for (const group of groups) {
    for (const bucket of ["llms", "images"] as const) {
      const model = group[bucket].find((item) => item.id === modelId);
      if (model) {
        return model;
      }
    }
  }
  return null;
}

export function findFirstSdkworkGenerationModelForModality<TModel>(
  groups: readonly SdkworkGenerationModelBuckets<TModel>[],
  modality: SdkworkGenerationAssetModality,
): TModel | null {
  const bucket = getSdkworkGenerationModelBucket(modality);
  for (const group of groups) {
    const model = group[bucket][0];
    if (model) {
      return model;
    }
  }
  return null;
}

export function getSdkworkGenerationDurationOptions(
  modality: SdkworkGenerationAssetModality,
): number[] {
  switch (modality) {
    case "image":
      return [];
  }
}

export function estimateSdkworkGenerationCredits<TModel extends SdkworkGenerationPricedModel>({
  config,
  modality,
  model,
  pointsPerUsd = DEFAULT_SDKWORK_GENERATION_POINTS_PER_USD,
  unavailableDetail = DEFAULT_SDKWORK_GENERATION_COST_UNAVAILABLE_DETAIL,
}: EstimateSdkworkGenerationCreditsInput<TModel>): SdkworkGenerationCreditEstimate {
  if (!model || model.priceAvailability.status === "unavailable") {
    return createUnavailableSdkworkGenerationCreditEstimate(unavailableDetail);
  }

  const price = selectSdkworkGenerationReferencePrice(model.officialReferencePrices, modality)
    ?? createFallbackSdkworkGenerationReferencePrice(model);
  if (!price) {
    return createUnavailableSdkworkGenerationCreditEstimate(unavailableDetail);
  }

  const unitPrice = readPositiveSdkworkGenerationNumber(price.unitPrice);
  if (unitPrice === null) {
    return createUnavailableSdkworkGenerationCreditEstimate(unavailableDetail);
  }

  const quantity = estimateSdkworkGenerationMeterQuantity(price.usageMeter, config);
  const points = Math.ceil(unitPrice * quantity * pointsPerUsd);
  return {
    points,
    detail: describeSdkworkGenerationCreditEstimate(price, quantity),
    reference: model.priceAvailability.status === "reference",
  };
}

function normalizeImageAspectRatio(
  aspectRatio: SdkworkGenerationImageModeConfig["aspectRatio"],
  fallback: SdkworkGenerationAssetAspectRatio,
): SdkworkGenerationAssetAspectRatio {
  if (aspectRatio === "1:1" || aspectRatio === "16:9" || aspectRatio === "9:16") {
    return aspectRatio;
  }
  return fallback;
}

function createUnavailableSdkworkGenerationCreditEstimate(detail: string): SdkworkGenerationCreditEstimate {
  return {
    detail,
    points: null,
    reference: false,
  };
}

function selectSdkworkGenerationReferencePrice(
  prices: readonly SdkworkGenerationReferencePrice[],
  modality: SdkworkGenerationAssetModality,
): SdkworkGenerationReferencePrice | null {
  const meters = getSdkworkGenerationMetersForModality(modality);
  for (const meter of meters) {
    const price = prices.find((candidate) => candidate.usageMeter === meter);
    if (price) {
      return price;
    }
  }
  return prices[0] ?? null;
}

function createFallbackSdkworkGenerationReferencePrice(
  model: SdkworkGenerationPricedModel,
): SdkworkGenerationReferencePrice | null {
  if (!model.officialReferenceUnitPrice || readPositiveSdkworkGenerationNumber(model.officialReferenceUnitPrice) === null) {
    return null;
  }
  return {
    currency: model.officialReferenceCurrency || "USD",
    unitPrice: model.officialReferenceUnitPrice,
    usageMeter: "api_result",
  };
}

function getSdkworkGenerationMetersForModality(
  modality: SdkworkGenerationAssetModality,
): string[] {
  switch (modality) {
    case "image":
      return ["image_result", "image_megapixel", "image_pixel", "image_output_token", "api_result"];
  }
}

function estimateSdkworkGenerationMeterQuantity(
  usageMeter: string,
  config: SdkworkGenerationAssetConfig,
): number {
  const qualityMultiplier = config.quality === "high" ? 1.5 : 1;
  if (usageMeter === "image_result") {
    return config.imageCount * qualityMultiplier;
  }
  if (usageMeter === "image_megapixel") {
    return config.imageCount * estimateSdkworkGenerationImagePixels(config.aspectRatio) / 1_000_000 * qualityMultiplier;
  }
  if (usageMeter === "image_pixel") {
    return config.imageCount * estimateSdkworkGenerationImagePixels(config.aspectRatio) * qualityMultiplier;
  }
  return 1;
}

function estimateSdkworkGenerationImagePixels(
  aspectRatio: SdkworkGenerationAssetAspectRatio,
): number {
  switch (aspectRatio) {
    case "16:9":
    case "9:16":
      return 1792 * 1024;
    case "1:1":
    default:
      return 1024 * 1024;
  }
}

function describeSdkworkGenerationCreditEstimate(
  price: SdkworkGenerationReferencePrice,
  quantity: number,
): string {
  return `${price.currency} ${formatSdkworkGenerationDecimal(price.unitPrice)} x ${formatSdkworkGenerationDecimal(quantity.toString())} ${getSdkworkGenerationUnitLabelForMeter(price.usageMeter)}`;
}

function getSdkworkGenerationUnitLabelForMeter(usageMeter: string): string {
  if (usageMeter === "image_result") {
    return "image";
  }
  if (usageMeter === "image_megapixel") {
    return "MP";
  }
  if (usageMeter === "image_pixel") {
    return "px";
  }
  return "unit";
}

function readPositiveSdkworkGenerationNumber(value: string | null | undefined): number | null {
  if (!value) {
    return null;
  }
  const number = Number(value);
  if (!Number.isFinite(number) || number < 0) {
    return null;
  }
  return number;
}

function formatSdkworkGenerationDecimal(value: string): string {
  const number = Number(value);
  if (!Number.isFinite(number)) {
    return value;
  }
  return number.toLocaleString("en-US", {
    maximumFractionDigits: 6,
  });
}
