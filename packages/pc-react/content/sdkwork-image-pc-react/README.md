# @sdkwork/image-pc-react

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
