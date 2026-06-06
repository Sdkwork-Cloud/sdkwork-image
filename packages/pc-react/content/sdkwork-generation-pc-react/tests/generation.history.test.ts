import { describe, expect, it } from "vitest";
import {
  appendSdkworkGenerationArtifactToHistoryItem,
  createSdkworkGenerationPendingHistoryItem,
  getSdkworkGenerationPreviewKind,
  mapSdkworkGenerationArtifactsToHistoryMedia,
  mapSdkworkGenerationModalityToHistoryType,
  mapSdkworkGenerationHistoryTypeToModality,
  readSdkworkGenerationMediaUrl,
  restoreSdkworkGenerationSerializedConfigFromHistoryItem,
  type SdkworkGenerationArtifact,
  type SdkworkGenerationHistoryItem,
  type SdkworkGenerationMediaResource,
} from "../src/react.ts";

describe("sdkwork-generation-pc-react history helpers", () => {
  it("maps history item types and preview kinds across text and asset modalities", () => {
    expect(mapSdkworkGenerationModalityToHistoryType(undefined)).toBe("text");
    expect(mapSdkworkGenerationModalityToHistoryType("image")).toBe("images");

    expect(mapSdkworkGenerationHistoryTypeToModality("text")).toBeUndefined();
    expect(mapSdkworkGenerationHistoryTypeToModality("image")).toBe("image");
    expect(mapSdkworkGenerationHistoryTypeToModality("images")).toBe("image");

    expect(getSdkworkGenerationPreviewKind("text")).toBe("text");
    expect(getSdkworkGenerationPreviewKind("images")).toBe("image");
  });

  it("creates pending history items with serialized generation config summaries", () => {
    expect(createSdkworkGenerationPendingHistoryItem({
      createdAt: "2026-05-22T00:00:00Z",
      generationConfig: {
        aspectRatio: "1:1",
        imageCount: 2,
      },
      id: "pending-1",
      prompt: "Create product images",
      selectedModel: "image-model",
      targetType: "image",
    })).toEqual({
      aspectRatio: "1:1",
      createdAt: "2026-05-22T00:00:00Z",
      date: "2026-05-22",
      durationSeconds: undefined,
      generationConfig: {
        aspectRatio: "1:1",
        imageCount: 2,
      },
      id: "pending-1",
      images: [],
      modelCatalogKey: "image-model",
      modelInfo: "image-model",
      outputText: "",
      prompt: "Create product images",
      status: "processing",
      type: "images",
      updatedAt: "2026-05-22T00:00:00Z",
    });
  });

  it("restores serialized asset config from full config or history-safe summary fields", () => {
    const imageHistory: SdkworkGenerationHistoryItem = {
      date: "2026-05-22",
      id: "image-history",
      images: [
        mediaResource("image", "https://cdn.example/one.png"),
        mediaResource("image", "https://cdn.example/two.png"),
      ],
      prompt: "Two square images",
      type: "images",
    };
    expect(restoreSdkworkGenerationSerializedConfigFromHistoryItem(imageHistory)).toEqual({
      aspectRatio: "1:1",
      durationSeconds: 1,
      imageCount: 2,
      imageMode: {
        aspectRatio: "1:1",
        count: 2,
        quality: "1k",
      },
      quality: "standard",
    });
  });

  it("maps artifacts into history media collections for the requested modality", () => {
    const artifacts: SdkworkGenerationArtifact[] = [
      { modality: "image", asset: mediaResource("image", "https://cdn.example/a.png") },
    ];

    expect(mapSdkworkGenerationArtifactsToHistoryMedia(artifacts, "image")).toEqual({
      asset: mediaResource("image", "https://cdn.example/a.png"),
      durationSeconds: undefined,
      images: [mediaResource("image", "https://cdn.example/a.png")],
    });
    expect(readSdkworkGenerationMediaUrl(mediaResource("image", "image"))).toBe("image");
  });

  it("appends streamed artifacts to history items without duplicating existing media", () => {
    const base: SdkworkGenerationHistoryItem = {
      createdAt: "2026-05-22T00:00:00Z",
      date: "2026-05-22",
      id: "pending",
      images: [],
      prompt: "Generate",
      type: "text",
      updatedAt: "2026-05-22T00:00:00Z",
    };
    const imageArtifact: SdkworkGenerationArtifact = {
      asset: mediaResource("image", "https://cdn.example/image.png"),
      modality: "image",
    };
    const withImage = appendSdkworkGenerationArtifactToHistoryItem(base, imageArtifact, {
      updatedAt: "2026-05-22T00:00:01Z",
    });
    expect(withImage).toMatchObject({
      asset: mediaResource("image", "https://cdn.example/image.png"),
      images: [mediaResource("image", "https://cdn.example/image.png")],
      status: "processing",
      type: "images",
      updatedAt: "2026-05-22T00:00:01Z",
    });
    expect(appendSdkworkGenerationArtifactToHistoryItem(withImage, imageArtifact, {
      updatedAt: "2026-05-22T00:00:02Z",
    })).toMatchObject({
      images: [mediaResource("image", "https://cdn.example/image.png")],
      updatedAt: "2026-05-22T00:00:02Z",
    });
  });
});

function mediaResource(
  kind: "image",
  url: string,
  durationSeconds?: number,
): SdkworkGenerationMediaResource {
  return {
    kind,
    source: "external_url",
    url,
    publicUrl: url,
    ...(durationSeconds === undefined ? {} : { durationSeconds }),
  };
}
