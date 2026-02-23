import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { execSync } from 'node:child_process';

function parsePackJson() {
  const raw = execSync('npm pack --json', { encoding: 'utf8' });
  return JSON.parse(raw)[0];
}

test('npm pack excludes .docs tree', () => {
  const pkg = JSON.parse(readFileSync('package.json', 'utf8'));
  assert.ok(Array.isArray(pkg.files));
  assert.ok(pkg.files.includes('bin/'));
  assert.ok(pkg.files.includes('scripts/'));
  assert.ok(pkg.files.includes('rust/'));
  assert.ok(pkg.files.includes('README.md'));
  assert.ok(pkg.files.includes('LICENSE'));
  assert.ok(!pkg.files.some((entry) => entry === '.docs/' || entry === '.docs'));

  const pack = parsePackJson();
  const names = new Set(pack.files.map((f) => f.path));
  assert.ok(names.has('scripts/postinstall.mjs'));
  assert.ok(names.has('rust/Cargo.toml'));
  for (const name of names) {
    assert.ok(!name.startsWith('.docs/'));
  }
});
