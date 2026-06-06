import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const imageRoot = resolve(__dirname, "..");

const routeSources = [
  {
    owner: "image",
    path: resolve(imageRoot, "packages/native-rust/image/sdkwork-image-http-rust/src/lib.rs"),
    constructors: ["ImageHttpRoute::new"],
  },
];

const surfaces = {
  open: {
    sdkType: "open-api",
    generatorSdkType: "custom",
    sdkOwner: "sdkwork-image",
    familyName: "sdkwork-image-sdk",
    authorityName: "sdkwork-image-open-api",
    title: "SDKWork Image Open API",
    description: "Public/domain contract for SDKWork image compatibility integrations.",
    prefix: "/image/v3/api",
    audience: "External integrations, provider-compatible image clients, and public image automation",
    authMode: "api-key",
  },
  app: {
    sdkType: "app",
    generatorSdkType: "app",
    sdkOwner: "sdkwork-image",
    familyName: "sdkwork-image-app-sdk",
    authorityName: "sdkwork-image-app-api",
    title: "SDKWork Image App API",
    description: "App/client contract for SDKWork image presets, generation jobs, edit tasks, assets, and galleries.",
    prefix: "/app/v3/api",
    audience: "App, desktop, mobile, H5, and user-facing image clients",
    authMode: "dual-token",
  },
  backend: {
    sdkType: "backend",
    generatorSdkType: "backend",
    sdkOwner: "sdkwork-image",
    familyName: "sdkwork-image-backend-sdk",
    authorityName: "sdkwork-image-backend-api",
    title: "SDKWork Image Backend API",
    description: "Backend/admin contract for SDKWork image presets, generation jobs, assets, and gallery management.",
    prefix: "/backend/v3/api",
    audience: "Backend consoles, operators, image control-plane integrations, and admin automation",
    authMode: "dual-token",
  },
};

const methodNames = {
  Delete: "delete",
  Get: "get",
  Patch: "patch",
  Post: "post",
  Put: "put",
};

async function main() {
  const routes = await collectRoutes();
  const openRoutes = selectRoutes(routes, surfaces.open.prefix);
  const appRoutes = selectRoutes(routes, surfaces.app.prefix);
  const backendRoutes = selectRoutes(routes, surfaces.backend.prefix);

  if (openRoutes.length === 0) {
    throw new Error("No image open-api routes were materialized from Rust route catalogs.");
  }
  if (appRoutes.length === 0) {
    throw new Error("No image app-api routes were materialized from Rust route catalogs.");
  }
  if (backendRoutes.length === 0) {
    throw new Error("No image backend-api routes were materialized from Rust route catalogs.");
  }

  await writeSurfaceOpenApi(surfaces.open, openRoutes);
  await writeSurfaceOpenApi(surfaces.app, appRoutes);
  await writeSurfaceOpenApi(surfaces.backend, backendRoutes);

  console.log(`Materialized ${openRoutes.length} image open-api operations.`);
  console.log(`Materialized ${appRoutes.length} image app-api operations.`);
  console.log(`Materialized ${backendRoutes.length} image backend-api operations.`);
}

async function collectRoutes() {
  const routes = [];
  for (const source of routeSources) {
    const content = await readFile(source.path, "utf8");
    const constructors = source.constructors.map((constructor) => escapeRegExp(constructor)).join("|");
    const routePattern = new RegExp(
      `(?:${constructors})\\s*\\(\\s*HttpMethod::(Get|Post|Patch|Put|Delete)\\s*,\\s*"([^"]+)"\\s*,\\s*"([^"]+)"\\s*,\\s*"([^"]+)"\\s*,?\\s*\\)`,
      "g",
    );

    for (const match of content.matchAll(routePattern)) {
      routes.push({
        owner: source.owner,
        method: methodNames[match[1]],
        path: match[2],
        tag: toLowerCamel(match[3]),
        operationId: match[4],
      });
    }
  }

  const byKey = new Map();
  for (const route of routes) {
    const key = `${route.method.toUpperCase()} ${route.path}`;
    if (!byKey.has(key)) {
      byKey.set(key, route);
      continue;
    }
    const previous = byKey.get(key);
    if (previous.operationId !== route.operationId || previous.tag !== route.tag) {
      throw new Error(
        `Conflicting image route metadata for ${key}: ${previous.operationId}/${previous.tag} vs ${route.operationId}/${route.tag}`,
      );
    }
  }

  return Array.from(byKey.values()).sort(compareRoutes);
}

