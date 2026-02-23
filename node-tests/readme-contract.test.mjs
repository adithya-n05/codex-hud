import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

test('readme contains required install and policy clauses', () => {
  const readme = readFileSync('README.md', 'utf8');
  assert.ok(readme.includes('`codex-hud install`'));
  assert.ok(readme.includes('`codex-hud uninstall`'));
  assert.ok(readme.includes('reversible'));
  assert.ok(readme.includes('native integration limitations'));
  assert.ok(readme.includes('high-performance Rust'));
  assert.ok(!readme.includes('npx codex-hud'));
});
