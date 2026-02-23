#!/usr/bin/env node
import { join } from 'node:path';
import { homedir } from 'node:os';

export function resolveManagedBinaryPath(homeDir = homedir()) {
  return join(homeDir, '.codex-hud', 'bin', 'codex-hud');
}

if (process.argv[1] && process.argv[1].endsWith('codex-hud.js')) {
  console.log('codex-hud wrapper');
}
