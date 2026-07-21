<h1 align="center">lazyact</h1>

<p align="center">TUI tool to list and run GitHub Actions workflows locally.</p>

<p align="center">
  <img src=".github/screenshot.png" alt="lazyact screenshot" width="80%">
</p>

## Features

- Lists workflows from `.github/workflows/`
- Runs workflows via `act` directly from the TUI
- 3 panels: Workflows, History, Logs

## Quick start

```sh
curl -fsSL https://raw.githubusercontent.com/Guhszvv/lazyact/master/install.sh | bash
```

**Prerequisites:** `act` and `docker` installed.

## Keybindings

| Key | Action |
|-----|--------|
| `q` | Quit |
| `r` | Run selected workflow |
| `1`/`2`/`3` | Focus Workflows / History / Logs |
| `↑`/`↓` | Navigate workflows / scroll logs |
| `PgUp`/`PgDn` | Scroll page up/down |
| `Home`/`End` | Scroll to top/bottom |

## Dependencies

- Rust: [ratatui](https://github.com/ratatui/ratatui), tokio, serde_yaml, [rattles](https://github.com/vyfor/rattles).
- Docker
- [Act](https://github.com/nektos/act)
