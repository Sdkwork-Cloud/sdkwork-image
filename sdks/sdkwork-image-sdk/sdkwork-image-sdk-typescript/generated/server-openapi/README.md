# sdkwork-image-sdk

Professional TypeScript SDK for SDKWork API.

## Installation

```bash
npm install @sdkwork/image-sdk
# or
yarn add @sdkwork/image-sdk
# or
pnpm add @sdkwork/image-sdk
```

## Quick Start

```typescript
import { SdkworkCustomClient } from '@sdkwork/image-sdk';

const client = new SdkworkCustomClient({
  baseUrl: 'http://localhost:8080',
  timeout: 30000,
});

// Mode A: API Key (recommended for server-to-server calls)
client.setApiKey('your-api-key');

// Use the SDK
const body = {};
const result = await client.imageCompat.compat.openai.images.generate(body);
```

## Authentication Modes (Mutually Exclusive)

Choose exactly one mode for the same client instance.

### Mode A: API Key

```typescript
const client = new SdkworkCustomClient({ baseUrl: 'http://localhost:8080' });
client.setApiKey('your-api-key');
// Sends: X-API-Key: <apiKey>
```

### Mode B: Dual Token

```typescript
const client = new SdkworkCustomClient({ baseUrl: 'http://localhost:8080' });
client.setAuthToken('your-auth-token');
client.setAccessToken('your-access-token');
// Sends:
// Authorization: Bearer <authToken>
// Access-Token: <accessToken>
```

> Do not call `setApiKey(...)` together with `setAuthToken(...)` + `setAccessToken(...)` on the same client.

## Configuration (Non-Auth)

```typescript
import { SdkworkCustomClient } from '@sdkwork/image-sdk';

const client = new SdkworkCustomClient({
  baseUrl: 'http://localhost:8080',
  timeout: 30000, // Request timeout in ms
  headers: {      // Custom headers
    'X-Custom-Header': 'value',
  },
});
```

## API Modules

- `client.imageCompat` - image_compat API

## Usage Examples

### image_compat

```typescript
// Compat openai images generate.
const body = {};
const result = await client.imageCompat.compat.openai.images.generate(body);
```

## Error Handling

```typescript
import { SdkworkCustomClient, NetworkError, TimeoutError, AuthenticationError } from '@sdkwork/image-sdk';

try {
  const body = {};
  const result = await client.imageCompat.compat.openai.images.generate(body);
} catch (error) {
  if (error instanceof AuthenticationError) {
    console.error('Authentication failed:', error.message);
  } else if (error instanceof TimeoutError) {
    console.error('Request timed out:', error.message);
  } else if (error instanceof NetworkError) {
    console.error('Network error:', error.message);
  } else {
    throw error;
  }
}
```

## Publishing

This SDK includes cross-platform publish scripts in `bin/`:
- `bin/publish-core.mjs`
- `bin/publish.sh`
- `bin/publish.ps1`

### Check

```bash
./bin/publish.sh --action check
```

### Publish

```bash
./bin/publish.sh --action publish --channel release
```

```powershell
.\bin\publish.ps1 --action publish --channel test --dry-run
```

> Set `NPM_TOKEN` (and optional `NPM_REGISTRY_URL`) before release publish.

## License

MIT

## Regeneration Contract

- Generator-owned files are tracked in `.sdkwork/sdkwork-generator-manifest.json`.
- Each run also writes `.sdkwork/sdkwork-generator-changes.json` so automation can inspect created, updated, deleted, unchanged, scaffolded, and backed-up files plus the classified impact areas, verification plan, and execution decision for the latest generation.
- Apply mode also writes `.sdkwork/sdkwork-generator-report.json` with the full execution report, including `schemaVersion`, `generator`, stable artifact paths, and the execution handoff commands that match CLI `--json` output.
- CLI JSON output also includes an execution handoff with concrete next commands, including reviewed apply commands for dry-run flows.
- Put hand-written wrappers, adapters, and orchestration in `custom/`.
- Files scaffolded under `custom/` are created once and preserved across regenerations.
- If a generated-owned file was modified locally, its previous content is copied to `.sdkwork/manual-backups/` before overwrite or removal.
