import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { SdkworkThemeProvider } from "@sdkwork/ui-pc-react/theme";
import * as generationModule from "../src/react.ts";

describe("sdkwork-generation-pc-react intl", () => {
  it("lets standalone generation components consume host overrides through the intl provider", () => {
    const GenerationIntlProvider = (generationModule as Record<string, any>).SdkworkGenerationIntlProvider;
    const GenerationRunList = (generationModule as Record<string, any>).SdkworkGenerationRunList;

    expect(GenerationIntlProvider).toBeTypeOf("function");

    if (typeof GenerationIntlProvider !== "function") {
      return;
    }

    render(
      <SdkworkThemeProvider defaultTheme="light">
        <GenerationIntlProvider
          locale="zh-CN"
          messages={{
            empty: {
              noRunsDescription: "Host no runs description",
              noRunsTitle: "Host no runs",
            },
          }}
        >
          <GenerationRunList runs={[]} />
        </GenerationIntlProvider>
      </SdkworkThemeProvider>,
    );

    expect(screen.getByText("Host no runs")).toBeInTheDocument();
    expect(screen.getByText("Host no runs description")).toBeInTheDocument();
  });

  it("keeps usable built-in English copy for standalone generation components without a host provider", () => {
    const GenerationRunDetail = (generationModule as Record<string, any>).SdkworkGenerationRunDetail;

    render(
      <SdkworkThemeProvider defaultTheme="light">
        <GenerationRunDetail run={null} />
      </SdkworkThemeProvider>,
    );

    expect(screen.getByText("No selected run")).toBeInTheDocument();
    expect(
      screen.getByText("Select a generation run to inspect prompt preview, model, and usage details."),
    ).toBeInTheDocument();
  });
});
