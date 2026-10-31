# DeepTutor macOS Native App (Tauri)

This directory contains the experimental macOS native app for DeepTutor, built using Tauri 2.

## Prerequisites

1. **Rust and Cargo**: Install via [rustup.rs](https://rustup.rs/).
2. **Node.js**: Ensure Node.js and npm are installed.
3. **Xcode Command Line Tools**: Run `xcode-select --install`.

## Development

1. Run the Next.js development server from the `web/` directory in a separate terminal:
   ```bash
   cd ../web
   npm run dev &
   ```

2. Start the Tauri app:
   ```bash
   npm run tauri dev &
   ```

## Build

1. Build the Next.js frontend (export static files):
   ```bash
   cd ../web
   npm run build
   ```
   *(Ensure `next.config.js` uses `output: 'export'`)*

2. Build the Tauri app:
   ```bash
   npm run tauri build
   ```
   The compiled macOS `.dmg` and `.app` bundles will be available in `src-tauri/target/release/bundle/`.
