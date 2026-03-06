🌍 [English](README.md) | **Deutsch** | [Nederlands](README.nl.md) | [Português](README.pt.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [中文](README.zh.md) | [Polski](README.pl.md) | [Türkçe](README.tr.md)

# ClaudeAdmin

Eine webbasierte Admin-Konsole zur Verwaltung deiner [Claude Code](https://claude.com/claude-code)-Konfiguration.

Sobald man mit mehr als einer Handvoll Projekte arbeitet, wird das Überblicken von Skills, Rules, Memory-Dateien, MCP-Servern und Einstellungen in `~/.claude/` mühsam. ClaudeAdmin bietet eine zentrale Oberfläche, um alles zu verwalten — kein manuelles Bearbeiten von JSON und YAML mehr.

> **Hinweis:** Dies ist ein unabhängiges Community-Projekt. Nicht verbunden mit oder von Anthropic empfohlen.

![ClaudeAdmin Dashboard](https://github.com/conradBruchmann/claude-admin/blob/master/docs/screenshot.png?raw=true)

## Funktionen

- **Dashboard** — Übersicht über deine globale und projektbezogene Konfiguration
- **Projekte** — Erkannte Projekte durchsuchen, CLAUDE.md bearbeiten, projektspezifische Rules, Skills und Memory verwalten
- **Skills** — Globale Skills erstellen, bearbeiten und durchsuchen (YAML-Frontmatter + Markdown)
- **Skill-Browser** — Community-Skills entdecken und mit einem Klick installieren
- **Rules** — Globale und projektbezogene Rules verwalten
- **Memory** — Projektbezogene Memory-Dateien anzeigen und bearbeiten (MEMORY.md + Themendateien)
- **MCP-Server** — Vollständige MCP-Server-Verwaltung: hinzufügen, bearbeiten, löschen und Health-Check
- **MCP-Browser** — Beliebte MCP-Server entdecken und installieren (Datenbanken, APIs, Tools)
- **Einstellungen** — Globale Einstellungen und Hooks bearbeiten
- **Berechtigungen** — Tool-Berechtigungen und Konfigurations-Health prüfen
- **Pläne** — Plan-Dateien verwalten
- **Sessions** — Session-Verlauf durchsuchen
- **Analytics** — Nutzungsmetriken und Einblicke

## Architektur

```
┌─────────────────────┐     ┌──────────────────────┐
│  claude-admin-       │     │  claude-admin-        │
│  backend             │◄────│  frontend             │
│  Rust + Axum 0.7     │     │  Leptos 0.6 CSR/WASM  │
│  Port 9022           │     │  Port 9023 (dev)       │
└──────────┬──────────┘     └──────────────────────┘
           │                         │
           │    claude-admin-shared  │
           │    (gemeinsame Typen)   │
           │                         │
           ▼                         │
   ~/.claude/  (Dateisystem)         │
                                     │
   Im Produktionsbetrieb wird das    │
   WASM-Frontend über rust-embed     │
   in das Backend-Binary eingebettet.│
```

- **Keine Datenbank** — liest aus und schreibt in das Dateisystem (`~/.claude/`)
- **Automatische Backups** — werden vor jeder Schreiboperation in `~/.claude/backups/` erstellt, zeitgestempelt pro Datei
- **Claude API optional** — funktioniert vollständig ohne `ANTHROPIC_API_KEY`
- **Einzelnes Binary** — der Produktions-Build bettet das Frontend über `rust-embed` ein

## Sicherheit

ClaudeAdmin läuft **lokal auf deinem Rechner**. Es ist für den Einzelbenutzer-Einsatz auf einer Entwicklungsworkstation konzipiert.

- Liest und schreibt ausschließlich unter `~/.claude/` und `~/.claude.json`
- Kein Telemetrie, keine Analytics, keine Remote-Aufrufe (außer optionale Anthropic-API, wenn konfiguriert)
- Keine Authentifizierung — **nicht ohne Reverse-Proxy und Auth-Schicht im Internet exponieren**

## Erste Schritte

### Voraussetzungen

- [Rust](https://rustup.rs/) (stable)
- [Trunk](https://trunkrs.dev/) für WASM-Builds: `cargo install trunk`
- WASM-Target: `rustup target add wasm32-unknown-unknown`

### Entwicklung

```bash
# Projekt klonen und öffnen
git clone https://github.com/conradbruchmann/claude-admin.git
cd claude-admin

# Backend starten
cargo run -p claude-admin-backend

# In einem separaten Terminal: Frontend mit Hot-Reload starten
cd claude-admin-frontend && trunk serve --port 9023
```

Backend: `http://localhost:9022` — Frontend: `http://localhost:9023`

### Produktions-Build

```bash
# WASM-Frontend bauen
cd claude-admin-frontend && trunk build --release && cd ..

# Backend bauen (bettet das Frontend ein)
cargo build --release -p claude-admin-backend

# Einzelnes Binary ausführen
./target/release/claude-admin-backend
```

Vorgefertigte Binaries für Linux, macOS und Windows sind auf der [Releases](https://github.com/conradBruchmann/claude-admin/releases)-Seite verfügbar.

### macOS-Installation

Lade die `.dmg`-Datei aus dem neuesten Release herunter, öffne sie und ziehe **ClaudeAdmin.app** in `/Applications`.

Da die App nicht mit einem Apple-Entwicklerzertifikat signiert ist, wird macOS Gatekeeper sie beim ersten Start blockieren. Um sie zuzulassen, führe aus:

```bash
xattr -dr com.apple.quarantine /Applications/ClaudeAdmin.app
```

Dann die App per Doppelklick starten — sie startet den Server und öffnet `http://localhost:9022` im Browser. Das DMG enthält ein Universal Binary, das nativ auf Intel- und Apple-Silicon-Macs läuft.

### Windows-Installation

Lade die `ClaudeAdmin-*-Setup.exe` aus dem neuesten Release herunter und führe den Installer aus. Er installiert in dein Benutzerprofil (keine Administratorrechte erforderlich), erstellt Start-Menü- und Desktop-Verknüpfungen und registriert sich unter „Apps & Features" für eine saubere Deinstallation.

Nach der Installation ClaudeAdmin über das Start-Menü oder den Desktop starten — er startet den Server und öffnet `http://localhost:9022` im Browser.

## Konfigurationspfade

ClaudeAdmin liest und schreibt die standardmäßige Claude Code-Konfiguration:

| Pfad                                   | Beschreibung                          |
| -------------------------------------- | ------------------------------------- |
| `~/.claude.json`                       | Projektregister, MCP-Server           |
| `~/.claude/settings.json`              | Globale Einstellungen, Hooks          |
| `~/.claude/skills/`                    | Globale Skills                        |
| `~/.claude/rules/`                     | Globale Rules                         |
| `~/.claude/plans/`                     | Plan-Dateien                          |
| `~/.claude/projects/<encoded>/memory/` | Projektbezogener Memory               |
| `~/.claude/backups/`                   | Automatische Backups (zeitgestempelt) |

## Lizenz

[MIT](LICENSE) — Copyright (c) 2024-2026 Conrad Bruchmann, BRUCHMANN [TEC] INNOVATION GMBH
