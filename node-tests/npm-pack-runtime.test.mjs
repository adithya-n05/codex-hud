import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { execSync } from 'node:child_process';

function parsePackJson() {
  return JSON.parse(execSync('npm pack --json', { encoding: 'utf8' }))[0];
}

test('npm pack includes runtime payload files', () => {
  const pkg = JSON.parse(readFileSync('package.json', 'utf8'));
  assert.ok(Array.isArray(pkg.files));
  assert.ok(pkg.files.includes('bin/'));
  assert.ok(pkg.files.includes('scripts/'));
  assert.ok(pkg.files.includes('assets/'));
  assert.ok(pkg.files.includes('rust/'));
  assert.ok(pkg.files.includes('package.json'));

  const pack = parsePackJson();
  const names = new Set(pack.files.map((f) => f.path));
  assert.ok(names.has('package.json'));
  assert.ok(names.has('bin/codex-hud.js'));
  assert.ok(names.has('scripts/postinstall.mjs'));
  assert.ok(names.has('assets/compat/compat.json'));
  assert.ok(names.has('assets/compat/public_key.hex'));
  assert.ok(names.has('rust/Cargo.toml'));
});
