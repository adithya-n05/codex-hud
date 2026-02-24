import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const DEFAULT_SEEN_FILE = '.github/release-tracking/seen-codex-versions.json';
const NPM_CODEX_URL = 'https://registry.npmjs.org/@openai/codex';
const BREW_CODEX_URL = 'https://formulae.brew.sh/api/formula/codex.json';

export function normalizeVersion(raw) {
  if (typeof raw !== 'string') {
    return null;
  }
  const cleaned = raw.trim().replace(/^v/i, '');
  return cleaned.length > 0 ? cleaned : null;
}

export function parseNpmVersionResponse(json) {
  return normalizeVersion(json?.['dist-tags']?.latest ?? json?.version ?? null);
}

export function parseBrewVersionResponse(json) {
  return normalizeVersion(json?.versions?.stable ?? json?.version ?? null);
}

function compareSemverLike(a, b) {
  const aParts = a.split('.').map((part) => Number.parseInt(part, 10) || 0);
  const bParts = b.split('.').map((part) => Number.parseInt(part, 10) || 0);
  const len = Math.max(aParts.length, bParts.length);
  for (let i = 0; i < len; i += 1) {
    const av = aParts[i] ?? 0;
    const bv = bParts[i] ?? 0;
    if (av !== bv) {
      return bv - av;
    }
  }
  return 0;
}

export function detectUnseenVersions(currentVersions, seenVersions) {
  const seen = new Set(
    (seenVersions ?? []).map((version) => normalizeVersion(version)).filter(Boolean),
  );
  const latest = [
    normalizeVersion(currentVersions?.npm),
    normalizeVersion(currentVersions?.brew),
  ].filter(Boolean);
  const uniqueLatest = [...new Set(latest)];
  const unseen = uniqueLatest.filter((version) => !seen.has(version));
  return unseen.sort(compareSemverLike);
}

async function fetchJson(url, fetchImpl = globalThis.fetch) {
  const res = await fetchImpl(url, { headers: { 'user-agent': 'codex-hud-ci' } });
  if (!res.ok) {
    throw new Error(`request failed: ${url} (${res.status})`);
  }
  return res.json();
}

export async function resolveCurrentCodexVersions(fetchImpl = globalThis.fetch) {
  const out = { npm: null, brew: null };

  try {
    const npmJson = await fetchJson(NPM_CODEX_URL, fetchImpl);
    out.npm = parseNpmVersionResponse(npmJson);
  } catch {
    out.npm = null;
  }

  try {
    const brewJson = await fetchJson(BREW_CODEX_URL, fetchImpl);
    out.brew = parseBrewVersionResponse(brewJson);
  } catch {
    out.brew = null;
  }

  return out;
}

function readSeenVersions(pathname) {
  if (!existsSync(pathname)) {
    return [];
  }
  const parsed = JSON.parse(readFileSync(pathname, 'utf8'));
  if (!Array.isArray(parsed?.seen)) {
    return [];
  }
  return parsed.seen.filter((value) => typeof value === 'string');
}

function writeSeenVersions(pathname, versions) {
  mkdirSync(dirname(pathname), { recursive: true });
  const deduped = [...new Set(versions.map((version) => normalizeVersion(version)).filter(Boolean))]
    .sort(compareSemverLike);
  writeFileSync(pathname, `${JSON.stringify({ seen: deduped }, null, 2)}\n`);
}

export async function runDetectCodexUpdates({
  fetchImpl = globalThis.fetch,
  seenFile = DEFAULT_SEEN_FILE,
  writeSeen = false,
} = {}) {
  const seenPath = resolve(process.cwd(), seenFile);
  const seenVersions = readSeenVersions(seenPath);
  const current = await resolveCurrentCodexVersions(fetchImpl);
  const newVersions = detectUnseenVersions(current, seenVersions);

  if (writeSeen && newVersions.length > 0) {
    writeSeenVersions(seenPath, [...seenVersions, ...newVersions]);
  }

  return {
    current,
    seen_versions: seenVersions,
    new_versions: newVersions,
    has_updates: newVersions.length > 0,
  };
}

const isMain = (() => {
  if (!process.argv[1]) {
    return false;
  }
  return resolve(process.argv[1]) === resolve(fileURLToPath(import.meta.url));
})();

if (isMain) {
  const seenFile = process.env.CODEX_HUD_SEEN_FILE ?? DEFAULT_SEEN_FILE;
  const writeSeen = process.env.CODEX_HUD_WRITE_SEEN === '1';
  runDetectCodexUpdates({ seenFile, writeSeen })
    .then((result) => {
      process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
    })
    .catch((error) => {
      process.stderr.write(`codex release detection failed: ${error.message}\n`);
      process.exitCode = 1;
    });
}
