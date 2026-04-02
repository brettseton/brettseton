This project is a Rust WebAssembly frontend built with Trunk.

## Getting Started

Run the app locally with Trunk:

```bash
trunk serve
```

Create a production build:

```bash
trunk build --release
```

## Structure

- `index.html`: Trunk entrypoint and static shell markup
- `src/lib.rs`: WASM entrypoint that wires UI, effects, and terminal core
- `src/apps/`: interactive terminal programs implemented in Rust
- `src/commands/`: built-in command registry, parsing, and execution
- `src/terminal/core/`: state machine, action dispatch, and shell/app transitions
- `src/terminal/input/`: keyboard, shell, tick, and joystick event bindings
- `src/terminal/ui/`: DOM diffing, markup, presentation, and surface management
- `src/terminal/emulator.rs`: terminal-style primary and alternate screen buffers
- `src/assets/style.css`: site styling
