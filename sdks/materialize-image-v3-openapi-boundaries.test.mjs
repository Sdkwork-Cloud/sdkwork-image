import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { test } from "node:test";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";

const workspaceRoot = resolve(import.meta.dirname, "..");
const materializerPath = resolve(workspaceRoot, "sdks/materialize-image-v3-openapi-boundaries.mjs");

function readJson(relativePath) {
  return JSON.parse(readFileSync(resolve(workspaceRoot, relativePath), "utf8"));
}

test("image OpenAPI materializer writes app and backend SDK authorities from Rust route catalog", () => {
  const result = spawnSync(process.execPath, [materializerPath], {
    cwd: workspaceRoot,
    encoding: "utf8",
  });

  assert.equal(result.status, 0, result.stderr || result.stdout);

  for (const relativePath of [
    "sdks/sdkwork-image-sdk/openapi/sdkwork-image-open-api.openapi.yaml",
    "sdks/sdkwork-image-sdk/openapi/sdkwork-image-open-api.sdkgen.yaml",
    "sdks/sdkwork-image-sdk/openapi/sdkwork-image-open-api.flutter.sdkgen.yaml",
    "sdks/sdkwork-image-app-sdk/openapi/sdkwork-image-app-api.openapi.yaml",
    "sdks/sdkwork-image-app-sdk/openapi/sdkwork-image-app-api.sdkgen.yaml",
    "sdks/sdkwork-image-app-sdk/openapi/sdkwork-image-app-api.flutter.sdkgen.yaml",
    "sdks/sdkwork-image-backend-sdk/openapi/sdkwork-image-backend-api.openapi.yaml",
    "sdks/sdkwork-image-backend-sdk/openapi/sdkwork-image-backend-api.sdkgen.yaml",
    "sdks/sdkwork-image-backend-sdk/openapi/sdkwork-image-backend-api.flutter.sdkgen.yaml",
  ]) {
    assert.equal(existsSync(resolve(workspaceRoot, relativePath)), true, `missing ${relativePath}`);
  }

  const app = readJson("sdks/sdkwork-image-app-sdk/openapi/sdkwork-image-app-api.openapi.yaml");
  const backend = readJson("sdks/sdkwork-image-backend-sdk/openapi/sdkwork-image-backend-api.openapi.yaml");
  const open = readJson("sdks/sdkwork-image-sdk/openapi/sdkwork-image-open-api.openapi.yaml");

  assert.equal(open.openapi, "3.1.2");
  assert.equal(open.info["x-sdkwork-api-authority"], "sdkwork-image-open-api");
  assert.equal(open.info["x-sdkwork-sdk-family"], "sdkwork-image-sdk");
  assert.equal(
    open.paths["/image/v3/api/compat/openai/images/generations"].post.operationId,
    "compat.openai.images.generate",
  );
  assert.equal(
    open.paths["/image/v3/api/compat/openai/images/generations"].post["x-sdkwork-owner"],
    "sdkwork-image",
  );
  assert.equal(
    open.paths["/image/v3/api/compat/openai/images/generations"].post["x-sdkwork-domain"],
    "image",
  );
  assert.deepEqual(open.paths["/image/v3/api/compat/openai/images/generations"].post.security, [
    { ApiKey: [] },
  ]);

  assert.equal(app.openapi, "3.1.2");
  assert.equal(app.info["x-sdkwork-api-authority"], "sdkwork-image-app-api");
  assert.equal(app.info["x-sdkwork-sdk-family"], "sdkwork-image-app-sdk");
  assert.equal(app.paths["/image/v3/api/compat/openai/images/generations"], undefined);
  assert.equal(app.paths["/app/v3/api/image/generation_jobs"].post.operationId, "generationJobs.create");
  assert.equal(app.paths["/app/v3/api/image/generation_jobs"].post["x-sdkwork-owner"], "sdkwork-image");
  assert.equal(app.paths["/app/v3/api/image/generation_jobs"].post["x-sdkwork-domain"], "image");
  assert.deepEqual(app.paths["/app/v3/api/image/generation_jobs"].post.security, [
    { AuthToken: [], AccessToken: [] },
  ]);

  assert.equal(backend.openapi, "3.1.2");
  assert.equal(backend.info["x-sdkwork-api-authority"], "sdkwork-image-backend-api");
  assert.equal(backend.info["x-sdkwork-sdk-family"], "sdkwork-image-backend-sdk");
  assert.equal(backend.paths["/image/v3/api/compat/openai/images/generations"], undefined);
  assert.equal(backend.paths["/backend/v3/api/image/presets"].post.operationId, "presets.create");
  assert.equal(backend.paths["/backend/v3/api/image/galleries/{galleryId}/items"].post.operationId, "galleries.items.create");
  assert.equal(
    backend.paths["/backend/v3/api/image/galleries/{galleryId}/items/{itemId}"].delete.operationId,
    "galleries.items.delete",
  );
  assert.equal(
    backend.paths["/backend/v3/api/image/galleries/{galleryId}/items/{itemId}"].delete.parameters.length,
    2,
  );

  assert.equal(
    readFileSync(resolve(workspaceRoot, "sdks/sdkwork-image-sdk/openapi/sdkwork-image-open-api.openapi.yaml"), "utf8"),
    readFileSync(resolve(workspaceRoot, "sdks/sdkwork-image-sdk/openapi/sdkwork-image-open-api.sdkgen.yaml"), "utf8"),
  );
  assert.equal(
    readFileSync(resolve(workspaceRoot, "sdks/sdkwork-image-app-sdk/openapi/sdkwork-image-app-api.openapi.yaml"), "utf8"),
    readFileSync(resolve(workspaceRoot, "sdks/sdkwork-image-app-sdk/openapi/sdkwork-image-app-api.sdkgen.yaml"), "utf8"),
  );
  assert.equal(
    readFileSync(resolve(workspaceRoot, "sdks/sdkwork-image-backend-sdk/openapi/sdkwork-image-backend-api.openapi.yaml"), "utf8"),
    readFileSync(resolve(workspaceRoot, "sdks/sdkwork-image-backend-sdk/openapi/sdkwork-image-backend-api.sdkgen.yaml"), "utf8"),
  );
});

test("image workspace TypeScript SDK script covers open, app, and backend SDK families", () => {
  const packageJson = readJson("package.json");
  const generateScript = packageJson.scripts?.["sdk:generate:ts"] ?? "";

  for (const expectedFamily of [
    "sdks/sdkwork-image-sdk/bin/generate-sdk.ps1",
    "sdks/sdkwork-image-app-sdk/bin/generate-sdk.ps1",
    "sdks/sdkwork-image-backend-sdk/bin/generate-sdk.ps1",
  ]) {
    assert.match(generateScript, new RegExp(expectedFamily.replaceAll("/", "[/\\\\]")));
  }

  assert.doesNotMatch(generateScript, /;/u);
  assert.match(generateScript, /generate-sdk\.ps1 -Languages typescript && powershell/u);
});
