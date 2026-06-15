import { describe, expect, it, vi } from "vitest";
import { createSdkworkGenerationService } from "../src/generation-service";

describe("sdkwork-generation-pc-react service", () => {
  it("uses fallback runs when remote list operation fails", async () => {
    const listRuns = vi.fn()
      .mockResolvedValueOnce([
        {
          id: "remote",
          latencyMs: 900,
          model: "gpt-5.4-mini",
          promptPreview: "Remote",
          status: "completed",
          title: "Remote Run",
          tokensUsed: 1000,
          updatedAt: "2026-04-03T01:00:00.000Z",
        },
      ])
      .mockRejectedValueOnce(new Error("down"));

    const service = createSdkworkGenerationService({
      getSessionTokens: () => ({ authToken: "token" }),
      listRuns,
      runs: [
        {
          id: "fallback",
          latencyMs: 1200,
          model: "gpt-5.4-mini",
          promptPreview: "Fallback",
          status: "queued",
          title: "Fallback Run",
          tokensUsed: 300,
          updatedAt: "2026-04-01T01:00:00.000Z",
        },
      ],
    });

    const first = await service.getWorkspace();
    expect(first.runs[0]?.id).toBe("remote");
    expect(first.isAuthenticated).toBe(true);

    const second = await service.getWorkspace();
    expect(second.runs[0]?.id).toBe("fallback");
  });

  it("returns an empty workspace when sample runs are disabled", async () => {
    const service = createSdkworkGenerationService({
      getSessionTokens: () => ({ authToken: "token" }),
      includeSampleRuns: false,
    });

    const emptyWorkspace = service.getEmptyWorkspace();
    expect(emptyWorkspace.isAuthenticated).toBe(true);
    expect(emptyWorkspace.runs).toEqual([]);
    expect(emptyWorkspace.digest).toEqual({
      completedRuns: 0,
      failedRuns: 0,
      runningRuns: 0,
      totalRuns: 0,
      totalTokensUsed: 0,
    });

    const workspace = await service.getWorkspace();
    expect(workspace.isAuthenticated).toBe(true);
    expect(workspace.runs).toEqual([]);
    expect(workspace.digest).toEqual({
      completedRuns: 0,
      failedRuns: 0,
      runningRuns: 0,
      totalRuns: 0,
      totalTokensUsed: 0,
    });
  });
});
