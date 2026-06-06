import { describe, expect, it } from "vitest";
import * as generationModule from "../src";

describe("sdkwork-generation-pc-react domain contract", () => {
  it("creates manifest/route helpers and deterministic workspace", () => {
    const {
      createEmptySdkworkGenerationWorkspace,
      createGenerationRouteIntent,
      createGenerationWorkspaceManifest,
      generationPackageMeta,
    } = generationModule as Record<string, any>;

    expect(generationPackageMeta).toMatchObject({
      domain: "content",
      package: "@sdkwork/generation-pc-react",
      status: "ready",
    });

    expect(createGenerationWorkspaceManifest()).toMatchObject({
      capability: "generation",
      routePath: "/generation",
    });

    expect(
      createGenerationRouteIntent({
        runId: "run-desktop-hero",
      }),
    ).toEqual({
      focusWindow: true,
      route: "/generation?runId=run-desktop-hero",
      runId: "run-desktop-hero",
      source: "generation-workspace",
      type: "generation-route-intent",
    });

    expect(createEmptySdkworkGenerationWorkspace()).toMatchObject({
      digest: {
        totalRuns: 3,
      },
      runs: expect.arrayContaining([
        expect.objectContaining({ id: "run-desktop-hero" }),
      ]),
    });

    expect((generationModule as Record<string, unknown>).SdkworkGenerationPage).toBeUndefined();
    expect((generationModule as Record<string, unknown>).SdkworkGenerationIntlProvider).toBeUndefined();
    expect((generationModule as Record<string, unknown>).SdkworkGenerationRunList).toBeUndefined();
  });
});
