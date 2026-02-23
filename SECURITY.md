# Security Policy

## Supported Versions

Security updates are provided for the latest release line.

| Version | Supported |
| --- | --- |
| 0.1.x | Yes |
| < 0.1.0 | No |

## Reporting a Vulnerability

Please report vulnerabilities privately. Do **not** open a public issue first.

Preferred channel:

- GitHub private vulnerability report: https://github.com/adithya-n05/codex-hud/security/advisories/new

Please include:

- A clear description of the issue and impact
- Reproduction steps or proof-of-concept
- Affected versions/commits
- Any suggested remediation

If sensitive data is involved, redact secrets before sharing.

## Response Process

- Initial acknowledgement target: within 3 business days
- Triage/update target: within 7 business days
- Fixes are developed and validated privately when needed
- Public disclosure happens after a fix or mitigation is available

## Security Boundaries and Expectations

When contributing changes, preserve these project expectations:

- No secret/token leakage in rendered output, logs, tests, or docs
- Compatibility checks remain fail-closed for unknown Codex `version + sha256`
- Installer/uninstaller actions remain scoped to codex-hud managed artifacts
- RC file modifications remain confined to codex-hud managed blocks

## Out of Scope

The following are usually out of scope unless they produce a concrete security impact:

- Cosmetic/documentation-only typos
- Feature requests without a security defect
- Known limitations documented as non-goals in project plans

## Disclosure and Credit

We appreciate responsible disclosure. With your permission, we will credit valid
reports in release notes or advisories.
