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

The wrapper calls `D:\javasource\spring-ai-plus\sdk\sdkwork-sdk-generator\bin\sdkgen.js` with `--standard-profile sdkwork-v3`.
