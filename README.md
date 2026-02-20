# ClaudeAdmin

A web-based admin console for managing your [Claude Code](https://claude.com/claude-code) configuration. Manage Skills, Rules, Memory, CLAUDE.md files, Settings, MCP Servers, Plans, and more — all from a single UI.

> **Note:** This is a community project and is not affiliated with or endorsed by Anthropic.

## Features

- **Dashboard** — Overview of your global and project-level Claude Code configuration
- **Projects** — Browse detected projects, edit CLAUDE.md, manage project-specific rules, skills, and memory
- **Skills** — Create, edit, and browse global skills (YAML frontmatter + markdown)
- **Rules** — Manage global and project-level rules
- **Memory** — View and edit per-project memory files (MEMORY.md + topic files)
- **Settings** — Edit global settings, hooks, and MCP server configuration
- **MCP Servers** — Full MCP server management: add, edit, delete, and health-check servers
- **MCP Browser** — Discover and install popular MCP servers (databases, APIs, tools) with one click
- **Plans** — Manage plan files
- **Permissions** — Review and manage tool permissions and config health
- **Skill Browser** — Discover and install community skills
- **Sessions** — Browse session history
- **Analytics** — Usage metrics and insights

## Architecture

```
claude-admin-backend/    Rust + Axum 0.7 (Port 9022)
claude-admin-frontend/   Leptos 0.6 CSR / WASM (Port 9023 dev)
claude-admin-shared/     Shared types for API contract
```

- **No database** — reads from and writes to the filesystem (`~/.claude/`)
- **Automatic backups** — created in `~/.claude/backups/` before every write
- **Claude API optional** — works without `ANTHROPIC_API_KEY`
- **Production bundle** — frontend is embedded into the backend binary via `rust-embed`

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Trunk](https://trunkrs.dev/) for WASM builds: `cargo install trunk`
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

## Getting Started

```bash
# Clone the repository
git clone https://github.com/conradbruchmann/claude-admin.git
cd claude-admin

# Start the backend
cargo run -p claude-admin-backend

# Start the frontend (separate terminal)
cd claude-admin-frontend && trunk serve --port 9023

# Or build everything
cargo build --workspace
```

The backend runs on `http://localhost:9022`, the frontend dev server on `http://localhost:9023`.

## Configuration Paths

ClaudeAdmin reads and writes the standard Claude Code configuration:

| Path | Description |
|------|-------------|
| `~/.claude.json` | Project registry, MCP servers |
| `~/.claude/settings.json` | Global settings, hooks |
| `~/.claude/skills/` | Global skills |
| `~/.claude/rules/` | Global rules |
| `~/.claude/plans/` | Plan files |
| `~/.claude/projects/<encoded>/memory/` | Per-project memory |

## License

[MIT](LICENSE) — Copyright (c) 2024-2026 Conrad Bruchmann, BRUCHMANN [TEC] INNOVATION GMBH
