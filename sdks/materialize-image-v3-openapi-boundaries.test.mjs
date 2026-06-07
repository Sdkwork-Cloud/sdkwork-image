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
  assert.equal(
    open.paths["/image/v3/api/compat/openai/images/generations"].post.requestBody.content["application/json"].schema.$ref,
    "#/components/schemas/ImageOperationCommand",
  );
  assert.deepEqual(open.paths["/image/v3/api/compat/openai/images/generations"].post.security, [
    { ApiKey: [] },
  ]);
  for (const schemaName of [
    "ImageGenerationCommand",
    "ImageGenerationRefreshCommand",
    "ImageGenerationRetryCommand",
    "ImageGenerationCancelCommand",
    "ImageGeneration",
    "ImageGenerationOutput",
    "ImageGenerationStatus",
    "DriveSyncStatus",
  ]) {
    assert.equal(
      Object.hasOwn(open.components.schemas, schemaName),
      false,
      `open-api authority must not expose app/backend-only schema ${schemaName}`,
    );
  }

  assert.equal(app.openapi, "3.1.2");
  assert.equal(app.info["x-sdkwork-api-authority"], "sdkwork-image-app-api");
  assert.equal(app.info["x-sdkwork-sdk-family"], "sdkwork-image-app-sdk");
  assert.equal(app.paths["/image/v3/api/compat/openai/images/generations"], undefined);
  assert.equal(app.paths["/app/v3/api/image/generations"].post.operationId, "generations.create");
  assert.equal(app.paths["/app/v3/api/image/generations"].post["x-sdkwork-owner"], "sdkwork-image");
  assert.equal(app.paths["/app/v3/api/image/generations"].post["x-sdkwork-domain"], "image");
  assert.deepEqual(app.paths["/app/v3/api/image/generations"].post.security, [
    { AuthToken: [], AccessToken: [] },
  ]);
  assert.equal(
    app.paths["/app/v3/api/image/generations/{generationId}"].get.operationId,
    "generations.retrieve",
  );
  assert.equal(
    app.paths["/app/v3/api/image/generations/{generationId}"].get.parameters[0].name,
    "generationId",
  );
  assert.equal(
    app.paths["/app/v3/api/image/generations/{generationId}/refresh"].post.operationId,
    "generations.refresh",
  );
  assert.equal(
    app.paths["/app/v3/api/image/generations/{generationId}/cancel"].post.operationId,
    "generations.cancel",
  );

  assert.equal(backend.openapi, "3.1.2");
  assert.equal(backend.info["x-sdkwork-api-authority"], "sdkwork-image-backend-api");
  assert.equal(backend.info["x-sdkwork-sdk-family"], "sdkwork-image-backend-sdk");
  assert.equal(backend.paths["/image/v3/api/compat/openai/images/generations"], undefined);
  assert.equal(backend.paths["/backend/v3/api/image/generations"].get.operationId, "generations.list");
  assert.equal(
    backend.paths["/backend/v3/api/image/generations/{generationId}/retry"].post.operationId,
    "generations.retry",
  );
  assert.equal(
    backend.paths["/backend/v3/api/image/generations/{generationId}/cancel"].post.parameters[0].name,
    "generationId",
  );
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
    backend.paths["/backend/v3/api/image/provider_webhooks/{providerCode}"].post.operationId,
    "providerWebhooks.receive",
  );
  assert.equal(
    backend.paths["/backend/v3/api/image/provider_webhooks/{providerCode}"].post.parameters[0].name,
    "providerCode",
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

  const serialized = JSON.stringify({ app, backend });
  assert.equal(serialized.includes("generation_jobs"), false);
  assert.equal(serialized.includes("generationJobs"), false);
  assert.equal(serialized.includes("jobId"), false);
  assert.equal(app.components.schemas.ImageGenerationCommand.properties.scene.type, "string");
  assert.deepEqual(
    Object.keys(app.components.schemas.ImageGenerationCommand.properties).sort(),
    [
      "idempotencyKey",
      "model",
      "negativePrompt",
      "outputCount",
      "prompt",
      "providerCode",
      "referenceImages",
      "resolution",
      "scene",
      "style",
      "webhookUrl",
    ],
    "ImageGenerationCommand must expose only fields consumed by ImageGenerationCreateCommand",
  );
  for (const [surfaceName, authority] of Object.entries({ app, backend })) {
    assert.deepEqual(
      authority.components.schemas.ImageGenerationCommand.properties.referenceImages,
      {
        type: "array",
        maxItems: 16,
        items: { type: "string", minLength: 1, maxLength: 2048 },
        default: [],
      },
      `${surfaceName} ImageGenerationCommand must expose referenceImages`,
    );
  }
  assert.equal(app.components.schemas.ImageGenerationOutput.properties.scene.type, "string");
  assert.equal(
    app.components.schemas.ImageGenerationOutput.properties.resource.$ref,
    "#/components/schemas/MediaResource",
  );

  const backendSdkSource = readFileSync(
    resolve(
      workspaceRoot,
      "sdks/sdkwork-image-backend-sdk/sdkwork-image-backend-sdk-typescript/generated/server-openapi/src/api/image.ts",
    ),
    "utf8",
  );
  assert.match(backendSdkSource, /class ImageProviderWebhooksApi/u);
  assert.match(backendSdkSource, /providerWebhooks: ImageProviderWebhooksApi/u);
  assert.match(backendSdkSource, /receive\(providerCode: string, body: ImageOperationCommand\)/u);
  assert.match(backendSdkSource, /\/image\/provider_webhooks\/\$\{serializePathParameter\(providerCode/u);
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

test("image SDK family assembly metadata mirrors component SDK dependencies", () => {
  for (const familyName of [
    "sdkwork-image-sdk",
    "sdkwork-image-app-sdk",
    "sdkwork-image-backend-sdk",
  ]) {
    const assembly = readJson(`sdks/${familyName}/.sdkwork-assembly.json`);
    const component = readJson(`sdks/${familyName}/specs/component.spec.json`);

    assert.ok(
      Object.hasOwn(assembly, "sdkDependencies"),
      `${familyName} assembly must explicitly declare sdkDependencies`,
    );
    assert.deepEqual(
      assembly.sdkDependencies,
      component.contracts.sdkDependencies,
      `${familyName} assembly sdkDependencies must match component spec`,
    );
  }
});
