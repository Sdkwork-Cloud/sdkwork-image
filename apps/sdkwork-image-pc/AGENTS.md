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
