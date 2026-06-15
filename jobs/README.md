# Jobs

Job definitions, schedules, queue bindings, batch descriptors, and maintenance runbooks.

## Purpose

Stores non-request/response job schedules, queue bindings, batch descriptors, and maintenance runbooks.

## Owner

Repository maintainers.

## Allowed Content

- Schedule definitions
- Queue bindings
- Batch descriptors
- Maintenance runbooks
- Non-Rust job packages

## Forbidden Content

- Rust worker implementations (belong in `crates/sdkwork-<domain>-<capability>-worker/`)
- Runtime secrets or credentials

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
