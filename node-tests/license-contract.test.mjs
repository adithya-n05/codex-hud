import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

test('license file contains canonical mit clauses', () => {
  const license = readFileSync('LICENSE', 'utf8');
  assert.ok(license.includes('Permission is hereby granted, free of charge'));
  assert.ok(license.includes('THE SOFTWARE IS PROVIDED "AS IS"'));
  assert.ok(license.includes('copies or substantial portions of the Software'));
});
