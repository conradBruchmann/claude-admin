🌍 [English](README.md) | [Deutsch](README.de.md) | [Nederlands](README.nl.md) | **Português** | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [中文](README.zh.md) | [Polski](README.pl.md) | [Türkçe](README.tr.md)

# ClaudeAdmin

Um console de administração web para gerenciar sua configuração do [Claude Code](https://claude.com/claude-code).

Quando você trabalha com mais do que alguns projetos, acompanhar skills, regras, arquivos de memória, servidores MCP e configurações espalhados pelo `~/.claude/` se torna trabalhoso. O ClaudeAdmin oferece uma interface única para gerenciar tudo isso — sem precisar editar JSON e YAML manualmente.

> **Nota:** Este é um projeto comunitário independente. Não é afiliado nem endossado pela Anthropic.

![ClaudeAdmin Dashboard](https://github.com/conradBruchmann/claude-admin/blob/master/docs/screenshot.png?raw=true)

## Funcionalidades

- **Dashboard** — Visão geral da sua configuração global e por projeto
- **Projetos** — Explore projetos detectados, edite o CLAUDE.md, gerencie regras, skills e memória específicas por projeto
- **Skills** — Crie, edite e explore skills globais (YAML frontmatter + markdown)
- **Navegador de Skills** — Descubra e instale skills da comunidade com um clique
- **Regras** — Gerencie regras globais e por projeto
- **Memória** — Visualize e edite arquivos de memória por projeto (MEMORY.md + arquivos de tópicos)
- **Servidores MCP** — Gerenciamento completo de servidores MCP: adicionar, editar, excluir e verificar saúde
- **Navegador MCP** — Descubra e instale servidores MCP populares (bancos de dados, APIs, ferramentas)
- **Configurações** — Edite configurações globais e hooks
- **Permissões** — Revise permissões de ferramentas e a saúde da configuração
- **Planos** — Gerencie arquivos de planos
- **Sessões** — Explore o histórico de sessões
- **Analytics** — Métricas de uso e insights

## Arquitetura

```
┌─────────────────────┐     ┌──────────────────────┐
│  claude-admin-       │     │  claude-admin-        │
│  backend             │◄────│  frontend             │
│  Rust + Axum 0.7     │     │  Leptos 0.6 CSR/WASM  │
│  Port 9022           │     │  Port 9023 (dev)       │
└──────────┬──────────┘     └──────────────────────┘
           │                         │
           │    claude-admin-shared  │
           │    (tipos compartilhados)│
           │                         │
           ▼                         │
   ~/.claude/  (sistema de arquivos) │
                                     │
   Em produção, o frontend WASM      │
   é embutido no binário do backend  │
   via rust-embed.                   │
```

- **Sem banco de dados** — lê e grava diretamente no sistema de arquivos (`~/.claude/`)
- **Backups automáticos** — criados em `~/.claude/backups/` antes de cada operação de escrita, com timestamp por arquivo
- **Claude API opcional** — funciona completamente sem `ANTHROPIC_API_KEY`
- **Binário único** — o build de produção embute o frontend via `rust-embed`

## Segurança

O ClaudeAdmin é executado **localmente na sua máquina**. Ele foi projetado para uso de um único usuário em uma estação de trabalho de desenvolvimento.

- Lê e grava apenas em `~/.claude/` e `~/.claude.json`
- Sem telemetria, sem analytics, sem chamadas remotas (exceto a API Anthropic opcional, se configurada)
- Sem autenticação — **não exponha à internet** sem um proxy reverso e uma camada de autenticação

## Primeiros Passos

### Pré-requisitos

- [Rust](https://rustup.rs/) (stable)
- [Trunk](https://trunkrs.dev/) para builds WASM: `cargo install trunk`
- Target WASM: `rustup target add wasm32-unknown-unknown`

### Desenvolvimento

```bash
# Clone e entre no projeto
git clone https://github.com/conradbruchmann/claude-admin.git
cd claude-admin

# Inicie o backend
cargo run -p claude-admin-backend

# Em um terminal separado: inicie o frontend com hot-reload
cd claude-admin-frontend && trunk serve --port 9023
```

Backend: `http://localhost:9022` — Frontend: `http://localhost:9023`

### Build de Produção

```bash
# Build do frontend WASM
cd claude-admin-frontend && trunk build --release && cd ..

# Build do backend (embute o frontend)
cargo build --release -p claude-admin-backend

# Execute o binário único
./target/release/claude-admin-backend
```

Binários pré-compilados para Linux, macOS e Windows estão disponíveis na página de [Releases](https://github.com/conradBruchmann/claude-admin/releases).

### Instalação no macOS

Baixe o `.dmg` da versão mais recente, abra-o e arraste o **ClaudeAdmin.app** para `/Applications`.

Como o aplicativo não é assinado com um certificado Apple Developer, o macOS Gatekeeper irá bloqueá-lo na primeira execução. Para permitir sua execução, execute:

```bash
xattr -dr com.apple.quarantine /Applications/ClaudeAdmin.app
```

Em seguida, clique duas vezes no aplicativo — ele inicia o servidor e abre `http://localhost:9022` no seu navegador. O DMG contém um Binário Universal que roda nativamente em Macs com Intel e Apple Silicon.

### Instalação no Windows

Baixe o `ClaudeAdmin-*-Setup.exe` da versão mais recente e execute o instalador. Ele instala no seu perfil de usuário (sem necessidade de direitos de administrador), cria atalhos no Menu Iniciar e na Área de Trabalho, e registra em "Aplicativos e Recursos" para uma desinstalação limpa.

Após a instalação, inicie o ClaudeAdmin pelo Menu Iniciar ou pela Área de Trabalho — ele inicia o servidor e abre `http://localhost:9022` no seu navegador.

## Caminhos de Configuração

O ClaudeAdmin lê e grava na configuração padrão do Claude Code:

| Caminho | Descrição |
|---------|-----------|
| `~/.claude.json` | Registro de projetos, servidores MCP |
| `~/.claude/settings.json` | Configurações globais, hooks |
| `~/.claude/skills/` | Skills globais |
| `~/.claude/rules/` | Regras globais |
| `~/.claude/plans/` | Arquivos de planos |
| `~/.claude/projects/<encoded>/memory/` | Memória por projeto |
| `~/.claude/backups/` | Backups automáticos (com timestamp) |

## Licença

[MIT](LICENSE) — Copyright (c) 2024-2026 Conrad Bruchmann, BRUCHMANN [TEC] INNOVATION GMBH
