# APIs

Author-owned API contracts and API source inputs for all API kinds.

## Purpose

Stores OpenAPI surfaces, RPC/proto contracts, async/event API manifests, API examples, API changelogs, and API validation inputs.

## Owner

Repository maintainers.

## Allowed Content

- `open-api/<domain>/openapi.yaml` and supporting files
- `app-api/<domain>/openapi.yaml` and supporting files
- `backend-api/<domain>/openapi.yaml` and supporting files
- Route schemas, examples, changelogs, tests

## Forbidden Content

- Generated SDK transport output
- Generated SDK control-plane `.sdkwork/` files
- Implementation code
- Runtime secrets or credentials

## Related Specs

- `../sdkwork-specs/API_SPEC.md`
- `../sdkwork-specs/SDK_SPEC.md`
- `../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md`
