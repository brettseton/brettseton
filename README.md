This project is a Rust WebAssembly frontend built with Trunk.

The terminal content is sourced from real files under `src/terminal-fs/`. A `build.rs` step reads that directory at compile time and bakes the filesystem snapshot into the WASM app, so the browser-side terminal stays aligned with the real project files without a runtime server. The terminal also includes built-in Rust apps exposed through the `guess` and `snake` commands.

## Getting Started

Run the app locally with Trunk:

```bash
trunk serve
```

Create a production build:

```bash
trunk build --release
```

By default, Trunk writes production output to `dist/`.

Useful verification command:

```bash
cargo check --target wasm32-unknown-unknown
```

## Deploying

GitHub Pages deployment is configured in `.github/workflows/deploy.yml`. On every push to `main`, GitHub Actions will:

- install the `wasm32-unknown-unknown` Rust target
- install `trunk`
- run `trunk build --release`
- publish the generated `dist/` directory to GitHub Pages

To enable it in GitHub:

1. Push this repo to GitHub.
2. In the repository settings, open `Settings -> Pages`.
3. Set `Source` to `GitHub Actions`.
4. Ensure your custom domain is set to `brettseton.com` if you want Pages to serve that domain.

## Structure

- `index.html`: Trunk entrypoint and static shell markup
- `src/lib.rs`: Rust/WASM terminal app and DOM behavior
- `src/apps/`: interactive terminal apps implemented in Rust
- `build.rs`: compile-time terminal filesystem snapshot generator
- `src/terminal_fs.rs`: runtime access to generated terminal filesystem data
- `src/assets/style.css`: site styling
- `src/terminal-fs/pages/`: text content exposed as terminal files
- `src/terminal-fs/links/`: link targets opened by terminal commands

## Notes

- Root `ls` output is generated from the real files and links plus the built-in `guess`, `help`, and `snake` commands.
- Files under `src/terminal-fs/links/` open in a new tab.
- The terminal supports `?`, `about`, `clear`, `github`, `guess`, `help`, `linkedin`, `ls`, `snake`, and `stack`. `game` remains as an alias for `guess`.
- In this local environment, `cargo check --target wasm32-unknown-unknown` passes, while `trunk build --release` currently fails because Trunk receives `--no-color=1`.
