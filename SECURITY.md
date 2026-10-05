# Security Policy

## Reporting a Vulnerability

Please **do not** open a public GitHub issue or email a personal address to
report a security vulnerability in this repository.

Instead, report it privately through GitHub Security Advisories:

**https://github.com/Mendl-Labs/databaseschema/security/advisories/new**

This creates a private advisory visible only to the maintainer until a fix
is ready, which prevents disclosing an exploitable issue (e.g. a migration
that weakens a constraint, or a query helper vulnerable to SQL injection)
before it can be patched.

## What to Include

- A description of the vulnerability and its potential impact.
- Steps to reproduce, including affected migration(s), model(s), or
  `ops`/query helper(s) if applicable.
- The version/commit SHA you tested against.

## Supported Versions

This is a single-maintainer, pre-1.0 open-source crate. Only the latest
commit on `main` is supported; fixes are not backported to older tags.

## Response

The maintainer ([@ItsJustIkenna](https://github.com/ItsJustIkenna)) will
acknowledge new advisories as soon as possible and coordinate a fix and
disclosure timeline with the reporter.
