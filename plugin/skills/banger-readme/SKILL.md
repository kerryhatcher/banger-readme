---
name: banger-readme
description: "Create outstanding, aesthetically pleasing, genuinely useful README files for GitHub repositories. Use when the user asks to create a README, improve a README, write project documentation, or mentions their README needs work. Generates SVG logos and headers, scores the README with the rmb CLI tool, and iterates until reaching an A grade. Covers all 14 essential sections, visual design, project hygiene, cognitive funneling, and anti-pattern avoidance based on extensive research of 100+ exemplary READMEs."
---

# /banger-readme

Create a standout README that converts visitors into users and contributors. Backed by research analyzing 100+ exemplary open source READMEs, the tool scores every iteration against 50+ criteria across 5 categories.

## Usage

```
/banger-readme                          # analyze current repo, propose improvements
/banger-readme --create                 # create a new README from scratch
/banger-readme --improve                # improve existing README iteratively
/banger-readme --logo                   # generate just the SVG logo and header
/banger-readme --score                  # run rmb score and show report
```

## What banger-readme does

Analyzes your repository, understands its purpose and tone, then builds a README that scores an **A (80+)** on the `rmb score` tool. Iterates automatically: write → score → identify gaps → improve → repeat until A.

Generates a custom SVG logo and header banner that matches your project's identity. Asks you about companion files (LICENSE, CONTRIBUTING.md, etc.) to get your intent right. Presents color palette options when generating visual assets.

## The Research Behind This Skill

This skill encodes findings from a comprehensive research project analyzing:

