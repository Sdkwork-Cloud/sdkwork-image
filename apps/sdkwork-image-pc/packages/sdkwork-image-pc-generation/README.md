# @sdkwork/image-pc-generation

Image generation workspace and generation runtime helpers for PC surfaces.

## Exports

- `@sdkwork/image-pc-generation/react` — `ImageGenerationWorkspaceView`, `ImageGenerationModePopup`, generation pages
- `@sdkwork/image-pc-generation/generation-*` — generation service, history, asset config helpers

Shared studio UI (`SdkworkGenerationModePopupBase`, `SdkworkStudioGenerationBottomBar`, credit formatting) lives in `@sdkwork/generations-pc-studio/react`.

## Workspace composition

`ImageGenerationWorkspaceView` delegates layout to `@sdkwork/generations-pc-workspace/generation-playground-workspace` with `modality="image"` and `bucket="images"`.

## Verification

```bash
pnpm --filter @sdkwork/image-pc-generation typecheck
```
