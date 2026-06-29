# PC Application Root - sdkwork-image

<!-- SDKWORK-AGENTS-GENERATED: v1 -->

## SDKWORK Soul

Read `../../../sdkwork-specs/SOUL.md` before executing tasks in this root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards

Canonical SDKWORK specs path from this root:

- `../../../sdkwork-specs/README.md`
- `../../../sdkwork-specs/SOUL.md`
- `../../../sdkwork-specs/AGENTS_SPEC.md`
- `../../../sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md`
- `../../../sdkwork-specs/APP_PC_REACT_UI_SPEC.md`
- `../../../sdkwork-specs/DESKTOP_APP_ARCHITECTURE_SPEC.md`
- `../../../sdkwork-specs/CODE_STYLE_SPEC.md`
- `../../../sdkwork-specs/NAMING_SPEC.md`
Build scripts, dev runners, and `pnpm clean` must follow `CODE_STYLE_SPEC.md` §7 (Build Source Integrity And Self-Healing). Git-tracked build-critical source files must be verified before builds and self-healed from git when missing; `clean` must not delete them.


Do not copy root standard text into this repository. If these relative paths do not resolve, stop and report the broken workspace layout.

## Application Identity

- Application: `sdkwork-image-pc`
- Architecture: PC browser/desktop
- Product: image
- Surface: pc

## Local Dictionary Structure

- `AGENTS.md`: local agent entrypoint and relative SDKWORK spec index.
- `CLAUDE.md`: Claude Code compatibility shim that points to `AGENTS.md`.
- `GEMINI.md`: Gemini CLI compatibility shim that points to `AGENTS.md`.
- `CODEX.md`: Codex compatibility shim that points to `AGENTS.md`.
- `sdkwork.app.config.json`: application manifest.
- `.sdkwork/`: local skills, plugins, and workspace metadata.
- `config/`: environment and runtime config templates.
- `src/`: root shell bootstrap and composition boundary.
- `packages/`: reusable PC runtime, shell, app, console, admin, and desktop host packages.

## Spec Resolution Order

1. Read this `AGENTS.md` and any nearer component-level `AGENTS.md`.
2. Read `sdkwork.app.config.json`.
3. Read local `specs/` when present.
4. Read `.sdkwork/README.md`, `.sdkwork/skills/`, `.sdkwork/plugins/` when relevant.
5. Read `../../../sdkwork-specs/README.md` and task-specific root specs.
6. Inspect implementation files only after the relevant dictionary entries are clear.

## Package Taxonomy

PC packages under `packages/`:

- `sdkwork-image-pc-core` - Core runtime, SDK client factories, TokenManager, appbase IAM
- `sdkwork-image-pc-commons` - Domain-neutral components, hooks, utilities
- `sdkwork-image-pc-shell` - App shell, layout, navigation, route composition
- `sdkwork-image-pc-<capability>` - App/user domain feature packages
- `sdkwork-image-pc-console-core` - Console runtime
- `sdkwork-image-pc-console-shell` - Console shell
- `sdkwork-image-pc-console-<capability>` - Console domain feature packages
- `sdkwork-image-pc-admin-core` - Admin runtime
- `sdkwork-image-pc-admin-shell` - Admin shell
- `sdkwork-image-pc-admin-<capability>` - Admin domain feature packages
- `sdkwork-image-pc-desktop` - Tauri/native host for desktop and tablet

## Verification

- `pnpm typecheck`
- `pnpm test`

## HTTP API Response Envelope

All L2+ `app-api`, `backend-api`, and SDKWork-owned business `open-api` HTTP contracts `MUST` follow `API_SPEC.md` section 4.5, section 14, and section 15:

