import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { runPostinstallForHome } from '../scripts/postinstall.mjs';

test('postinstall generates unix runtime fallback when packaged runtime is missing', () => {
  const pkg = fs.mkdtempSync(path.join(os.tmpdir(), 'hud-pkg-'));
  const home = fs.mkdtempSync(path.join(os.tmpdir(), 'hud-home-'));
  fs.mkdirSync(path.join(pkg, 'dist', 'runtime', 'linux-x64'), { recursive: true });
  fs.mkdirSync(path.join(pkg, 'assets', 'compat'), { recursive: true });
  fs.writeFileSync(path.join(pkg, 'assets', 'compat', 'compat.json'), '{"schema_version":1,"supported_keys":[],"signature_hex":"00"}');
  fs.writeFileSync(path.join(pkg, 'assets', 'compat', 'public_key.hex'), '00');

  runPostinstallForHome(pkg, home, 'linux', 'x64');

  const installedRuntime = path.join(home, '.codex-hud', 'bin', 'codex-hud');
  const runtimeText = fs.readFileSync(installedRuntime, 'utf8');

  assert.ok(runtimeText.includes('exec cargo run --quiet --manifest-path'));
  assert.ok(runtimeText.includes('codex_hud_cli'));
});

test('postinstall writes fallback compat files when packaged compat artifacts are missing', () => {
  const pkg = fs.mkdtempSync(path.join(os.tmpdir(), 'hud-pkg-'));
  const home = fs.mkdtempSync(path.join(os.tmpdir(), 'hud-home-'));
  const runtime = path.join(pkg, 'dist', 'runtime', 'linux-x64', 'codex_hud_cli');
  fs.mkdirSync(path.dirname(runtime), { recursive: true });
  fs.writeFileSync(runtime, '#!/usr/bin/env sh\nexit 0\n', { mode: 0o755 });

  runPostinstallForHome(pkg, home, 'linux', 'x64');

  const compatJson = path.join(home, '.codex-hud', 'compat', 'compat.json');
  const publicKey = path.join(home, '.codex-hud', 'compat', 'public_key.hex');

  assert.equal(
    fs.readFileSync(compatJson, 'utf8'),
    '{"schema_version":1,"supported_keys":[],"signature_hex":"00"}\n',
  );
  assert.equal(fs.readFileSync(publicKey, 'utf8'), '00\n');
});

test('postinstall generates windows runtime fallback when packaged runtime is missing', () => {
  const pkg = fs.mkdtempSync(path.join(os.tmpdir(), 'hud-pkg-'));
  const home = fs.mkdtempSync(path.join(os.tmpdir(), 'hud-home-'));
  fs.mkdirSync(path.join(pkg, 'dist', 'runtime', 'win32-x64'), { recursive: true });

  runPostinstallForHome(pkg, home, 'win32', 'x64');

  const installedCmd = path.join(home, '.codex-hud', 'bin', 'codex-hud.cmd');
  const runtimeText = fs.readFileSync(installedCmd, 'utf8');

  assert.ok(runtimeText.includes('cargo run --quiet --manifest-path'));
  assert.ok(runtimeText.includes('codex_hud_cli -- %*'));
});

test('postinstall prefers local built runtime over cargo-run fallback when available', () => {
  const pkg = fs.mkdtempSync(path.join(os.tmpdir(), 'hud-pkg-'));
  const home = fs.mkdtempSync(path.join(os.tmpdir(), 'hud-home-'));
  const localRuntime = path.join(pkg, 'rust', 'target', 'release', 'codex_hud_cli');
  fs.mkdirSync(path.dirname(localRuntime), { recursive: true });
  fs.writeFileSync(localRuntime, '#!/usr/bin/env sh\necho local-built\n', { mode: 0o755 });
  fs.mkdirSync(path.join(pkg, 'assets', 'compat'), { recursive: true });
  fs.writeFileSync(path.join(pkg, 'assets', 'compat', 'compat.json'), '{"schema_version":1,"supported_keys":[],"signature_hex":"00"}');
  fs.writeFileSync(path.join(pkg, 'assets', 'compat', 'public_key.hex'), '00');

  runPostinstallForHome(pkg, home, 'linux', 'x64');

  const installedRuntime = path.join(home, '.codex-hud', 'bin', 'codex-hud');
  const runtimeText = fs.readFileSync(installedRuntime, 'utf8');

  assert.equal(runtimeText, '#!/usr/bin/env sh\necho local-built\n');
  assert.ok(!runtimeText.includes('cargo run --quiet --manifest-path'));
});

test('postinstall uses resolved built runtime when packaged and local binaries are missing', () => {
  const pkg = fs.mkdtempSync(path.join(os.tmpdir(), 'hud-pkg-'));
  const home = fs.mkdtempSync(path.join(os.tmpdir(), 'hud-home-'));
  const builtRuntime = path.join(pkg, 'tmp', 'codex_hud_cli');
  fs.mkdirSync(path.dirname(builtRuntime), { recursive: true });
  fs.writeFileSync(builtRuntime, '#!/usr/bin/env sh\necho built-on-install\n', { mode: 0o755 });
  fs.mkdirSync(path.join(pkg, 'assets', 'compat'), { recursive: true });
  fs.writeFileSync(path.join(pkg, 'assets', 'compat', 'compat.json'), '{"schema_version":1,"supported_keys":[],"signature_hex":"00"}');
  fs.writeFileSync(path.join(pkg, 'assets', 'compat', 'public_key.hex'), '00');

  runPostinstallForHome(pkg, home, 'linux', 'x64', {
    resolveBuiltRuntime: () => builtRuntime,
  });

  const installedRuntime = path.join(home, '.codex-hud', 'bin', 'codex-hud');
  const runtimeText = fs.readFileSync(installedRuntime, 'utf8');

  assert.equal(runtimeText, '#!/usr/bin/env sh\necho built-on-install\n');
  assert.ok(!runtimeText.includes('cargo run --quiet --manifest-path'));
});

test('postinstall script entrypoint runs when executed directly', () => {
  const home = fs.mkdtempSync(path.join(os.tmpdir(), 'hud-home-'));
  const out = spawnSync('node', ['scripts/postinstall.mjs'], {
    cwd: process.cwd(),
    env: {
      ...process.env,
      HOME: home,
      USERPROFILE: home,
      CODEX_HUD_SKIP_LOCAL_BUILD: '1',
    },
    encoding: 'utf8',
  });

  const entryName = process.platform === 'win32' ? 'codex-hud.cmd' : 'codex-hud';
  const entry = path.join(home, '.codex-hud', 'bin', entryName);

  assert.equal(out.status, 0);
  assert.ok(fs.existsSync(entry));
});
