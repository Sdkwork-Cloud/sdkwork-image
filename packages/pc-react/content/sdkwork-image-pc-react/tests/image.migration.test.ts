import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

const packageRoot = resolve(import.meta.dirname, "..");

describe("sdkwork-image migration boundary", () => {
  it("declares sdkwork-image ownership instead of appbase ownership", () => {
    const packageJson = JSON.parse(readFileSync(resolve(packageRoot, "package.json"), "utf8")) as {
      peerDependencies?: Record<string, string>;
      sdkwork?: { workspace?: string };
    };

    expect(packageJson.sdkwork?.workspace).toBe("sdkwork-image");
    expect(packageJson.peerDependencies).not.toHaveProperty("@sdkwork/appbase-pc-react");
    expect(packageJson.peerDependencies).toHaveProperty("@sdkwork/image-contracts");
  });

  it("uses image contracts for media resources instead of appbase media exports", () => {
    const imageSource = readFileSync(resolve(packageRoot, "src/image.ts"), "utf8");

    expect(imageSource).toContain("@sdkwork/image-contracts");
    expect(imageSource).not.toContain("@sdkwork/appbase-pc-react");
  });
});
