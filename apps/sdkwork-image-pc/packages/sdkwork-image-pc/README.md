# @sdkwork/image-pc

## Purpose

Image generation, editing tasks, and galleries.

## Placement

- Architecture: `pc-react`
- Domain: `content`
- Capability: `image`
- Status: `ready`

## Depends on

- `@sdkwork/image-contracts` for framework-independent media resource shapes
- `@sdkwork/ui-pc-react` for shared UI primitives and patterns
- `@sdkwork/core-pc-react` for SDK runtime, env, and session integration
- Lower-level image packages only

## Extraction sources

- `sdkwork-react-image`
- `sdkwork-cloud-portal`

## Next implementation steps

- Define package contracts under `src/contracts`
- Extract shared services under `src/services`
- Add UI composition surfaces under `src/components`
- Register routes or manifest metadata under `src/routes` or `src/manifests`

## SDKWork Documentation Contract

Domain: content
Capability: image
Package type: react-package
Status: ready

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

- `pnpm --filter @sdkwork/image-pc typecheck`

### Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`.
