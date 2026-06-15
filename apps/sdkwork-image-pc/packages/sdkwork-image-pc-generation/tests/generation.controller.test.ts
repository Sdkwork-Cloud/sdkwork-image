import { describe, expect, it, vi } from "vitest";
import * as generationModule from "../src/react.ts";

describe("sdkwork-generation-pc-react controller", () => {
  it("filters runs by status and query", async () => {
    const controller = generationModule.createSdkworkGenerationController({
      service: {
        getEmptyWorkspace: vi.fn().mockReturnValue({
          digest: { completedRuns: 0, failedRuns: 0, runningRuns: 0, totalRuns: 0, totalTokensUsed: 0 },
          isAuthenticated: false,
          runs: [],
        }),
        getWorkspace: vi.fn().mockResolvedValue({
          digest: { completedRuns: 1, failedRuns: 0, runningRuns: 1, totalRuns: 2, totalTokensUsed: 2000 },
          isAuthenticated: true,
          runs: [
            {
              id: "run-1",
              latencyMs: 900,
              model: "gpt-5.4",
              promptPreview: "pipeline",
              status: "running",
              title: "Pipeline Run",
              tokensUsed: 1000,
              updatedAt: "2026-04-03T01:00:00.000Z",
            },
            {
              id: "run-2",
              latencyMs: 700,
              model: "gpt-5.4-mini",
              promptPreview: "summary",
              status: "completed",
              title: "Summary Run",
              tokensUsed: 1000,
              updatedAt: "2026-04-02T01:00:00.000Z",
            },
          ],
        }),
      },
    });

    await controller.bootstrap();
    controller.setStatus("completed");
    expect(controller.getState().visibleRuns).toHaveLength(1);

    controller.setSearchQuery("pipeline");
    expect(controller.getState().visibleRuns).toHaveLength(0);
  });

  it("uses host override fallback copy when workspace bootstrap fails without an Error instance", async () => {
    const controller = (generationModule as Record<string, any>).createSdkworkGenerationController({
      messages: {
        service: {
          loadWorkspaceFailed: "Host load failed",
        },
      },
      service: {
        getEmptyWorkspace: vi.fn().mockReturnValue({
          digest: { completedRuns: 0, failedRuns: 0, runningRuns: 0, totalRuns: 0, totalTokensUsed: 0 },
          isAuthenticated: false,
          runs: [],
        }),
        getWorkspace: vi.fn().mockRejectedValue("boom"),
      },
    });

    await expect(controller.bootstrap()).rejects.toBe("boom");
    expect(controller.getState().lastError).toBe("Host load failed");
  });
});
