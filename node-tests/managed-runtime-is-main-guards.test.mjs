import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';

test('wrapper exits with fallback status when managed binary is missing', () => {
  const home = fs.mkdtempSync(path.join(os.tmpdir(), 'hud-home-missing-'));
  const out = spawnSync('node', ['bin/codex-hud.js', 'status'], {
    cwd: process.cwd(),
    env: { ...process.env, HOME: home, USERPROFILE: home },
    encoding: 'utf8',
  });

  assert.equal(out.status, 1);
});

test('wrapper guard handles missing argv entry', () => {
  const out = spawnSync(
    'node',
    ['--input-type=module', '-e', "delete process.argv[1]; await import('./bin/codex-hud.js');"],
    { cwd: process.cwd(), encoding: 'utf8' },
  );

  assert.equal(out.status, 0);
});

test('wrapper guard handles realpath failures', () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'hud-missing-entry-'));
  const missingEntry = path.join(root, 'missing-entrypoint.js');
  const code = `process.argv[1] = ${JSON.stringify(missingEntry)}; await import('./bin/codex-hud.js');`;
  const out = spawnSync('node', ['--input-type=module', '-e', code], {
    cwd: process.cwd(),
    encoding: 'utf8',
  });

  assert.equal(out.status, 0);
});
