import test from 'node:test';
import assert from 'node:assert/strict';
import { resolveManagedBinaryPath } from '../bin/codex-hud.js';
import { buildManagedInvocation } from '../bin/codex-hud.js';
import { executeManagedInvocation } from '../bin/codex-hud.js';
import { join } from 'node:path';

test('managed binary path uses ~/.codex-hud/bin/codex-hud entrypoint', () => {
  const out = resolveManagedBinaryPath('/home/example');
  const expected = join(
    '/home/example',
    '.codex-hud',
    'bin',
    process.platform === 'win32' ? 'codex-hud.cmd' : 'codex-hud',
  );
  assert.equal(out, expected);
});

test('wrapper builds managed runtime invocation with passthrough args', () => {
  const out = buildManagedInvocation('/home/example', ['status', 'details']);
  const expected = join(
    '/home/example',
    '.codex-hud',
    'bin',
    process.platform === 'win32' ? 'codex-hud.cmd' : 'codex-hud',
  );
  assert.equal(out.command, expected);
  assert.deepEqual(out.args, ['status', 'details']);
});

test('wrapper exports executeManagedInvocation helper', () => {
  assert.equal(typeof executeManagedInvocation, 'function');
});