- **12 primary sources** including [Awesome README](https://github.com/matiassingers/awesome-readme) (21.2k stars), [Art of README](https://github.com/hackergrrl/art-of-readme) (7.2k stars), [Readme Driven Development](https://tom.preston-werner.com/2010/08/23/readme-driven-development.html) by Tom Preston-Werner, and [Daytona's 4,000-star launch guide](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project)
- **100+ exemplary READMEs** curated and categorized by project type
- **31 production READMEs** audited against research criteria (meilisearch, dioxus, helix, jj, yt-dlp, rustdesk, ruff, and more)
- **Emerging patterns for 2024–2026** including hero animations, Mermaid diagrams, collapsible sections, and dark/light mode support

Key findings baked into this skill:
- A great README can **5× your star conversion rate** (TOAST UI Editor went from 4% to 20% star/visitor ratio)
- The **cognitive funneling** principle: structure content broad→specific so readers can bail early if the project isn't for them
- **Demo GIFs are the single highest-ROI addition** — most projects miss this
- **Project hygiene** (LICENSE, CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md) is the foundation of trust
- **Visual design directly impacts conversion** — logos, badges, emoji headers, collapsible sections, and dark/light mode support

## What You Must Do When Invoked

### Step 0 — Understand the repository

Before doing anything else, understand what this project is about:

1. Read the existing README.md if it exists
2. Check the repository name, description, and topics
3. Look at the directory structure to understand what the project does
4. Read any Cargo.toml, package.json, pyproject.toml, or go.mod to understand the tech stack
5. Check if there's a LICENSE file, CONTRIBUTING.md, CODE_OF_CONDUCT.md, or SECURITY.md

If the user invoked `/banger-readme --score`, skip to Step 5 (just run the score).

### Step 1 — Ask the user about companion files

Before writing anything, ask the user about their intent for companion files. Use `ask_user_question` with these questions:

**Question 1: License**
```
header: "License"
question: "Which license should the project use?"
options:
  - label: "MIT (Recommended)"
    description: "Most permissive, widely adopted, path of least resistance for adoption"
  - label: "Apache 2.0"
    description: "Like MIT but includes explicit patent grant"
  - label: "GPLv3"
    description: "Copyleft — requires modifications to also be open source"
  - label: "No preference / keep existing"
    description: "Leave the license as-is or skip this decision"
```

**Question 2: Contributing**
```
header: "Contributing"
question: "Do you want a CONTRIBUTING.md with development setup instructions?"
options:
  - label: "Yes, detailed guide (Recommended)"
    description: "Full CONTRIBUTING.md with dev setup, PR process, and code standards"
  - label: "Yes, brief section only"
    description: "Just a short contributing section in the README linking to issues"
  - label: "No, skip for now"
    description: "Don't add contributing guidance yet"
```

**Question 3: Code of Conduct**
```
header: "Code of Conduct"
question: "Include a CODE_OF_CONDUCT.md? (Contributor Covenant is the standard)"
options:
  - label: "Yes, Contributor Covenant (Recommended)"
    description: "The most widely-adopted code of conduct for open source"
  - label: "No, skip for now"
    description: "Don't add a code of conduct yet"
```

**Question 4: Security Policy**
```
header: "Security"
question: "Include a SECURITY.md for vulnerability reporting?"
options:
  - label: "Yes (Recommended)"
    description: "Add a SECURITY.md with responsible disclosure instructions"
  - label: "No, skip for now"
    description: "Don't add a security policy yet"
```

Wait for the user's answers before proceeding.

### Step 2 — Generate the SVG logo and header

If the project doesn't have a logo, or the user asked for one, generate an SVG logo and header banner.

**First, ask about color palette:**

```
header: "Color Palette"
question: "Choose a color palette for the logo and header:"
options:
  - label: "Vibrant Tech (Recommended)"
    description: "Electric blue (#2563EB), neon cyan (#06B6D4), deep navy (#1E3A5F). Modern, energetic, perfect for dev tools and CLI apps."
    preview: "Primary: #2563EB | Accent: #06B6D4 | Dark: #1E3A5F | Light: #F0F9FF"
  - label: "Warm Startup"
    description: "Coral (#F43F5E), amber (#F59E0B), warm gray (#1F2937). Friendly, approachable, great for community projects."
    preview: "Primary: #F43F5E | Accent: #F59E0B | Dark: #1F2937 | Light: #FFF7ED"
  - label: "Forest Natural"
    description: "Emerald (#059669), teal (#0D9488), dark green (#064E3B). Calm, trustworthy, ideal for infrastructure and data tools."
    preview: "Primary: #059669 | Accent: #0D9488 | Dark: #064E3B | Light: #ECFDF5"
  - label: "Midnight Minimal"
    description: "Slate (#475569), indigo (#4338CA), near-black (#0F172A). Serious, professional, best for libraries and frameworks."
    preview: "Primary: #475569 | Accent: #4338CA | Dark: #0F172A | Light: #F8FAFC"
```

**Then generate the SVG files:**

Create two files in an `assets/` directory:

1. **`assets/logo.svg`** — A simple, clean logo (icon + project name). Keep it under 100px height. Use the chosen color palette. The logo should reflect the project's purpose:
   - CLI tools: terminal-inspired geometric shapes
   - Libraries: abstract, minimal symbol
   - Web apps: browser or UI-inspired icon
   - Data tools: node/graph or database-inspired shapes
   - Infrastructure: server or network-inspired geometry

2. **`assets/header.svg`** — A wide banner (1200×300 or similar ratio) for the top of the README. Include the project name in bold, a subtle tagline, and decorative elements from the logo. Use dark/light mode variants if possible (add `#gh-dark-mode-only` and `#gh-light-mode-only` image references).

**SVG design principles:**
- Use the chosen color palette consistently
- Keep paths clean and minimal — avoid excessive detail
- Include the project name in a clean, modern font (system fonts: system-ui, -apple-system, sans-serif)
- Make it responsive — use viewBox, not fixed pixel dimensions
- Add a subtle gradient or geometric background pattern for visual interest
- The header should work as a standalone banner at the top of the README

### Step 3 — Build the README structure

Write the README following the **14-section cognitive funneling structure** backed by research:

```
1. Logo/Banner        → Visual identity (use the generated SVG)
2. Badges             → CI, version, license, downloads (max 5 per line)
3. One-liner          → What the project does in a single sentence
4. Demo GIF/Screenshot → Show, don't tell — visual proof of value
5. Feature Highlights → Key differentiators with emoji icons
6. Quick Start        → Copy-paste commands to get running in 60 seconds
7. Table of Contents  → Navigation for longer READMEs
8. The "Why"          → Problem it solves, motivation, backstory
9. Installation       → Multi-platform setup instructions
10. Usage/Examples    → Code snippets showing real-world use
11. API Reference     → Or link to full docs
12. Contributing      → How to help (link to CONTRIBUTING.md)
13. License           → Brief identifier (GitHub auto-detects LICENSE file)
14. Acknowledgements  → Credits, inspirations, contributors
```

**Content rules (from research):**
- **One-liner:** Single paragraph, 30–400 chars, no bullets. Use bold for the project name. Example: `**rmb** is a CLI tool to install plugins for the Pi coding harness and Claude Code.`
- **Demo:** If the project is a CLI tool, include a terminal screenshot or ASCII cast. If it's a library, show a code example. If it's a web app, show a screenshot. Use `<p align="center">` for centering.
- **Features:** Use emoji-prefixed bullets. Each feature should be one line with a brief explanation. Link to docs where relevant.
- **Quick Start:** 3–5 copy-paste-ready commands. The user should be able to get something working in 60 seconds. Use fenced code blocks with language identifiers.
- **Installation:** Cover all supported platforms. Use collapsible `<details>` sections for platform-specific instructions if there are many.
- **Usage:** Show common workflows, not just API calls. Use syntax-highlighted code blocks.
- **Contributing:** Brief section linking to CONTRIBUTING.md. Include "good first issue" badge if applicable.

**Visual design rules (from research):**
- Use `##` for major sections, `###` for subsections. Never use `####`.
- Add emoji to section headers for visual navigation
- Use `<details>` for collapsible long content
- Add dark/light mode image variants where possible
- Keep non-code lines under 80 characters
- Use markdown tables for structured data (platform support, comparisons)
- Aggressively linkify — link to related projects, docs, and references
- Single empty lines between blocks, no indentation

**Anti-patterns to avoid (from research):**
- No "Coming soon" or "TODO" placeholders — remove empty sections
- No duplicate title (H1 matching repo name) — start with logo instead
- No wall of text without headers
- No excessive length (>2000 lines) — move detailed content to separate docs
- No badge abuse (>15 badges)
- No images without alt text
- No all-caps headers

### Step 4 — Create companion files

Based on the user's answers in Step 1, create the companion files:

**If MIT license chosen:**
Create `LICENSE` with the standard MIT license text (copyright year = current year, copyright holder = repo owner or "The [Project] Authors").

**If Apache 2.0 chosen:**
Create `LICENSE` with the standard Apache 2.0 text.

**If GPLv3 chosen:**
Create `LICENSE` with the standard GPLv3 text.

**If CONTRIBUTING.md requested:**
Create `CONTRIBUTING.md` with:
- Development environment setup (tools, versions, dependencies)
- How to build and run tests
- Code style and conventions
- Pull request process
- How to find something to work on (link to issues)
- Reference to CODE_OF_CONDUCT.md if it exists

**If CODE_OF_CONDUCT.md requested:**
Create `CODE_OF_CONDUCT.md` using the [Contributor Covenant](https://www.contributor-covenant.org/) v2.1 template. Add a contact email for reporting violations.

**If SECURITY.md requested:**
Create `SECURITY.md` with:
- How to report a vulnerability (email, not public issue)
- Response timeline commitment
- Supported versions table
- Security update disclosure process

### Step 5 — Score and iterate

After writing the README and companion files, run the `rmb score` tool to evaluate:

```bash
rmb score . --json
```

Parse the JSON output. If the score is **below 80 (A grade)**:

1. Identify the top 3 failed checks by points
2. Fix each one — add the missing section, improve the visual element, or fix the anti-pattern
3. Re-run `rmb score . --json`
4. Repeat until score ≥ 80 or no more improvements can be made without user input

If the score is stuck and can't reach 80, explain to the user which remaining gaps require their input (e.g., "We need a demo GIF — could you record one?" or "The project needs a SECURITY.md — would you like me to create one?").

**When the score reaches A (80+):**

Print a summary:
```
✅ README Score: XX/100 [A]
   Top Y% of open source READMEs

Files created/updated:
  • README.md
  • assets/logo.svg
  • assets/header.svg
  • LICENSE
  • CONTRIBUTING.md
  • CODE_OF_CONDUCT.md
  • SECURITY.md

The README is ready. Run `rmb score .` anytime to re-check.
```

### Step 6 — Final polish

After reaching an A:

1. Verify all links in the README are valid (check relative paths)
2. Ensure the logo and header render correctly (check image paths)
3. Confirm the README looks good on mobile (images use reasonable widths)
4. Make sure the one-line repo description on GitHub is compelling and under 120 characters
5. Suggest GitHub Topics that match the project (from Featured Topics where possible)

## Scoring Reference

The `rmb score` tool evaluates READMEs across 5 categories (100 points total):

| Category | Weight | What It Checks |
|----------|--------|----------------|
| Content Completeness | 35 pts | 14 essential sections: logo, badges, one-liner, demo, features, quick start, TOC, why, install, usage, API, contributing, license, acknowledgements |
| Visual Design | 25 pts | Logo, badge organization, header hierarchy, emojis, images, collapsible sections, syntax highlighting, link density, dark/light mode, tables, line length |
| Project Hygiene | 20 pts | LICENSE, CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md, CHANGELOG.md, CI badge, test mention, version badge, support channels |
| Cognitive Funneling | 15 pts | Section ordering: one-liner before install, demo before API, features before contributing, quick start before detailed install, license at bottom |
| Anti-Patterns | −15 pts | Deductions for placeholders, duplicate titles, wall of text, excessive length, badge abuse, missing alt text, all-caps headers |

**Grade scale:** A+ (90+) → A (80–89) → B (70–79) → C (60–69) → D (40–59) → F (0–39)

## Quick Reference: Section Templates

### Logo/Banner
```markdown
<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/header-dark.svg">
    <img src="assets/header.svg" alt="ProjectName" height="80px">
  </picture>
</p>
```

### Badges
```markdown
[![CI](https://github.com/user/repo/actions/workflows/ci.yml/badge.svg)](https://github.com/user/repo/actions)
[![Version](https://img.shields.io/crates/v/package.svg)](https://crates.io/crates/package)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
```

### One-liner
```markdown
**ProjectName** is a [type of tool] that [key benefit] for [target audience].
```

### Quick Start
```markdown
## 🚀 Quick Start

```bash
# Install
cargo install project-name

# Run
project-name --help
```
```

### Features
```markdown
## ✨ Features

- **🚀 Fast** — benchmarks show 3× faster than alternatives
- **🔌 Extensible** — plugin system with 50+ community plugins
- **📦 Zero config** — works out of the box with sensible defaults
```

### Installation
```markdown
## 📦 Installation

### macOS
```bash
brew install project-name
```

### Linux
```bash
curl -fsSL https://install.project.dev | bash
```

### From Source
```bash
git clone https://github.com/user/repo
cd repo
cargo build --release
```
```

### Contributing
```markdown
## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for
development setup, guidelines, and how to submit pull requests.

<a href="https://github.com/user/repo/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=user/repo" />
</a>
```

## Notes

- The `rmb` CLI tool must be installed and available on the PATH. If `rmb score` fails with "command not found", tell the user to install it with `cargo install --git https://github.com/kwhatcher/banger-readme`.
- SVG logos should be committed to the repository (typically in `assets/`). They are text files and version-control friendly.
- The skill assumes the user wants a complete, production-quality README. If they only want specific sections, they should use `/banger-readme --improve` and specify what they need.
- For projects that already have a strong README, focus on the gaps identified by `rmb score` rather than rewriting from scratch.
