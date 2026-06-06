import { describe, expect, it } from "vitest";
import * as imageModule from "../src";

describe("sdkwork-image-pc-react appearance", () => {
  it("exports theme-driven tone styles for reusable image chips and accents", () => {
    const createToneStyle = (imageModule as Record<string, any>).createSdkworkImageToneStyle;
    const createHeroTextStyle = (imageModule as Record<string, any>).createSdkworkImageHeroTextStyle;

    expect(createToneStyle).toBeTypeOf("function");
    expect(createHeroTextStyle).toBeTypeOf("function");

    if (
      typeof createToneStyle !== "function"
      || typeof createHeroTextStyle !== "function"
    ) {
      return;
    }

    expect(
      createToneStyle("brand", {
        backgroundWeight: 18,
        borderWeight: 32,
      }),
    ).toEqual({
      backgroundColor: "color-mix(in srgb, var(--sdk-color-brand-primary) 18%, transparent)",
      borderColor: "color-mix(in srgb, var(--sdk-color-brand-primary) 32%, transparent)",
      color: "var(--sdk-color-brand-primary)",
    });
    expect(createHeroTextStyle("muted")).toEqual({
      color: "color-mix(in srgb, white 72%, var(--sdk-color-brand-accent))",
    });
  });

  it("exports layered Sdkwork-style image gradients for hero and panel surfaces", () => {
    const createPanelStyle = (imageModule as Record<string, any>).createSdkworkImagePanelStyle;
    const createHeroStyle = (imageModule as Record<string, any>).createSdkworkImageHeroStyle;
    const createBackdropStyle = (imageModule as Record<string, any>).createSdkworkImageBackdropStyle;

    expect(createPanelStyle).toBeTypeOf("function");
    expect(createHeroStyle).toBeTypeOf("function");
    expect(createBackdropStyle).toBeTypeOf("function");

    if (
      typeof createPanelStyle !== "function"
      || typeof createHeroStyle !== "function"
      || typeof createBackdropStyle !== "function"
    ) {
      return;
    }

    expect(createPanelStyle("accent").backgroundImage).toContain("var(--sdk-color-brand-accent)");
    expect(createPanelStyle("accent").backgroundImage).toContain("var(--sdk-color-surface-panel)");
    expect(createHeroStyle().backgroundImage).toContain("var(--sdk-color-brand-accent)");
    expect(createHeroStyle().backgroundImage).toContain("var(--sdk-color-surface-canvas)");
    expect(createHeroStyle().backgroundImage).toContain("var(--sdk-color-surface-panel)");
    expect(createHeroStyle().backgroundImage).toContain("var(--sdk-color-surface-elevated)");
    expect(createHeroStyle().backgroundImage).not.toContain("#18181b");
    expect(createBackdropStyle().backgroundImage).toContain("var(--sdk-color-brand-primary)");
    expect(createBackdropStyle().backgroundImage).toContain("var(--sdk-color-brand-accent)");
  });
});
