import test from 'node:test';
import assert from 'node:assert/strict';
import { resolveManagedBinaryPath } from '../bin/codex-hud.js';
import { buildManagedInvocation } from '../bin/codex-hud.js';

test('managed binary path uses ~/.codex-hud/bin/codex-hud', () => {
  const out = resolveManagedBinaryPath('/home/example');
  assert.equal(out, '/home/example/.codex-hud/bin/codex-hud');
});

test('wrapper builds managed runtime invocation with passthrough args', () => {
  const out = buildManagedInvocation('/home/example', ['status', 'details']);
  assert.equal(out.command, '/home/example/.codex-hud/bin/codex-hud');
  assert.deepEqual(out.args, ['status', 'details']);
});
