import { describe, expect, it, vi } from "vitest";
import * as imageModule from "../src";

describe("sdkwork-image-pc-react controller", () => {
  it("filters images by preset, status, and search state", async () => {
    const createSdkworkImageController = (imageModule as Record<string, any>).createSdkworkImageController;

    const controller = createSdkworkImageController({
      service: {
        getEmptyWorkspace: vi.fn().mockReturnValue({
          digest: { activeRenders: 0, presetCount: 0, readyImages: 0, totalImages: 0 },
          images: [],
          isAuthenticated: false,
          presets: [],
        }),
        getWorkspace: vi.fn().mockResolvedValue({
          digest: { activeRenders: 1, presetCount: 2, readyImages: 1, totalImages: 2 },
          images: [
            {
              id: "image-device-beauty",
              presetId: "studio-product",
              prompt: "Device beauty",
              resolution: "1024x1024",
              status: "ready",
              style: "studio",
              title: "Device Beauty",
              updatedAt: "2026-04-03T01:00:00.000Z",
            },
            {
              id: "image-launch-poster",
              presetId: "launch-key-visual",
              prompt: "Launch poster",
              resolution: "1024x1536",
              status: "queued",
              style: "campaign",
              title: "Launch Poster",
              updatedAt: "2026-04-02T01:00:00.000Z",
            },
          ],
          isAuthenticated: true,
          presets: [
            { id: "studio-product", itemCount: 1, title: "Studio Product" },
            { id: "launch-key-visual", itemCount: 1, title: "Launch Key Visual" },
          ],
        }),
      },
    });

    await controller.bootstrap();
    controller.setPreset("launch-key-visual");
    expect(controller.getState().visibleImages).toHaveLength(1);
    controller.setStatus("queued");
    expect(controller.getState().visibleImages).toHaveLength(1);
    controller.setSearchQuery("device");
    expect(controller.getState().visibleImages).toHaveLength(0);
  });

  it("uses host override fallback copy when image bootstrap fails without an Error instance", async () => {
    const createSdkworkImageController = (imageModule as Record<string, any>).createSdkworkImageController;

    const controller = createSdkworkImageController({
      messages: {
        service: {
          loadWorkspaceFailed: "Host image load failed",
        },
      },
      service: {
        getEmptyWorkspace: vi.fn().mockReturnValue({
          digest: { activeRenders: 0, presetCount: 0, readyImages: 0, totalImages: 0 },
          images: [],
          isAuthenticated: false,
          presets: [],
        }),
        getWorkspace: vi.fn().mockRejectedValue("boom"),
      },
    });

    await expect(controller.bootstrap()).rejects.toBe("boom");
    expect(controller.getState().lastError).toBe("Host image load failed");
  });
});
