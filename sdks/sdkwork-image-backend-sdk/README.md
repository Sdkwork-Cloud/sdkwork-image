# SDKWork Image Backend SDK

This SDK family is generated from the `sdkwork-image-backend-api` authority contract for `/backend/v3/api`.

## Contract

- SDK family: `sdkwork-image-backend-sdk`
- API authority: `sdkwork-image-backend-api`
- API prefix: `/backend/v3/api`
- Audience: backend consoles, operators, image control-plane integrations, and admin automation
- Auth mode: `Authorization: Bearer <auth_token>` plus `Access-Token: <access_token>`
- Request context: server middleware resolves `AppRequestContext`; clients must not send `X-Request-Id`

## Generation

Run from `sdkwork-image`:

```powershell
node .\sdks\materialize-image-v3-openapi-boundaries.mjs
.\sdks\sdkwork-image-backend-sdk\bin\generate-sdk.ps1
```

The wrapper calls `D:\javasource\spring-ai-plus\sdk\sdkwork-sdk-generator\bin\sdkgen.js` with `--standard-profile sdkwork-v3`.
