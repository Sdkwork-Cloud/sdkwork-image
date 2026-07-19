# Integrating Media Generation MCP Services

The media MCP crates are libraries. A host injects the existing generation service port and chooses
stdio or Streamable HTTP without constructing provider adapters inside the MCP layer.

## Stdio

```rust
use std::sync::Arc;

use sdkwork_image_generation_mcp_service::{ImageGenerationMcpService, serve_stdio};

let generation_service = Arc::new(build_image_generation_service()?);
let mcp = ImageGenerationMcpService::new(generation_service);
let running = serve_stdio(mcp).await?;
running.waiting().await?;
```

Use the corresponding constructor and `serve_stdio` export for video, voice, music, or sound
effects.

## Streamable HTTP And SSE

```rust
use std::sync::Arc;

use axum::Router;
use rmcp::transport::streamable_http_server::StreamableHttpServerConfig;
use sdkwork_image_generation_mcp_service::{
    ImageGenerationMcpService,
    streamable_http_service,
};

let generation_service = Arc::new(build_image_generation_service()?);
let mcp = ImageGenerationMcpService::new(generation_service);
let config = StreamableHttpServerConfig::default()
    .with_allowed_hosts(["agents.internal.example"])
    .with_allowed_origins(["https://agents.example"]);
let app = Router::new().nest_service("/mcp/image", streamable_http_service(mcp, config));
```

The host must set its deployment host/origin policy explicitly and mount authentication,
authorization, tenant context, request limits, tracing, cancellation, and graceful shutdown around
the MCP service. In stateful mode, valid MCP requests receive `text/event-stream`; clients must send
both `application/json` and `text/event-stream` in `Accept`.

Do not add a second `/sse` compatibility endpoint. SSE delivery is already part of MCP Streamable
HTTP and the separate legacy transport is not part of this architecture.

## Task Context

Image, video, voice, and music generation can return asynchronous provider tasks. Their MCP response
contains `taskHandle`; subsequent retrieve/cancel calls accept only that handle. Dispatch plans do
not cross the public MCP boundary.

The default task store is bounded and process-local. A long-running or horizontally scaled host
should inject the package's `*GenerationMcpTaskStore` port backed by its approved durable runtime.
Task handles are routing references, not credentials; every call still requires normal host-level
authorization.

## Agent Discovery

Agents can inspect these resources before selecting vendor-specific parameters:

- `sdkwork://image/generation/vendors`
- `sdkwork://video/generation/vendors`
- `sdkwork://voice/generation/vendors`
- `sdkwork://music/generation/vendors`
- `sdkwork://sound-effect/generation/vendors`

All generation inputs place non-portable fields under `vendorParameters` with both `schema` and
`values`. No MCP client should infer a provider SDK DTO or route method.
