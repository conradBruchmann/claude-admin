🌍 [English](README.md) | [Deutsch](README.de.md) | [Nederlands](README.nl.md) | [Português](README.pt.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | **日本語** | [한국어](README.ko.md) | [中文](README.zh.md) | [Polski](README.pl.md) | [Türkçe](README.tr.md)

# ClaudeAdmin

[Claude Code](https://claude.com/claude-code) の設定を管理するウェブベースの管理コンソールです。

複数のプロジェクトを扱うようになると、`~/.claude/` 全体にわたるスキル、ルール、メモリファイル、MCP サーバー、設定の管理が煩雑になります。ClaudeAdmin はそれらをすべて一元管理できる UI を提供します — JSON や YAML を手動で編集する手間はもうありません。

> **注意:** これは独立したコミュニティプロジェクトです。Anthropic との提携や推薦はありません。

![ClaudeAdmin Dashboard](https://github.com/conradBruchmann/claude-admin/blob/master/docs/screenshot.png?raw=true)

## 機能

- **ダッシュボード** — グローバルおよびプロジェクトレベルの設定の概要
- **プロジェクト** — 検出されたプロジェクトの閲覧、CLAUDE.md の編集、プロジェクト固有のルール・スキル・メモリの管理
- **スキル** — グローバルスキルの作成・編集・閲覧（YAML フロントマター + マークダウン）
- **スキルブラウザー** — ワンクリックでコミュニティスキルを検索・インストール
- **ルール** — グローバルおよびプロジェクトレベルのルールの管理
- **メモリ** — プロジェクトごとのメモリファイルの表示・編集（MEMORY.md + トピックファイル）
- **MCP サーバー** — MCP サーバーの完全な管理：追加・編集・削除・ヘルスチェック
- **MCP ブラウザー** — 人気の MCP サーバーの検索・インストール（データベース、API、ツール）
- **設定** — グローバル設定とフックの編集
- **パーミッション** — ツールのパーミッションと設定の健全性の確認
- **プラン** — プランファイルの管理
- **セッション** — セッション履歴の閲覧
- **アナリティクス** — 使用状況メトリクスとインサイト

## アーキテクチャ

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

- **データベース不要** — ファイルシステム（`~/.claude/`）への読み書きのみ
- **自動バックアップ** — すべての書き込み操作前に `~/.claude/backups/` へファイルごとのタイムスタンプ付きで作成
- **Claude API はオプション** — `ANTHROPIC_API_KEY` なしで完全に動作
- **シングルバイナリ** — プロダクションビルドは `rust-embed` でフロントエンドを埋め込み

## セキュリティ

ClaudeAdmin は**あなたのマシン上でローカルに**動作します。開発用ワークステーションでの単一ユーザー利用を想定して設計されています。

- `~/.claude/` および `~/.claude.json` 配下のみへの読み書き
- テレメトリなし、アナリティクスなし、外部通信なし（設定済みの場合の Anthropic API オプションを除く）
- 認証なし — リバースプロキシと認証レイヤーなしに**インターネットへ公開しないでください**

## はじめに

### 前提条件

- [Rust](https://rustup.rs/)（stable）
- WASM ビルド用 [Trunk](https://trunkrs.dev/)：`cargo install trunk`
- WASM ターゲット：`rustup target add wasm32-unknown-unknown`

### 開発

```bash
# リポジトリをクローンしてディレクトリに入る
git clone https://github.com/conradbruchmann/claude-admin.git
cd claude-admin

# バックエンドを起動
cargo run -p claude-admin-backend

# 別のターミナルで：ホットリロード付きフロントエンドを起動
cd claude-admin-frontend && trunk serve --port 9023
```

バックエンド：`http://localhost:9022` — フロントエンド：`http://localhost:9023`

### プロダクションビルド

```bash
# WASM フロントエンドをビルド
cd claude-admin-frontend && trunk build --release && cd ..

# バックエンドをビルド（フロントエンドを埋め込み）
cargo build --release -p claude-admin-backend

# シングルバイナリを実行
./target/release/claude-admin-backend
```

Linux、macOS、Windows 向けのビルド済みバイナリは [Releases](https://github.com/conradBruchmann/claude-admin/releases) ページからダウンロードできます。

### macOS インストール

最新リリースから `.dmg` をダウンロードし、開いて **ClaudeAdmin.app** を `/Applications` にドラッグします。

アプリは Apple Developer 証明書で署名されていないため、初回起動時に macOS Gatekeeper によってブロックされます。許可するには次のコマンドを実行してください：

```bash
xattr -dr com.apple.quarantine /Applications/ClaudeAdmin.app
```

その後アプリをダブルクリックすると、サーバーが起動し `http://localhost:9022` がブラウザで開きます。DMG には Intel および Apple Silicon Mac の両方でネイティブ動作するユニバーサルバイナリが含まれています。

### Windows インストール

最新リリースから `ClaudeAdmin-*-Setup.exe` をダウンロードしてインストーラーを実行します。ユーザープロファイルにインストールされ（管理者権限不要）、スタートメニューとデスクトップにショートカットが作成され、「アプリと機能」にアンインストール用として登録されます。

インストール後、スタートメニューまたはデスクトップから ClaudeAdmin を起動すると、サーバーが起動し `http://localhost:9022` がブラウザで開きます。

## 設定パス

ClaudeAdmin は標準の Claude Code 設定を読み書きします：

| パス                                   | 説明                                   |
| -------------------------------------- | -------------------------------------- |
| `~/.claude.json`                       | プロジェクトレジストリ、MCP サーバー   |
| `~/.claude/settings.json`              | グローバル設定、フック                 |
| `~/.claude/skills/`                    | グローバルスキル                       |
| `~/.claude/rules/`                     | グローバルルール                       |
| `~/.claude/plans/`                     | プランファイル                         |
| `~/.claude/projects/<encoded>/memory/` | プロジェクトごとのメモリ               |
| `~/.claude/backups/`                   | 自動バックアップ（タイムスタンプ付き） |

## ライセンス

[MIT](LICENSE) — Copyright (c) 2024-2026 Conrad Bruchmann, BRUCHMANN [TEC] INNOVATION GMBH
