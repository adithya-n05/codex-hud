import {
  chmodSync,
  copyFileSync,
  existsSync,
  mkdirSync,
  writeFileSync,
} from 'node:fs';
import { spawnSync } from 'node:child_process';
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

export function resolveLocalBuiltRuntimeBinary(
  packageRoot,
  platform = process.platform,
) {
  const exe = platform === 'win32' ? 'codex_hud_cli.exe' : 'codex_hud_cli';
  return join(packageRoot, 'rust', 'target', 'release', exe);
}

function tryBuildRuntimeOnInstall(packageRoot, platform = process.platform) {
  const manifestPath = join(packageRoot, 'rust', 'Cargo.toml');
  if (!existsSync(manifestPath)) {
    return null;
  }

  const build = spawnSync(
    'cargo',
    ['build', '--release', '--manifest-path', manifestPath, '-p', 'codex_hud_cli'],
    { stdio: 'ignore' },
  );
  if (build.status !== 0) {
    return null;
  }

  const built = resolveLocalBuiltRuntimeBinary(packageRoot, platform);
  return existsSync(built) ? built : null;
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

function adHocCodesignRuntime(runtimePath, runCommand = spawnSync) {
  const result = runCommand('codesign', ['--force', '--sign', '-', runtimePath], {
    stdio: 'ignore',
  });
  if (result.status !== 0) {
    throw new Error(`codesign failed for ${runtimePath}`);
  }
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
  options = {},
) {
  const runtimeDir = join(homeDir, '.codex-hud', 'bin');
  mkdirSync(runtimeDir, { recursive: true });

  const packagedRuntime = resolvePackagedRuntimeBinary(packageRoot, platform, arch);
  const localBuiltRuntime = resolveLocalBuiltRuntimeBinary(packageRoot, platform);
  const skipLocalBuild =
    options.skipLocalBuild ?? process.env.CODEX_HUD_SKIP_LOCAL_BUILD === '1';
  const resolvedBuiltRuntime =
    options.resolveBuiltRuntime ??
    ((root, runtimePlatform) => tryBuildRuntimeOnInstall(root, runtimePlatform));
  const runtimeCandidate = existsSync(packagedRuntime)
    ? packagedRuntime
    : existsSync(localBuiltRuntime)
      ? localBuiltRuntime
      : !skipLocalBuild
        ? resolvedBuiltRuntime(packageRoot, platform, arch)
        : null;
  if (platform === 'win32') {
    const cmdPath = join(runtimeDir, 'codex-hud.cmd');
    if (runtimeCandidate) {
      const exePath = join(runtimeDir, 'codex-hud.exe');
      copyFileSync(runtimeCandidate, exePath);
      const cmdShim = ['@echo off', `"${exePath}" %*`, ''].join('\r\n');
      writeFileSync(cmdPath, cmdShim);
    } else {
      writeWindowsDevFallback(cmdPath, packageRoot);
    }
  } else {
    const runtimePath = join(runtimeDir, 'codex-hud');
    const runCommand = options.runCommand ?? spawnSync;
    if (runtimeCandidate) {
      copyFileSync(runtimeCandidate, runtimePath);
      chmodSync(runtimePath, 0o755);
      if (platform === 'darwin') {
        adHocCodesignRuntime(runtimePath, runCommand);
      }
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
