import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { resolvePackagedRuntimeBinary, runPostinstallForHome } from '../scripts/postinstall.mjs';

test('postinstall installs packaged runtime and compat assets without cargo fallback', () => {
  const pkg = fs.mkdtempSync(path.join(os.tmpdir(), 'hud-pkg-'));
  const home = fs.mkdtempSync(path.join(os.tmpdir(), 'hud-home-'));
  const runtimeDir = path.join(pkg, 'dist', 'runtime', 'linux-x64');
  fs.mkdirSync(runtimeDir, { recursive: true });
  fs.writeFileSync(path.join(runtimeDir, 'codex_hud_cli'), '#!/usr/bin/env sh\nexit 0\n', { mode: 0o755 });

  const compatDir = path.join(pkg, 'assets', 'compat');
  fs.mkdirSync(compatDir, { recursive: true });
  fs.writeFileSync(path.join(compatDir, 'compat.json'), '{"schema_version":1,"supported_keys":[],"signature_hex":"00"}');
  fs.writeFileSync(path.join(compatDir, 'public_key.hex'), '00');

  const resolved = resolvePackagedRuntimeBinary(pkg, 'linux', 'x64');
  assert.equal(resolved, path.join(pkg, 'dist', 'runtime', 'linux-x64', 'codex_hud_cli'));

  runPostinstallForHome(pkg, home, 'linux', 'x64');

  const installedEntry = path.join(home, '.codex-hud', 'bin', 'codex-hud');
  assert.ok(fs.existsSync(installedEntry));
  assert.ok(fs.statSync(installedEntry).isFile());
  assert.ok((fs.statSync(installedEntry).mode & 0o111) !== 0);
  assert.ok(!fs.existsSync(path.join(home, '.codex-hud', 'bin', 'codex_hud_cli')));
  assert.ok(fs.existsSync(path.join(home, '.codex-hud', 'compat', 'compat.json')));
  assert.ok(fs.existsSync(path.join(home, '.codex-hud', 'compat', 'public_key.hex')));
});
