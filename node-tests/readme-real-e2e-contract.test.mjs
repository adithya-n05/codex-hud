import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';

test('readme includes real codex e2e runbook', () => {
  const readme = fs.readFileSync('README.md', 'utf8');
  assert.ok(readme.includes('codex-hud install'));
  assert.ok(readme.includes('codex-hud status'));
  assert.ok(readme.includes('codex-hud uninstall'));
  assert.ok(readme.includes('codex-hud run --stock-codex'));
  assert.ok(readme.includes('/statusline'));
  assert.ok(readme.includes('CODEX_HUD_E2E_REAL=1'));
  assert.ok(readme.includes('CODEX_BIN='));
  assert.ok(readme.includes('source-layout'));
  assert.ok(readme.includes('native patch substrate unavailable for installed codex layout'));
});
