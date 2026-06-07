# SDKWork Image App SDK

This SDK family is generated from the `sdkwork-image-app-api` authority contract for `/app/v3/api`.

## Contract

- SDK family: `sdkwork-image-app-sdk`
- API authority: `sdkwork-image-app-api`
- API prefix: `/app/v3/api`
- Audience: app, desktop, mobile, H5, and user-facing image clients
- Auth mode: `Authorization: Bearer <auth_token>` plus `Access-Token: <access_token>` for protected operations
- Request context: server middleware resolves `AppRequestContext`; clients must not send `X-Request-Id`

## Generation

Run from `sdkwork-image`:

```powershell
node .\sdks\materialize-image-v3-openapi-boundaries.mjs
.\sdks\sdkwork-image-app-sdk\bin\generate-sdk.ps1
```

The wrapper calls `..\sdkwork-sdk-generator\bin\sdkgen.js` with `--standard-profile sdkwork-v3`.

## SDKWork Documentation Contract

Domain: content
Capability: image-app-sdk
Package type: sdk-family
Status: standard

### Public API

Public exports are declared in `specs/component.spec.json` under `contracts.publicExports`.

### Required SDK Surface

- None declared in `specs/component.spec.json`.

### Configuration

Configuration keys and runtime entrypoints are declared in `specs/component.spec.json`.

### SaaS/Private/Local Behavior

This module follows the canonical standards linked from `specs/component.spec.json`, including deployment and runtime configuration rules where applicable.

### Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module.

### Extension Points

Extension points are limited to declared public exports, runtime entrypoints, SDK clients, events, and config keys.

### Verification

- `powershell -NoProfile -Command "Get-Content specs/component.spec.json -Raw | ConvertFrom-Json | Out-Null"`

### Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`.
