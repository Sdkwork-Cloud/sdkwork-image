# SDKWork Image SDKs

This directory hosts generated SDK-facing integration packages for `sdkwork-image`.

- `sdkwork-image-sdk/`: generated open SDK family for the `sdkwork-image-open-api` `/image/v3/api` authority
- `sdkwork-image-app-sdk/`: generated app SDK family for the `sdkwork-image-app-api` `/app/v3/api` authority
- `sdkwork-image-backend-sdk/`: generated backend SDK family for the `sdkwork-image-backend-api` `/backend/v3/api` authority

Run `node .\sdks\materialize-image-v3-openapi-boundaries.mjs` from the `sdkwork-image` root before SDK generation. The materializer reads the Rust image route catalog and writes deterministic authority and sdkgen OpenAPI inputs.
