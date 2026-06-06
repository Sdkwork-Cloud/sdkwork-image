# @sdkwork/generation-pc-react

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

- `@sdkwork/generation-pc-react` for generation core, service, and workspace helpers
- `@sdkwork/generation-pc-react/react` for React UI, controller, intl, and page composition

## Extraction sources

- `sdkwork-react-generation-history`
- `sdkwork-studio`

## Next implementation steps

- Define package contracts under `src/contracts`
- Extract shared services under `src/services`
- Add UI composition surfaces under `src/components`
- Register routes or manifest metadata under `src/routes` or `src/manifests`