function selectRoutes(routes, prefix) {
  return routes.filter((route) => route.path.startsWith(`${prefix}/`) || route.path === prefix);
}

async function writeSurfaceOpenApi(surface, routes) {
  const authority = buildOpenApi(surface, routes);
  const familyRoot = resolve(imageRoot, "sdks", surface.familyName);
  const openapiRoot = resolve(familyRoot, "openapi");
  await mkdir(openapiRoot, { recursive: true });

  const authorityPath = resolve(openapiRoot, `${surface.authorityName}.openapi.yaml`);
  const sdkgenPath = resolve(openapiRoot, `${surface.authorityName}.sdkgen.yaml`);
  const flutterSdkgenPath = resolve(openapiRoot, `${surface.authorityName}.flutter.sdkgen.yaml`);
  const content = `${JSON.stringify(authority, null, 2)}\n`;

  await writeFile(authorityPath, content, "utf8");
  await writeFile(sdkgenPath, content, "utf8");
  await writeFile(flutterSdkgenPath, content, "utf8");
}

function buildOpenApi(surface, routes) {
  const paths = {};
  for (const route of routes) {
    const pathItem = paths[route.path] ?? {};
    pathItem[route.method] = buildOperation(surface, route);
    paths[route.path] = pathItem;
  }

  const tags = Array.from(new Set(routes.map((route) => route.tag)))
    .sort()
    .map((name) => ({
      name,
      description: `${toTitle(name)} API resources.`,
      "x-sdk-nested-resource-surface": true,
    }));

  return {
    openapi: "3.1.2",
    info: {
      title: surface.title,
      version: "1.0.0",
      description: surface.description,
      "x-sdkwork-api-authority": surface.authorityName,
      "x-sdkwork-sdk-family": surface.familyName,
      "x-sdkwork-audience": surface.audience,
    },
    servers: [
      {
        url: "http://localhost:8080",
        description: "Local sdkwork-image runtime",
      },
    ],
    tags,
    security: securityRequirement(surface),
    paths,
    components: {
      securitySchemes: securitySchemes(surface),
      schemas: buildSchemas(),
    },
    "x-sdkwork-materialized-from": routeSources.map((source) => ({
      owner: source.owner,
      path: relativeForOpenApi(source.path),
    })),
    "x-sdkwork-request-context": {
      contextObject: surface.authMode === "api-key" ? "OpenApiRequestContext" : "AppRequestContext",
      serverRequestId: "server-owned",
      clientRequestIdHeader: "forbidden",
      tenantSource: surface.authMode === "api-key" ? "ApiKey" : "AuthToken + AccessToken",
      organizationSource: surface.authMode === "api-key" ? "ApiKey" : "AuthToken + AccessToken",
      userSource: surface.authMode === "api-key" ? "ApiKey" : "AuthToken + AccessToken",
    },
  };
}

function buildOperation(surface, route) {
  const operation = {
    tags: [route.tag],
    summary: `${toTitle(route.operationId)}.`,
    operationId: route.operationId,
    parameters: extractPathParameters(route.path),
    responses: {
      200: jsonResponse("Success", "#/components/schemas/ImageApiResult"),
      400: problemResponse("Bad request"),
      401: problemResponse("Unauthorized"),
      403: problemResponse("Forbidden"),
      404: problemResponse("Not found"),
      409: problemResponse("Conflict"),
      500: problemResponse("Internal server error"),
    },
    security: securityRequirement(surface),
    "x-sdkwork-owner": surface.sdkOwner,
    "x-sdkwork-api-authority": surface.authorityName,
    "x-sdkwork-domain": route.owner,
    "x-sdkwork-resource": route.operationId.split(".").slice(0, -1).join("."),
    "x-sdkwork-request-context": surface.authMode === "api-key" ? "OpenApiRequestContext" : "AppRequestContext",
    "x-sdkwork-server-request-id": true,
  };

  if (usesJsonBody(route.method)) {
    operation.requestBody = {
      required: route.method !== "patch",
      content: {
        "application/json": {
          schema: { $ref: "#/components/schemas/ImageOperationCommand" },
        },
      },
    };
  }

  if (isListOperation(route)) {
    operation.parameters.push(
      queryParameter("page", { type: "integer", minimum: 1, default: 1 }),
      queryParameter("page_size", { type: "integer", minimum: 1, maximum: 200, default: 20 }),
      queryParameter("cursor", { type: "string" }),
      queryParameter("sort", { type: "string" }),
      queryParameter("q", { type: "string" }),
    );
  }

  return operation;
}

