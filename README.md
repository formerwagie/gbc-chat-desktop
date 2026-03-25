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

## Code Signing Policy

Free code signing provided by [SignPath.io](https://signpath.io), certificate by [SignPath Foundation](https://signpath.org).

### Team Roles

- **Authors & Reviewers:** [formerwagie](https://github.com/formerwagie)
- **Approvers:** [formerwagie](https://github.com/formerwagie)

### Privacy Policy

This program loads [gbc.chat](https://gbc.chat) in a native webview. User data (messages, account info) is handled by the gbc.chat platform. The desktop wrapper itself does not collect, store, or transmit any additional user data beyond what the web application does.

Full privacy policy: [https://gbc.chat/privacy](https://gbc.chat/privacy)

## License

MIT — see [LICENSE](LICENSE)
