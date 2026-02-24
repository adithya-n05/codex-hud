import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';

test('wrapper preserves runtime stderr and exit code', () => {
  const home = fs.mkdtempSync(path.join(os.tmpdir(), 'hud-home-'));
  const managedDir = path.join(home, '.codex-hud', 'bin');
  fs.mkdirSync(managedDir, { recursive: true });
  const managed = path.join(managedDir, process.platform === 'win32' ? 'codex-hud.cmd' : 'codex-hud');

  if (process.platform === 'win32') {
    fs.writeFileSync(managed, '@echo off\r\necho boom 1>&2\r\nexit /b 7\r\n');
  } else {
    fs.writeFileSync(
      managed,
      '#!/usr/bin/env sh\nprintf "boom" 1>&2\nexit 7\n',
      { mode: 0o755 },
    );
  }

  const out = spawnSync('node', ['bin/codex-hud.js', 'status'], {
    cwd: process.cwd(),
    env: { ...process.env, HOME: home },
    encoding: 'utf8',
  });

  assert.equal(out.status, 7);
  assert.equal(out.stderr.trim(), 'boom');
});
