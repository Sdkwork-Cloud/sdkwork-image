import {
  fireEvent,
  render,
  screen,
  waitFor,
} from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { SdkworkThemeProvider } from "@sdkwork/ui-pc-react/theme";
import * as imageModule from "../src";

describe("sdkwork-image-pc-react page", () => {
  it("renders image workspace and filters images by search input", async () => {
    const Page = (imageModule as Record<string, any>).SdkworkImagePage;

    const { container } = render(
      <SdkworkThemeProvider defaultTheme="light">
        <Page
          service={{
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
          }}
        />
      </SdkworkThemeProvider>,
    );

    expect(await screen.findByRole("heading", { name: /image workspace/i })).toBeInTheDocument();

    fireEvent.change(screen.getByPlaceholderText(/search images/i), {
      target: { value: "launch" },
    });

    await waitFor(() => {
      expect(screen.queryByText("Device Beauty")).not.toBeInTheDocument();
    });
    expect(container.innerHTML).not.toContain("border-white/10");
    expect(container.innerHTML).not.toContain("text-white/72");
    expect(container.innerHTML).not.toContain("text-white/60");
  });

  it("applies host localization overrides across the image page seam", async () => {
    const Page = (imageModule as Record<string, any>).SdkworkImagePage;

    render(
      <SdkworkThemeProvider defaultTheme="light">
        <Page
          messages={{
            page: {
              searchPlaceholder: "Host image search",
              title: "Host image cockpit",
            },
            presets: {
              all: "Host presets",
            },
            status: {
              all: "Host all",
            },
          }}
          service={{
            getEmptyWorkspace: vi.fn().mockReturnValue({
              digest: { activeRenders: 0, presetCount: 0, readyImages: 0, totalImages: 0 },
              images: [],
              isAuthenticated: false,
              presets: [],
            }),
            getWorkspace: vi.fn().mockResolvedValue({
              digest: { activeRenders: 0, presetCount: 0, readyImages: 0, totalImages: 0 },
              images: [],
              isAuthenticated: true,
              presets: [],
            }),
          }}
        />
      </SdkworkThemeProvider>,
    );

    expect(await screen.findByRole("heading", { name: "Host image cockpit" })).toBeInTheDocument();
    expect(screen.getByPlaceholderText("Host image search")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Host presets" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Host all" })).toBeInTheDocument();
  });
});
