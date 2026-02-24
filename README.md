# Codex HUD

[![GitHub stars](https://img.shields.io/github/stars/adithya-n05/codex-hud?style=social)](https://github.com/adithya-n05/codex-hud/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/adithya-n05/codex-hud?style=social)](https://github.com/adithya-n05/codex-hud/network/members)
[![Contributors](https://img.shields.io/github/contributors/adithya-n05/codex-hud)](https://github.com/adithya-n05/codex-hud/graphs/contributors)
[![Last commit](https://img.shields.io/github/last-commit/adithya-n05/codex-hud)](https://github.com/adithya-n05/codex-hud/commits/main)
[![License](https://img.shields.io/github/license/adithya-n05/codex-hud)](LICENSE)
[![npm downloads](https://img.shields.io/badge/npm%20downloads-pending%20publish-lightgrey)](https://www.npmjs.com/package/codex-hud)

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

## HUD Feature Bucket

- Two-line HUD model with wrap on narrow terminals.
- CTX / 5H / 7D usage bars with threshold-aware coloring.
- Tool counters with optional per-type activity breakdown.
- Auth, permission, provider, model, and repo/branch chips.
- Time-to-reset metrics for 5-hour and 7-day windows (countdown style).

## Real Local End-to-End Check With Existing Codex

1. Confirm Codex is reachable: `codex --version`
2. Install HUD: `codex-hud install`
3. Verify status: `codex-hud status`
4. Open Codex and run `/statusline`
5. Uninstall cleanly: `codex-hud uninstall`
6. Verify stock Codex still works: `codex --version`

Optional strict E2E smoke:

`CODEX_HUD_E2E_REAL=1 CODEX_BIN=codex cargo test -p codex_hud_cli --test real_codex_e2e -- --exact real_codex_minimal_env_install_status_uninstall_and_passthrough_e2e`

Native patch behavior and limits:

- codex-hud applies native patching when a supported source-layout substrate is detected.
- for packaged/non-source Codex installs, codex-hud falls back safely and reports `native patch substrate unavailable for installed codex layout`.

Stock passthrough check:

`codex-hud run --stock-codex codex -- --version`

## Development Requirements

To run and contribute locally, install:

- Node.js 20+
- npm 10+
- Rust stable toolchain
- Git

This repository has a high bar for clean, well-tested code. See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution quality expectations and required checks.

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
