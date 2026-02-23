#!/usr/bin/env node
import { join } from 'node:path';
import { homedir } from 'node:os';

export function resolveManagedBinaryPath(homeDir = homedir()) {
  return join(homeDir, '.codex-hud', 'bin', 'codex-hud');
}

export function buildManagedInvocation(homeDir = homedir(), passthroughArgs = []) {
  return {
    command: resolveManagedBinaryPath(homeDir),
    args: passthroughArgs,
  };
}

if (process.argv[1] && process.argv[1].endsWith('codex-hud.js')) {
  const invocation = buildManagedInvocation(homedir(), process.argv.slice(2));
  console.log(`${invocation.command} ${invocation.args.join(' ')}`.trim());
}