- **Input:** typed request bodies, section 14.1 list/search/command input, `SdkWorkListQuery`, and `q` for free-text search.
- **Success output:** `SdkWorkApiResponse` with `{ "code": 0, "data": <payload>, "traceId": "<server-uuid>" }`.
- **Error output:** HTTP 4xx/5xx `application/problem+json` (`ProblemDetail`) with numeric `code` and `traceId`.
- Success `code` is numeric `int32`; HTTP 2xx JSON bodies `MUST` use `0` only. REST semantics remain on HTTP status (`201`, `202`, etc.).
- Platform error codes are numeric non-zero values per section 15.3 (`40001`, `40101`, `40401`, …).
- Single resource: `data.item`
- Lists: `data.items` + `data.pageInfo` (`PageInfo.mode` is `offset` or `cursor`)
- Commands: `data.accepted` plus optional `resourceId` / `status`
- Async accept (`202`): `data.operationId`, `data.status`, optional `pollUrl`

Vendor compatibility `open-api` routes that mirror upstream tool or provider wire (for example OpenAI `/v1/*`, Claude Code, Codex) `MAY` opt out only when every exempt operation declares `x-sdkwork-wire-protocol: external` and `x-sdkwork-external-protocol-id` per `API_SPEC.md` section 4.5.2. SDKWork-owned business `open-api` operations `MUST NOT` opt out.

Errors `MUST` use HTTP 4xx/5xx with `application/problem+json` (`ProblemDetail`) including required numeric `code` and `traceId`. Business failures `MUST NOT` use HTTP 2xx with non-zero `code`, string wire codes, `success`, or human `message`.

Forbidden legacy envelopes and fields: `PlusApiResult`, `AppbaseApiResult`, `StoreApiResult`, `SdkWorkResponse`, per-domain `*ApiResult`, wire field `requestId`, bare domain DTOs at the HTTP root, and top-level `{ items, pageInfo, traceId }` without `data`.

Handlers `MUST` serialize success and map errors through `sdkwork-web-framework` response mapping. Generated HTTP SDKs (`--standard-profile sdkwork-v3`) unwrap `data` by default and expose typed numeric `ProblemDetail.code` / `traceId` on errors; use `.raw` when the full envelope is required.

Before completing API contract, SDK generation, or frontend service work, run:

```bash
node <sdkwork-specs>/tools/check-api-response-envelope.mjs --workspace <workspace-root>
```

Authority: `sdkwork-specs/API_SPEC.md` section 4.5 and sections 14–16, `SDK_SPEC.md` section 4.2, `FRONTEND_SPEC.md`, `MIGRATION_SPEC.md` section 4.2.

## HTTP API Response Envelope

All L2+ `app-api`, `backend-api`, and SDKWork-owned `open-api` success JSON bodies `MUST` use `SdkWorkResponse` from `API_SPEC.md` §15:

- Envelope: `{ "data": <payload>, "requestId": "<server-uuid>" }`
- Single resource: `data.item`
- Lists: `data.items` + `data.pageInfo` (`PageInfo.mode` is `offset` or `cursor`)
- Commands: `data.accepted` plus optional `resourceId` / `status`
- Async accept (`202`): `data.operationId`, `data.status`, optional `pollUrl`

Errors `MUST` use HTTP 4xx/5xx with `application/problem+json` (`ProblemDetail`). Business failures `MUST NOT` use HTTP 2xx with `success`, `code`, or `message`.

Forbidden legacy envelopes: `PlusApiResult`, `AppbaseApiResult`, `StoreApiResult`, per-domain `*ApiResult`, bare domain DTOs at the HTTP root, and top-level `{ items, pageInfo, requestId }` without `data`.

Handlers `MUST` serialize success and map errors through `sdkwork-web-framework` response mapping. Do not hand-build envelopes in controllers or route handlers.

Generated HTTP SDKs (`--standard-profile sdkwork-v3`) unwrap `data` by default; use `.raw` only when correlation headers or the full envelope are required.

Before completing API contract or handler work, run:

```bash
node <sdkwork-specs>/tools/check-api-response-envelope.mjs --workspace <workspace-root>
```

Authority: `sdkwork-specs/API_SPEC.md` §15–§16, `WEB_FRAMEWORK_SPEC.md`, `SDK_SPEC.md` §4.1, `MIGRATION_SPEC.md` §API Response Envelope Migration.
