import test from 'node:test';
import assert from 'node:assert/strict';
import {
  normalizeVersion,
  parseNpmVersionResponse,
  parseBrewVersionResponse,
  detectUnseenVersions,
} from '../scripts/ci/detect-codex-updates.mjs';

test('normalizeVersion strips leading v and whitespace', () => {
  assert.equal(normalizeVersion(' v0.104.0 '), '0.104.0');
  assert.equal(normalizeVersion('0.105.1'), '0.105.1');
  assert.equal(normalizeVersion(''), null);
});

test('parseNpmVersionResponse resolves dist-tag latest', () => {
  const version = parseNpmVersionResponse({
    'dist-tags': { latest: 'v0.104.0' },
  });
  assert.equal(version, '0.104.0');
});

test('parseBrewVersionResponse resolves stable formula version', () => {
  const version = parseBrewVersionResponse({
    versions: { stable: '0.103.0' },
  });
  assert.equal(version, '0.103.0');
});

test('detectUnseenVersions returns sorted unique unseen versions', () => {
  const result = detectUnseenVersions(
    { npm: '0.104.0', brew: '0.103.0' },
    ['0.103.0', '0.102.0', '0.103.0'],
  );
  assert.deepEqual(result, ['0.104.0']);
});
