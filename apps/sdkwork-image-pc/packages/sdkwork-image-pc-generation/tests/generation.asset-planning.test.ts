import { describe, expect, it } from "vitest";
import {
  estimateSdkworkGenerationCredits,
  findFirstSdkworkGenerationModelForModality,
  findSdkworkGenerationModelById,
  getSdkworkGenerationDurationOptions,
  getSdkworkGenerationModelBucket,
  updateSdkworkGenerationImageModeConfig,
  type SdkworkGenerationAssetConfig,
  type SdkworkGenerationModelBuckets,
  type SdkworkGenerationPricedModel,
} from "../src/react.ts";

interface TestModel extends SdkworkGenerationPricedModel {
  id: string;
  name: string;
}

function createModel(input: Partial<TestModel> & Pick<TestModel, "id">): TestModel {
  return {
    id: input.id,
    name: input.name ?? input.id,
    officialReferenceCurrency: input.officialReferenceCurrency ?? "USD",
    officialReferencePrices: input.officialReferencePrices ?? [],
    officialReferenceUnitPrice: input.officialReferenceUnitPrice ?? null,
    priceAvailability: input.priceAvailability ?? { status: "reference" },
  };
}

describe("sdkwork-generation-pc-react asset planning", () => {
  it("maps asset modalities to reusable model buckets and duration options", () => {
    expect(getSdkworkGenerationModelBucket("image")).toBe("images");

    expect(getSdkworkGenerationDurationOptions("image")).toEqual([]);
  });

  it("selects models across reusable generation model buckets", () => {
    const imageModel = createModel({ id: "image-1" });
    const groups: SdkworkGenerationModelBuckets<TestModel>[] = [{
      audios: [],
      images: [imageModel],
      llms: [],
      music: [],
      sfx: [],
      videos: [],
    }];

    expect(findSdkworkGenerationModelById(groups, "image-1")).toBe(imageModel);
    expect(findSdkworkGenerationModelById(groups, "missing")).toBeNull();
    expect(findFirstSdkworkGenerationModelForModality(groups, "image")).toBe(imageModel);
  });

  it("uses image count, pixels, and quality when estimating image metered credits", () => {
    const model = createModel({
      id: "image-priced",
      officialReferencePrices: [
        { currency: "USD", unitPrice: "0.01", usageMeter: "image_megapixel" },
      ],
    });
    const config = updateSdkworkGenerationImageModeConfig({
      aspectRatio: "1:1",
      durationSeconds: 1,
      imageCount: 1,
      quality: "standard",
    }, {
      aspectRatio: "16:9",
      count: 2,
      quality: "2k",
    });

    expect(estimateSdkworkGenerationCredits({
      config,
      modality: "image",
      model,
    })).toEqual({
      detail: "USD 0.01 x 5.505024 MP",
      points: 1,
      reference: true,
    });
  });

  it("falls back to legacy unit price and reports unavailable estimates without a usable price", () => {
    const config: SdkworkGenerationAssetConfig = {
      aspectRatio: "16:9",
      durationSeconds: 10,
      imageCount: 1,
      quality: "standard",
    };

    expect(estimateSdkworkGenerationCredits({
      config,
      modality: "image",
      model: createModel({
        id: "unpriced",
        priceAvailability: { status: "unavailable" },
      }),
    })).toEqual({
      detail: "sdkwork.generation.cost.unavailable",
      points: null,
      reference: false,
    });
  });
});
