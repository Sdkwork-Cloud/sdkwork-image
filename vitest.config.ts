import path from "node:path";
import { fileURLToPath } from "node:url";
import { configDefaults, defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";

const workspaceRoot = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  root: workspaceRoot,
  plugins: [react()],
  resolve: {
    alias: [
      {
        find: "react",
        replacement: path.resolve(workspaceRoot, "node_modules/react"),
      },
      {
        find: "react-dom",
        replacement: path.resolve(workspaceRoot, "node_modules/react-dom"),
      },
      {
        find: "react/jsx-runtime",
        replacement: path.resolve(workspaceRoot, "node_modules/react/jsx-runtime.js"),
      },
      {
        find: "react/jsx-dev-runtime",
        replacement: path.resolve(workspaceRoot, "node_modules/react/jsx-dev-runtime.js"),
      },
      {
        find: "lucide-react",
        replacement: path.resolve(workspaceRoot, "node_modules/lucide-react"),
      },
      {
        find: "@sdkwork/image-contracts",
        replacement: path.resolve(workspaceRoot, "packages/common/image/sdkwork-image-contracts/src/index.ts"),
      },
      {
        find: "@sdkwork/image-pc-react",
        replacement: path.resolve(workspaceRoot, "packages/pc-react/content/sdkwork-image-pc-react/src/index.ts"),
      },
      {
        find: "@sdkwork/core-pc-react",
        replacement: path.resolve(workspaceRoot, "tests/support/sdkwork-core-pc-react.ts"),
      },
      {
        find: "@sdkwork/ui-pc-react/theme",
        replacement: path.resolve(
          workspaceRoot,
          "../sdkwork-ui/sdkwork-ui-pc-react/src/theme/index.ts",
        ),
      },
      {
        find: "@sdkwork/ui-pc-react",
        replacement: path.resolve(
          workspaceRoot,
          "../sdkwork-ui/sdkwork-ui-pc-react/src/index.ts",
        ),
      },
    ],
    dedupe: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime", "lucide-react"],
  },
  test: {
    environment: "jsdom",
    exclude: [...configDefaults.exclude],
    include: ["packages/**/*.test.ts", "packages/**/*.test.tsx", "sdks/**/*.test.ts"],
    setupFiles: [path.join(workspaceRoot, "vitest.setup.ts")],
  },
});
