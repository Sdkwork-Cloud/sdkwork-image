# @sdkwork/image-pc-generation

## Purpose

Unified generation history and task result provenance.

## Placement

- Workspace: `sdkwork-image`
- Architecture: `pc-react`
- Domain: `content`
- Capability: `generation`
- Status: `ready`

## Depends on

- `@sdkwork/ui-pc-react` for shared UI primitives and patterns
- `@sdkwork/image-contracts` for image media resource contracts
- Lower-level shared UI packages only

## Entry points

- `@sdkwork/image-pc-generation` for generation core, service, and workspace helpers
- `@sdkwork/image-pc-generation/react` for React UI, controller, intl, and page composition

## Extraction sources

- `sdkwork-react-generation-history`
- `sdkwork-studio`

## Next implementation steps

- Define package contracts under `src/contracts`
- Extract shared services under `src/services`
- Add UI composition surfaces under `src/components`
- Register routes or manifest metadata under `src/routes` or `src/manifests`

## SDKWork Documentation Contract

Domain: intelligence
Capability: generation
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

- `pnpm --filter @sdkwork/image-pc-generation typecheck`

### Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`.
