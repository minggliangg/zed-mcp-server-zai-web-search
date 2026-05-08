# Z.AI Web Search — Zed Extension

Zed extension that integrates [Z.AI's Web Search MCP server](https://docs.z.ai/devpack/mcp/search-mcp-server.md) for web search capabilities directly in the Zed editor.

## Setup

1. Get an API key from the [Z.AI Console](https://z.ai/manage-apikey/apikey-list)
2. Install this extension in Zed
3. Add to your Zed `settings.json`:

```json
{
  "context_servers": {
    "zed-mcp-server-zai-web-search": {
      "settings": {
        "api_key": "your-api-key"
      }
    }
  }
}
```

## How it works

The extension uses `mcp-remote` to bridge Zed's stdio-based MCP transport to Z.AI's remote HTTP endpoint. No local Node.js server code needed.

## Development

```bash
cargo check
cargo build --target wasm32-wasip1 --release
```

Install as a dev extension in Zed via `Cmd+Shift+P` → "install dev extension" → select project root.
