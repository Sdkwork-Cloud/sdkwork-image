import { describe, expect, it, vi } from "vitest";
import * as imageModule from "../src";

describe("sdkwork-image-pc-react service", () => {
  it("keeps deterministic fallback images when list operation fails", async () => {
    const createSdkworkImageService = (imageModule as Record<string, any>).createSdkworkImageService;
    const service = createSdkworkImageService({
      getSessionTokens: () => ({ authToken: "token" }),
      images: [
        {
          id: "fallback-image",
          presetId: "studio-product",
          prompt: "Fallback image",
          resolution: "1024x1024",
          status: "ready",
          style: "studio",
          title: "Fallback Image",
          updatedAt: "2026-04-01T01:00:00.000Z",
        },
      ],
      listImages: vi.fn()
        .mockResolvedValueOnce([
          {
            id: "remote-image",
            presetId: "studio-product",
            prompt: "Remote image",
            resolution: "1024x1024",
            status: "rendering",
            style: "studio",
            title: "Remote Image",
            updatedAt: "2026-04-03T01:00:00.000Z",
          },
        ])
        .mockRejectedValueOnce(new Error("offline")),
      presets: [
        { id: "studio-product", itemCount: 1, title: "Studio Product" },
      ],
    });

    const first = await service.getWorkspace();
    expect(first.isAuthenticated).toBe(true);
    expect(first.images[0]?.id).toBe("remote-image");

    const second = await service.getWorkspace();
    expect(second.images[0]?.id).toBe("fallback-image");
  });
});
