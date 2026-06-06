# SDKWork Image SDK

This SDK family is generated from the `sdkwork-image-open-api` authority contract for `/image/v3/api`.

## Contract

- SDK family: `sdkwork-image-sdk`
- API authority: `sdkwork-image-open-api`
- API prefix: `/image/v3/api`
- Audience: external integrations, provider-compatible image clients, and public image automation
- Auth mode: `X-API-Key` for protected open-api operations
- Request context: server middleware resolves `OpenApiRequestContext`; clients must not send `X-Request-Id`

## Generation

Run from `sdkwork-image`:

```powershell
node .\sdks\materialize-image-v3-openapi-boundaries.mjs
.\sdks\sdkwork-image-sdk\bin\generate-sdk.ps1
```

The wrapper calls `D:\javasource\spring-ai-plus\sdk\sdkwork-sdk-generator\bin\sdkgen.js` with `--standard-profile sdkwork-v3`.
