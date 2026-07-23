# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in `rmb`, please **do not** open a public issue.
Instead, report it privately by emailing [security@kerryhatcher.com](mailto:security@kerryhatcher.com).

We will respond within **48 hours** to acknowledge your report and begin investigating.
We will work with you on a fix and coordinate a disclosure timeline.

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x (main) | ✅ Active development |
| Future 1.x | ✅ Will be supported |

As `rmb` is pre-1.0, only the latest commit on `main` is actively supported.

## What to Include in a Report

To help us triage quickly, please include:

- A description of the vulnerability and its potential impact
- Steps to reproduce the issue
- The version/commit of `rmb` you're using
- Any relevant logs or error output

## Disclosure Process

1. Reporter submits vulnerability privately
2. Maintainers acknowledge within 48 hours
3. Maintainers investigate and develop a fix
4. A security advisory is published on GitHub with the fix
5. Credit is given to the reporter (unless they prefer to remain anonymous)

## Security Best Practices for Users

- Always use the latest release of `rmb`
- Be cautious when installing plugins from untrusted sources — `rmb install` clones and executes code from git repositories
- Review plugin source code before installing, especially for plugins that modify your Pi or Claude Code configuration
