#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import { join } from 'node:path';
import { homedir } from 'node:os';

export function resolveManagedBinaryPath(homeDir = homedir()) {
  const entry = process.platform === 'win32' ? 'codex-hud.cmd' : 'codex-hud';
  return join(homeDir, '.codex-hud', 'bin', entry);
}

export function buildManagedInvocation(homeDir = homedir(), passthroughArgs = []) {
  return {
    command: resolveManagedBinaryPath(homeDir),
    args: passthroughArgs,
  };
}

export function executeManagedInvocation(homeDir = homedir(), passthroughArgs = []) {
  const invocation = buildManagedInvocation(homeDir, passthroughArgs);
  return spawnSync(invocation.command, invocation.args, { encoding: 'utf8' });
}

if (process.argv[1] && process.argv[1].endsWith('codex-hud.js')) {
  const out = executeManagedInvocation(homedir(), process.argv.slice(2));
  if (out.stdout) {
    process.stdout.write(out.stdout);
  }
  if (out.stderr) {
    process.stderr.write(out.stderr);
  }
  process.exit(out.status ?? 1);
}
