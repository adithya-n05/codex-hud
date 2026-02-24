import test from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';

test('dependabot config exists for npm, cargo, and github-actions', () => {
  const path = '.github/dependabot.yml';
  assert.ok(existsSync(path), 'expected dependabot config file');
  const text = readFileSync(path, 'utf8');
  assert.ok(text.includes('package-ecosystem: "npm"'));
  assert.ok(text.includes('package-ecosystem: "cargo"'));
  assert.ok(text.includes('package-ecosystem: "github-actions"'));
});

test('ci workflow runs npm test and rust workspace checks', () => {
  const path = '.github/workflows/ci.yml';
  assert.ok(existsSync(path), 'expected ci workflow file');
  const text = readFileSync(path, 'utf8');
  assert.ok(text.includes('npm test'));
  assert.ok(text.includes('cargo test --workspace'));
  assert.ok(text.includes('cargo clippy --workspace --all-targets -- -D warnings'));
});

test('codex release detection workflow exists and calls detect script', () => {
  const path = '.github/workflows/codex-release-detect.yml';
  assert.ok(existsSync(path), 'expected release detection workflow');
  const text = readFileSync(path, 'utf8');
  assert.ok(text.includes('schedule:'));
  assert.ok(text.includes('scripts/ci/detect-codex-updates.mjs'));
  assert.ok(text.includes('workflow_dispatch:'));
});

test('compat patch pipeline workflow supports dispatch from release detector', () => {
  const path = '.github/workflows/compat-patch-pipeline.yml';
  assert.ok(existsSync(path), 'expected compat patch pipeline workflow');
  const text = readFileSync(path, 'utf8');
  assert.ok(text.includes('repository_dispatch:'));
  assert.ok(text.includes('codex-release-detected'));
  assert.ok(text.includes('workflow_dispatch:'));
});
