# SDKWork Image Generation MCP Service

Protocol adapter that exposes the unified image generation service to MCP clients.

Public MCP surface:

- Tools: `image.generate`, `image.retrieve`, `image.cancel`, `image.capabilities`
- Resources: `sdkwork://image/generation/capabilities`, `sdkwork://image/generation/vendors`
- Prompt: `image.generation.request`
- Transports: stdio and MCP Streamable HTTP with SSE delivery

The crate depends only on `sdkwork-image-generation-service`. Provider adapters, generated SDKs,
credentials, HTTP authentication, listener binding, and deployment topology remain composition-root
responsibilities. Mount the returned Streamable HTTP service behind authentication, authorization,
origin validation, request limits, tracing, and graceful shutdown.