function securityRequirement(surface) {
  if (surface.authMode === "api-key") {
    return [{ ApiKey: [] }];
  }

  return [{ AuthToken: [], AccessToken: [] }];
}

function securitySchemes(surface) {
  if (surface.authMode === "api-key") {
    return {
      ApiKey: {
        type: "apiKey",
        in: "header",
        name: "X-API-Key",
        description: "SDKWork open-api key resolved by the server-side API key context.",
      },
    };
  }

  return {
    AuthToken: {
      type: "http",
      scheme: "bearer",
      bearerFormat: "JWT",
      description: "SDKWork auth token carried as Authorization: Bearer <auth_token>.",
    },
    AccessToken: {
      type: "apiKey",
      in: "header",
      name: "Access-Token",
      description: "SDKWork access isolation token.",
    },
  };
}

function buildSchemas() {
  return {
    ImageApiResult: {
      type: "object",
      additionalProperties: false,
      required: ["code", "message", "requestId", "data"],
      properties: {
        code: { type: "string" },
        message: { type: "string" },
        requestId: {
          type: "string",
          format: "uuid",
          description: "Server-owned request correlation id.",
        },
        data: {
          type: "object",
          additionalProperties: true,
        },
      },
    },
    ImageOperationCommand: {
      type: "object",
      additionalProperties: true,
      description: "Operation-specific image command payload defined by the owning sdkwork-image Rust module.",
    },
    ProblemDetail: {
      type: "object",
      additionalProperties: true,
      required: ["type", "title", "status"],
      properties: {
        type: { type: "string", format: "uri-reference" },
        title: { type: "string" },
        status: { type: "integer", minimum: 100, maximum: 599 },
        detail: { type: "string" },
        instance: { type: "string" },
        code: { type: "string" },
        traceId: { type: "string" },
        requestId: {
          type: "string",
          format: "uuid",
          description: "Server-owned request correlation id.",
        },
        errors: {
          type: "array",
          items: { $ref: "#/components/schemas/FieldError" },
        },
      },
    },
    FieldError: {
      type: "object",
      additionalProperties: false,
      required: ["field", "message"],
      properties: {
        field: { type: "string" },
        message: { type: "string" },
        code: { type: "string" },
      },
    },
  };
}

function jsonResponse(description, schemaRef) {
  return {
    description,
    content: {
      "application/json": {
        schema: { $ref: schemaRef },
      },
    },
  };
}

function problemResponse(description) {
  return {
    description,
    content: {
      "application/problem+json": {
        schema: { $ref: "#/components/schemas/ProblemDetail" },
      },
    },
  };
}

function extractPathParameters(path) {
  const parameters = [];
  for (const match of path.matchAll(/\{([^}]+)\}/g)) {
    parameters.push({
      name: match[1],
      in: "path",
      required: true,
      schema: { type: "string" },
    });
  }
  return parameters;
}

function queryParameter(name, schema) {
  return {
    name,
    in: "query",
    required: false,
    schema,
  };
}

function usesJsonBody(method) {
  return method === "post" || method === "put" || method === "patch";
}

function isListOperation(route) {
  return route.method === "get" && route.operationId.endsWith(".list");
}

function compareRoutes(left, right) {
  return left.path.localeCompare(right.path) || left.method.localeCompare(right.method);
}

function toLowerCamel(value) {
  const parts = String(value || "")
    .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
    .split(/[^a-zA-Z0-9]+/)
    .filter(Boolean);
  if (parts.length === 0) {
    return "api";
  }
  const [first, ...rest] = parts;
  return [
    first.charAt(0).toLowerCase() + first.slice(1),
    ...rest.map((part) => part.charAt(0).toUpperCase() + part.slice(1)),
  ].join("");
}

function toTitle(value) {
  return String(value || "")
    .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
    .replace(/[^a-zA-Z0-9]+/g, " ")
    .trim()
    .replace(/\s+/g, " ")
    .replace(/^./, (char) => char.toUpperCase());
}

function relativeForOpenApi(path) {
  return path.replace(imageRoot, "<sdkwork-image>").replace(/\\/g, "/");
}

function escapeRegExp(value) {
  return String(value).replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

await main();
