import { describe, expect, it } from "vitest";
import {
  createDefaultSdkworkGenerationAssetConfig,
  createSdkworkGenerationAssetConfigFromSerialized,
  serializeSdkworkGenerationAssetConfig,
  updateSdkworkGenerationImageModeConfig,
} from "../src/generation-asset-config.ts";

describe("sdkwork-generation-pc-react asset config", () => {
  it("creates modality-specific defaults that are already reconciled", () => {
    expect(createDefaultSdkworkGenerationAssetConfig("image")).toEqual({
      aspectRatio: "1:1",
      durationSeconds: 1,
      imageCount: 2,
      imageMode: {
        aspectRatio: "auto",
        count: 2,
        quality: "1k",
      },
      quality: "standard",
    });
  });

  it("serializes full image mode config while keeping history-safe summary fields", () => {
    const baseConfig = createDefaultSdkworkGenerationAssetConfig("image");
    const config = updateSdkworkGenerationImageModeConfig(baseConfig, {
      aspectRatio: "21:9",
      count: 4,
      quality: "2k",
    });

    expect(serializeSdkworkGenerationAssetConfig(config, "image")).toEqual({
      aspectRatio: "1:1",
      durationSeconds: 1,
      imageCount: 4,
      imageMode: {
        aspectRatio: "21:9",
        count: 4,
        quality: "2k",
      },
      quality: "high",
    });
  });

  it("restores image asset config from serialized mode config", () => {
    expect(createSdkworkGenerationAssetConfigFromSerialized({
      aspectRatio: "9:16",
      durationSeconds: 1,
      imageCount: 3,
      imageMode: {
        aspectRatio: "9:16",
        count: 3,
        quality: "2k",
      },
      quality: "high",
    }, "image")).toEqual({
      aspectRatio: "9:16",
      durationSeconds: 1,
      imageCount: 3,
      imageMode: {
        aspectRatio: "9:16",
        count: 3,
        quality: "2k",
      },
      quality: "high",
    });
  });

});
