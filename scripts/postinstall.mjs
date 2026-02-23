import { mkdirSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';
import { homedir } from 'node:os';

const runtimeDir = join(homedir(), '.codex-hud', 'bin');
mkdirSync(runtimeDir, { recursive: true });

const runtimePath = join(runtimeDir, 'codex-hud');
const here = dirname(fileURLToPath(import.meta.url));
const packageRoot = dirname(here);
const installedBin = join(packageRoot, 'bin', 'codex-hud.js');
const shim = `#!/usr/bin/env sh\nexec node \"${installedBin}\" \"$@\"\n`;
writeFileSync(runtimePath, shim, { mode: 0o755 });
