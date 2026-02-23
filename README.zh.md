🌍 [English](README.md) | [Deutsch](README.de.md) | [Nederlands](README.nl.md) | [Português](README.pt.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | **中文** | [Polski](README.pl.md) | [Türkçe](README.tr.md)

# ClaudeAdmin

一个基于 Web 的管理控制台，用于管理您的 [Claude Code](https://claude.com/claude-code) 配置。

当您同时处理多个项目时，在 `~/.claude/` 中跟踪技能、规则、记忆文件、MCP 服务器和设置会变得十分繁琐。ClaudeAdmin 为您提供单一界面来统一管理这一切——无需再手动编辑 JSON 和 YAML 文件。

> **注意：** 这是一个独立的社区项目，与 Anthropic 无任何关联，亦未获得 Anthropic 的认可。

![ClaudeAdmin Dashboard](https://github.com/conradBruchmann/claude-admin/blob/master/docs/screenshot.png?raw=true)

## 功能特性

- **仪表板** — 全局及项目级配置概览
- **项目** — 浏览已检测到的项目，编辑 CLAUDE.md，管理项目专属的规则、技能和记忆
- **技能** — 创建、编辑和浏览全局技能（YAML 前置元数据 + Markdown）
- **技能浏览器** — 一键发现并安装社区技能
- **规则** — 管理全局及项目级规则
- **记忆** — 查看和编辑每个项目的记忆文件（MEMORY.md 及主题文件）
- **MCP 服务器** — 完整的 MCP 服务器管理：添加、编辑、删除及健康检查
- **MCP 浏览器** — 发现并安装热门 MCP 服务器（数据库、API、工具等）
- **设置** — 编辑全局设置和钩子
- **权限** — 查看工具权限和配置健康状态
- **计划** — 管理计划文件
- **会话** — 浏览会话历史
- **分析** — 使用指标与洞察

## 架构

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

- **无数据库** — 直接从文件系统（`~/.claude/`）读写数据
- **自动备份** — 每次写操作前在 `~/.claude/backups/` 创建带时间戳的备份
- **Claude API 可选** — 无需 `ANTHROPIC_API_KEY` 也可完整运行
- **单一二进制文件** — 生产构建通过 `rust-embed` 将前端嵌入其中

## 安全性

ClaudeAdmin **在您的本地机器上运行**，专为开发工作站的单用户使用而设计。

- 仅读写 `~/.claude/` 和 `~/.claude.json` 目录下的文件
- 无遥测、无分析、无远程调用（已配置的 Anthropic API 除外）
- 无身份验证——**请勿在没有反向代理和认证层的情况下将其暴露于公网**

## 快速开始

### 前置条件

- [Rust](https://rustup.rs/)（稳定版）
- 用于 WASM 构建的 [Trunk](https://trunkrs.dev/)：`cargo install trunk`
- WASM 目标：`rustup target add wasm32-unknown-unknown`

### 开发环境

```bash
# 克隆并进入项目目录
git clone https://github.com/conradbruchmann/claude-admin.git
cd claude-admin

# 启动后端
cargo run -p claude-admin-backend

# 在另一个终端中：启动支持热重载的前端
cd claude-admin-frontend && trunk serve --port 9023
```

后端：`http://localhost:9022` — 前端：`http://localhost:9023`

### 生产构建

```bash
# 构建 WASM 前端
cd claude-admin-frontend && trunk build --release && cd ..

# 构建后端（嵌入前端）
cargo build --release -p claude-admin-backend

# 运行单一二进制文件
./target/release/claude-admin-backend
```

适用于 Linux、macOS 和 Windows 的预构建二进制文件可在 [Releases](https://github.com/conradBruchmann/claude-admin/releases) 页面下载。

### macOS 安装

从最新发布版本下载 `.dmg` 文件，打开后将 **ClaudeAdmin.app** 拖入 `/Applications` 文件夹。

由于该应用未使用 Apple 开发者证书签名，macOS Gatekeeper 将在首次启动时阻止它。请运行以下命令以允许其运行：

```bash
xattr -dr com.apple.quarantine /Applications/ClaudeAdmin.app
```

然后双击应用——它将启动服务器并在浏览器中打开 `http://localhost:9022`。DMG 包含通用二进制文件，可在 Intel 和 Apple Silicon Mac 上原生运行。

### Windows 安装

从最新发布版本下载 `ClaudeAdmin-*-Setup.exe` 并运行安装程序。安装至用户配置文件目录（无需管理员权限），自动创建开始菜单和桌面快捷方式，并在"应用和功能"中注册以便干净卸载。

安装完成后，从开始菜单或桌面启动 ClaudeAdmin——它将启动服务器并在浏览器中打开 `http://localhost:9022`。

## 配置路径

ClaudeAdmin 读写标准的 Claude Code 配置：

| 路径 | 描述 |
|------|------|
| `~/.claude.json` | 项目注册表、MCP 服务器 |
| `~/.claude/settings.json` | 全局设置、钩子 |
| `~/.claude/skills/` | 全局技能 |
| `~/.claude/rules/` | 全局规则 |
| `~/.claude/plans/` | 计划文件 |
| `~/.claude/projects/<encoded>/memory/` | 每个项目的记忆 |
| `~/.claude/backups/` | 自动备份（带时间戳） |

## 许可证

[MIT](LICENSE) — Copyright (c) 2024-2026 Conrad Bruchmann, BRUCHMANN [TEC] INNOVATION GMBH
