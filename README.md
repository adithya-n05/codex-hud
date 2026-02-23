# Codex HUD

## Install

Run `codex-hud install`.

## Configure

Run `/statusline` inside Codex to configure the HUD.

## Uninstall and Reversibility

Run `codex-hud uninstall` to remove codex-hud managed changes. Uninstall is reversible and removes only codex-hud managed artifacts.

## Native Integration Limitations

This project uses native integration surfaces where available and documents native integration limitations where Codex behavior cannot be fully replaced without compatibility patching.

## Performance

codex-hud is a high-performance Rust-first HUD designed to match Codex runtime expectations.

Performance priorities in this repository:

- Rust-first runtime for domain logic, classification, rendering, statusline integration, and operational flows.
- Minimal JavaScript surface area (`bin/` and `scripts/`) so runtime behavior remains predictable.
- Deterministic install/uninstall behavior scoped to codex-hud managed artifacts.
- Compatibility gates that fail closed on unknown Codex `version + sha256` pairs.

As releases mature, reproducible benchmark data will be published for supported platforms.

## Development Requirements

To run and contribute locally, install:

- Node.js 20+
- npm 10+
- Rust stable toolchain
- Git

## Development Install

```bash
npm install
npm test
cd rust
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Inspiration

This project is inspired by [Claude HUD](https://github.com/jarrodwatts/claude-hud), created by [@jarrodwatts](https://github.com/jarrodwatts).
If you use Claude Code and want a similar HUD experience, check out his repository as well.

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=adithya-n05/codex-hud&type=Date)](https://star-history.com/#adithya-n05/codex-hud&Date)
