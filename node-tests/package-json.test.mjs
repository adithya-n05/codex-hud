import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const pkg = JSON.parse(readFileSync('package.json', 'utf8'));

test('package has required identity fields', () => {
  assert.equal(pkg.name, 'codex-hud');
  assert.equal(pkg.version, '0.1.0');
  assert.equal(pkg.license, 'MIT');
});
