import {
  fireEvent,
  render,
  screen,
  waitFor,
} from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { SdkworkThemeProvider } from "@sdkwork/ui-pc-react/theme";
import * as generationModule from "../src/react.ts";

describe("sdkwork-generation-pc-react page", () => {
  it("renders generation page and filters runs from search", async () => {
    const Page = (generationModule as Record<string, any>).SdkworkGenerationPage;

    const { container } = render(
      <SdkworkThemeProvider defaultTheme="light">
        <Page
          service={{
            getEmptyWorkspace: vi.fn().mockReturnValue({
              digest: { completedRuns: 0, failedRuns: 0, runningRuns: 0, totalRuns: 0, totalTokensUsed: 0 },
              isAuthenticated: false,
              runs: [],
            }),
            getWorkspace: vi.fn().mockResolvedValue({
              digest: { completedRuns: 1, failedRuns: 1, runningRuns: 0, totalRuns: 2, totalTokensUsed: 2000 },
              isAuthenticated: true,
              runs: [
                {
                  id: "run-a",
                  latencyMs: 1000,
                  model: "gpt-5.4-mini",
                  promptPreview: "create cards",
                  status: "completed",
                  title: "Cards Run",
                  tokensUsed: 1000,
                  updatedAt: "2026-04-03T02:00:00.000Z",
                },
                {
                  id: "run-b",
                  latencyMs: 1900,
                  model: "gpt-5.4",
                  promptPreview: "generate layout",
                  status: "failed",
                  title: "Layout Run",
                  tokensUsed: 1000,
                  updatedAt: "2026-04-02T02:00:00.000Z",
                },
              ],
            }),
          }}
        />
      </SdkworkThemeProvider>,
    );

    expect(await screen.findByRole("heading", { name: /generation workspace/i })).toBeInTheDocument();

    fireEvent.change(screen.getByPlaceholderText(/search generation runs/i), {
      target: { value: "cards" },
    });

    await waitFor(() => {
      expect(screen.queryByText("Layout Run")).not.toBeInTheDocument();
    });
    expect(container.innerHTML).not.toContain("border-white/10");
    expect(container.innerHTML).not.toContain("text-white/72");
    expect(container.innerHTML).not.toContain("text-white/60");
  });

  it("applies host localization overrides across the generation page seam", async () => {
    const Page = (generationModule as Record<string, any>).SdkworkGenerationPage;

    render(
      <SdkworkThemeProvider defaultTheme="light">
        <Page
          locale="zh-CN"
          messages={{
            actions: {
              refresh: "Host refresh",
            },
            page: {
              searchPlaceholder: "Host search field",
              title: "Host generation cockpit",
            },
            sort: {
              recent: "Host recent",
            },
            status: {
              all: "Host all",
            },
          }}
          service={{
            getEmptyWorkspace: vi.fn().mockReturnValue({
              digest: { completedRuns: 0, failedRuns: 0, runningRuns: 0, totalRuns: 0, totalTokensUsed: 0 },
              isAuthenticated: false,
              runs: [],
            }),
            getWorkspace: vi.fn().mockResolvedValue({
              digest: { completedRuns: 0, failedRuns: 0, runningRuns: 0, totalRuns: 0, totalTokensUsed: 0 },
              isAuthenticated: true,
              runs: [],
            }),
          }}
        />
      </SdkworkThemeProvider>,
    );

    expect(
      await screen.findByRole("heading", {
        name: "Host generation cockpit",
      }),
    ).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Host refresh" })).toBeInTheDocument();
    expect(screen.getByPlaceholderText("Host search field")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Host recent" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Host all" })).toBeInTheDocument();
  });
});
