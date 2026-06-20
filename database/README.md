# IMAGE Database Module

Canonical lifecycle assets for `sdkwork-image` per `DATABASE_FRAMEWORK_SPEC.md`.

- moduleId: `image`
- serviceCode: `IMAGE`
- tablePrefix: `image_`

## Commands

```bash
pnpm run db:materialize:contract
pnpm run db:validate
```

Legacy SQL: `crates/sdkwork-image-generation-repository-sqlx/migrations/*.sql` → `database/ddl/baseline/postgres/0001_image_legacy_baseline.sql`

Runtime bootstrap: `sdkwork-image-database-host` / `connect_and_bootstrap_image_database_from_env()`.
