## Summary

Briefly describe what this PR changes.

## Why

Explain the problem or goal this PR addresses.

## What Changed

- 

## Validation

List exactly what you ran locally:

```bash
npm test
cd rust
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

If you skipped any checks, explain why.

## Risk and Rollback

- Risk level: low / medium / high
- Rollback plan:

## Checklist

- [ ] Behavior changes include tests.
- [ ] Docs updated for user-facing changes.
- [ ] No secrets/tokens added to code, fixtures, logs, or screenshots.
- [ ] Changes remain scoped and reviewable.
