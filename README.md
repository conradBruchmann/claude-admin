# ClaudeAdmin

A web-based admin console for managing your [Claude Code](https://claude.com/claude-code) configuration.

Once you work with more than a handful of projects, keeping track of skills, rules, memory files, MCP servers, and settings across `~/.claude/` becomes painful. ClaudeAdmin gives you a single UI to manage it all — no more hand-editing JSON and YAML.

> **Note:** This is an independent community project. Not affiliated with or endorsed by Anthropic.

![ClaudeAdmin Dashboard](https://github.com/conradBruchmann/claude-admin/blob/master/docs/screenshot.png?raw=true)

## Features

- **Dashboard** — Overview of your global and project-level configuration
- **Projects** — Browse detected projects, edit CLAUDE.md, manage project-specific rules, skills, and memory
- **Skills** — Create, edit, and browse global skills (YAML frontmatter + markdown)
- **Skill Browser** — Discover and install community skills with one click
- **Rules** — Manage global and project-level rules
- **Memory** — View and edit per-project memory files (MEMORY.md + topic files)
- **MCP Servers** — Full MCP server management: add, edit, delete, and health-check
- **MCP Browser** — Discover and install popular MCP servers (databases, APIs, tools)
- **Settings** — Edit global settings and hooks
- **Permissions** — Review tool permissions and config health
- **Plans** — Manage plan files
- **Sessions** — Browse session history
- **Analytics** — Usage metrics and insights

## Architecture

```
┌─────────────────────┐     ┌──────────────────────┐
│  claude-admin-       │     │  claude-admin-        │
│  backend             │◄────│  frontend             │
│  Rust + Axum 0.7     │     │  Leptos 0.6 CSR/WASM  │
│  Port 9022           │     │  Port 9023 (dev)       │
└──────────┬──────────┘     └──────────────────────┘
           │                         │
           │    claude-admin-shared  │
           │    (shared types)       │
           │                         │
           ▼                         │
   ~/.claude/  (filesystem)          │
                                     │
   In production, the WASM frontend  │
   is embedded into the backend      │
   binary via rust-embed.            │
```

- **No database** — reads from and writes to the filesystem (`~/.claude/`)
- **Automatic backups** — created in `~/.claude/backups/` before every write operation, timestamped per file
- **Claude API optional** — works fully without `ANTHROPIC_API_KEY`
- **Single binary** — production build embeds the frontend via `rust-embed`

## Security

ClaudeAdmin runs **locally on your machine**. It is designed for single-user use on a development workstation.

- Reads and writes only under `~/.claude/` and `~/.claude.json`
- No telemetry, no analytics, no remote calls (except optional Anthropic API if configured)
- No authentication — **do not expose to the internet** without a reverse proxy and auth layer

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Trunk](https://trunkrs.dev/) for WASM builds: `cargo install trunk`
- WASM target: `rustup target add wasm32-unknown-unknown`

### Development

```bash
# Clone and enter the project
git clone https://github.com/conradbruchmann/claude-admin.git
cd claude-admin

# Start the backend
cargo run -p claude-admin-backend

# In a separate terminal: start the frontend with hot-reload
cd claude-admin-frontend && trunk serve --port 9023
```

Backend: `http://localhost:9022` — Frontend: `http://localhost:9023`

### Production Build

```bash
# Build the WASM frontend
cd claude-admin-frontend && trunk build --release && cd ..

# Build the backend (embeds the frontend)
cargo build --release -p claude-admin-backend

# Run the single binary
./target/release/claude-admin-backend
```

Pre-built binaries for Linux, macOS, and Windows are available on the [Releases](https://github.com/conradBruchmann/claude-admin/releases) page.

### macOS Installation

Download the `.dmg` from the latest release, open it, and drag **ClaudeAdmin.app** into `/Applications`.

Since the app is not signed with an Apple Developer certificate, macOS Gatekeeper will block it on first launch. To allow it, run:

```bash
xattr -dr com.apple.quarantine /Applications/ClaudeAdmin.app
```

Then double-click the app — it starts the server and opens `http://localhost:9022` in your browser. The DMG contains a Universal Binary that runs natively on both Intel and Apple Silicon Macs.

### Windows Installation

Download the `ClaudeAdmin-*-Setup.exe` from the latest release and run the installer. It installs to your user profile (no admin rights needed), creates Start Menu and Desktop shortcuts, and registers in "Apps & Features" for clean uninstall.

After installation, launch ClaudeAdmin from the Start Menu or Desktop — it starts the server and opens `http://localhost:9022` in your browser.

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
| `~/.claude/backups/` | Automatic backups (timestamped) |

## License

[MIT](LICENSE) — Copyright (c) 2024-2026 Conrad Bruchmann, BRUCHMANN [TEC] INNOVATION GMBH
