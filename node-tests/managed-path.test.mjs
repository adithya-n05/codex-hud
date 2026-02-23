import test from 'node:test';
import assert from 'node:assert/strict';
import { resolveManagedBinaryPath } from '../bin/codex-hud.js';

test('managed binary path uses ~/.codex-hud/bin/codex-hud', () => {
  const out = resolveManagedBinaryPath('/home/example');
  assert.equal(out, '/home/example/.codex-hud/bin/codex-hud');
});
