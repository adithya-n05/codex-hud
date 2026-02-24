import {
  chmodSync,
  copyFileSync,
  existsSync,
  mkdirSync,
  writeFileSync,
} from 'node:fs';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { dirname, join } from 'node:path';
import { homedir } from 'node:os';

export function resolvePackagedRuntimeBinary(
  packageRoot,
  platform = process.platform,
  arch = process.arch,
) {
  const exe = platform === 'win32' ? 'codex_hud_cli.exe' : 'codex_hud_cli';
  return join(packageRoot, 'dist', 'runtime', `${platform}-${arch}`, exe);
}

function writeUnixDevFallback(runtimePath, packageRoot) {
  const manifestPath = join(packageRoot, 'rust', 'Cargo.toml');
  const shim = [
    '#!/usr/bin/env sh',
    `exec cargo run --quiet --manifest-path "${manifestPath}" -p codex_hud_cli -- "$@"`,
    '',
  ].join('\n');
  writeFileSync(runtimePath, shim);
  chmodSync(runtimePath, 0o755);
}

function writeWindowsDevFallback(cmdPath, packageRoot) {
  const manifestPath = join(packageRoot, 'rust', 'Cargo.toml');
  const shim = [
    '@echo off',
    `cargo run --quiet --manifest-path "${manifestPath}" -p codex_hud_cli -- %*`,
    '',
  ].join('\r\n');
  writeFileSync(cmdPath, shim);
}

function installCompatAssets(packageRoot, homeDir) {
  const compatDir = join(homeDir, '.codex-hud', 'compat');
  mkdirSync(compatDir, { recursive: true });
  const compatSrcDir = join(packageRoot, 'assets', 'compat');
  const manifestSrc = join(compatSrcDir, 'compat.json');
  const pubkeySrc = join(compatSrcDir, 'public_key.hex');
  if (existsSync(manifestSrc) && existsSync(pubkeySrc)) {
    copyFileSync(manifestSrc, join(compatDir, 'compat.json'));
    copyFileSync(pubkeySrc, join(compatDir, 'public_key.hex'));
    return;
  }

  writeFileSync(
    join(compatDir, 'compat.json'),
    '{"schema_version":1,"supported_keys":[],"signature_hex":"00"}\n',
  );
  writeFileSync(join(compatDir, 'public_key.hex'), '00\n');
}

export function runPostinstallForHome(
  packageRoot,
  homeDir = homedir(),
  platform = process.platform,
  arch = process.arch,
) {
  const runtimeDir = join(homeDir, '.codex-hud', 'bin');
  mkdirSync(runtimeDir, { recursive: true });

  const packagedRuntime = resolvePackagedRuntimeBinary(packageRoot, platform, arch);
  if (platform === 'win32') {
    const cmdPath = join(runtimeDir, 'codex-hud.cmd');
    if (existsSync(packagedRuntime)) {
      const exePath = join(runtimeDir, 'codex-hud.exe');
      copyFileSync(packagedRuntime, exePath);
      const cmdShim = ['@echo off', `"${exePath}" %*`, ''].join('\r\n');
      writeFileSync(cmdPath, cmdShim);
    } else {
      writeWindowsDevFallback(cmdPath, packageRoot);
    }
  } else {
    const runtimePath = join(runtimeDir, 'codex-hud');
    if (existsSync(packagedRuntime)) {
      copyFileSync(packagedRuntime, runtimePath);
      chmodSync(runtimePath, 0o755);
    } else {
      writeUnixDevFallback(runtimePath, packageRoot);
    }
  }

  installCompatAssets(packageRoot, homeDir);
}

if (process.argv[1] && pathToFileURL(process.argv[1]).href === import.meta.url) {
  const here = dirname(fileURLToPath(import.meta.url));
  const packageRoot = dirname(here);
  runPostinstallForHome(packageRoot, homedir());
}
