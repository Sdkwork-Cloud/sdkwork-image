import { describe, expect, it } from "vitest";
import * as imageModule from "../src";

describe("sdkwork-image-pc-react domain contract", () => {
  it("creates workspace manifest, route intents, and deterministic image workspace", () => {
    const {
      createEmptySdkworkImageWorkspace,
      createImageRouteIntent,
      createImageWorkspaceManifest,
      imagePackageMeta,
    } = imageModule as Record<string, any>;

    expect(imagePackageMeta).toMatchObject({
      domain: "content",
      package: "@sdkwork/image-pc",
      status: "ready",
    });

    expect(createImageWorkspaceManifest({ title: "Image Workspace" })).toMatchObject({
      capability: "image",
      routePath: "/image",
      title: "Image Workspace",
    });

    expect(createImageRouteIntent({ imageId: "image-device-beauty", presetId: "studio-product" })).toEqual({
      focusWindow: true,
      imageId: "image-device-beauty",
      presetId: "studio-product",
      route: "/image?presetId=studio-product&imageId=image-device-beauty",
      source: "image-workspace",
      type: "image-route-intent",
    });

    const workspace = createEmptySdkworkImageWorkspace();

    expect(workspace).toMatchObject({
      digest: {
        presetCount: 3,
        totalImages: 4,
      },
      isAuthenticated: false,
      presets: expect.arrayContaining([
        expect.objectContaining({ id: "studio-product" }),
      ]),
    });
    expect(workspace.images[0]).toMatchObject({
      resource: {
        metadata: {
          scene: "studio",
        },
        kind: "image",
        source: "drive",
      },
    });
    expect(workspace.images[0].resource.uri).toContain("drive://spaces/space-ai-generated-user-demo");
  });
});
