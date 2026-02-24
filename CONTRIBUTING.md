# Contributing to codex-hud

Thanks for helping improve `codex-hud`.

## Ground Rules

- Be respectful and constructive. Follow [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
- Keep changes focused and reviewable.
- For non-trivial features or behavior changes, open an issue first.

## Quality Bar

`codex-hud` is maintained as a high code-quality repository. Contributions should favor clean, understandable, and well-tested changes over quick patches.

- Keep logic cohesive and avoid cross-layer coupling.
- Prefer explicit behavior and clear failure modes over hidden side effects.
- Add or update tests for every behavior change, bug fix, or regression risk.
- Do not merge changes with failing checks; all required verification must pass locally.
- Treat code review feedback on correctness, reliability, and maintainability as release-blocking.

## Development Setup

1. Install prerequisites:
- Node.js 20+
- npm 10+
- Rust stable toolchain

2. Install Node dependencies:

```bash
npm install
```

3. Run baseline Node tests:

```bash
npm test
```

4. Run Rust workspace checks:

```bash
cd rust
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Project Structure

- `bin/` Node entrypoint shim for `codex-hud`
- `scripts/` install-time scripts
- `rust/crates/codex_hud_domain` config/default/validation logic
- `rust/crates/codex_hud_classifier` provider/auth detection
- `rust/crates/codex_hud_renderer` two-line HUD rendering
- `rust/crates/codex_hud_statusline` `/statusline` integration layer
- `rust/crates/codex_hud_ops` install/uninstall/shim/compatibility/release-gate ops
- `rust/crates/codex_hud_cli` CLI parse/dispatch

## Branching and Commits

- Branch from `main`.
- Use conventional commit messages:
  - `feat(scope): ...`
  - `fix(scope): ...`
  - `refactor(scope): ...`
  - `test(scope): ...`
  - `docs(scope): ...`
  - `chore(scope): ...`
- Preferred scopes: `bootstrap`, `domain`, `classifier`, `renderer`, `statusline`, `ops`, `cli`, `release`, `docs`.

## Pull Request Checklist

Before opening a PR, ensure:

- Tests pass locally:

```bash
npm test
cd rust
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

- Any behavior change includes tests.
- Docs are updated when user-facing behavior changes.
- No secrets/tokens are committed (including in tests, logs, fixtures, or screenshots).
- PR description clearly states what changed, why, and how it was validated.

## Changelog Workflow

Track user-facing changes in [CHANGELOG.md](CHANGELOG.md).

- Add entries under `## [Unreleased]` as part of normal PR work.
- Use one of the existing buckets: `Added`, `Changed`, `Fixed`, `Security`.
- Keep entries short and user-focused (what changed and why it matters).
- At release time, move `Unreleased` entries into a versioned section using `## [x.y.z] - YYYY-MM-DD`.
- Start a fresh `## [Unreleased]` section immediately after cutting the release.

## Security and Privacy Expectations

- Never include raw API keys, bearer tokens, or credentials in code, docs, or issue text.
- Follow existing redaction patterns for status/detail outputs.
- Preserve safety guarantees around managed artifacts (`~/.codex-hud`) and uninstall behavior.
- For security issues, follow [SECURITY.md](SECURITY.md) instead of opening a public issue.

## License

By contributing to this repository, you agree that your contributions are licensed
under the [MIT License](LICENSE).
