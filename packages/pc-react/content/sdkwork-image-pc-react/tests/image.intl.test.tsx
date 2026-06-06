import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { SdkworkThemeProvider } from "@sdkwork/ui-pc-react/theme";
import * as imageModule from "../src";

describe("sdkwork-image-pc-react intl", () => {
  it("lets standalone image components consume host overrides through the intl provider", () => {
    const ImageIntlProvider = (imageModule as Record<string, any>).SdkworkImageIntlProvider;
    const ImageGallery = (imageModule as Record<string, any>).SdkworkImageGallery;

    expect(ImageIntlProvider).toBeTypeOf("function");

    if (typeof ImageIntlProvider !== "function") {
      return;
    }

    render(
      <SdkworkThemeProvider defaultTheme="light">
        <ImageIntlProvider
          messages={{
            empty: {
              noImagesDescription: "Host image empty description",
              noImagesTitle: "Host image empty",
            },
          }}
        >
          <ImageGallery images={[]} />
        </ImageIntlProvider>
      </SdkworkThemeProvider>,
    );

    expect(screen.getByText("Host image empty")).toBeInTheDocument();
    expect(screen.getByText("Host image empty description")).toBeInTheDocument();
  });
});
