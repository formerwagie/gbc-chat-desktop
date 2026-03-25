# gbc.chat Desktop

The official desktop client for [gbc.chat](https://gbc.chat) — built with [Tauri v2](https://tauri.app/).

This is a lightweight native wrapper that loads `gbc.chat` in a system webview, with added desktop-specific features:

- **Native window** — proper title bar, resizing, minimize/maximize
- **External link handling** — links open in your default browser instead of navigating the app
- **Auto-updates** — silently checks for and installs updates on startup
- **System tray integration** — runs natively on Windows, macOS, and Linux

## Building from source

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) 18+
- [Tauri CLI](https://tauri.app/start/): `npm install -g @tauri-apps/cli`

### Build

```bash
npm install
npx tauri build
```

The installer will be in `src-tauri/target/release/bundle/`.

### Development

```bash
npx tauri dev
```

## Architecture

This is intentionally minimal. The entire app is ~65 lines of Rust:

```
src-tauri/
├── src/main.rs        # Webview + navigation interceptor + auto-updater
├── Cargo.toml         # Rust dependencies
├── tauri.conf.json    # Window config, updater endpoint, bundle settings
├── icons/             # App icons for all platforms
└── build.rs           # Tauri build script
```

The actual application (chat UI, backend, database) is served from `https://gbc.chat`. This wrapper just provides the native shell.

## License

MIT — see [LICENSE](LICENSE)
