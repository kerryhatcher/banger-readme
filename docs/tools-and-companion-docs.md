# Tools, Generators & Companion Documents

> Tools to create better READMEs, and the companion documents that complete your project's documentation suite.

---

## README Generators & Templates

### Interactive Generators

| Tool | Description | Best For |
|------|-------------|----------|
| [readme-md-generator](https://github.com/kefranabg/readme-md-generator) | CLI that generates beautiful README.md files by asking questions about your project | Quick, interactive setup |
| [Readme Forge](https://readme-forge.github.io/) | Component-based README generator with extensive template library | Custom, modular READMEs |
| [Make a README](https://www.makeareadme.com/) | Guide with editable template and live Markdown rendering | Learning while creating |
| [GPRM](https://github.com/VishwaGauravIn/github-profile-readme-maker#readme) | Tool to generate customized GitHub Profile README with modern UI | Profile READMEs |
| [StackEdit](https://stackedit.io/) | Online Markdown editor for README customization | WYSIWYG editing |

### Templates

| Template | Description | Sections Included |
|----------|-------------|-------------------|
| [readme-best-practices](https://github.com/jehna/readme-best-practices) | Copy-paste template, 944 stars | Installing, Developing, Features, Configuration, Contributing, Links, Licensing |
| [Best-README-Template](https://github.com/othneildrew/Best-README-Template) | Feature-rich template, thousands of stars | TOC, About, Getting Started, Usage, Roadmap, Contributing, License, Contact, Acknowledgments |
| [Standard Readme](https://github.com/RichardLitt/standard-readme) | Lintable README specification with generator | Background, Install, Usage, API, Contributing, License |
| [READMINE](https://github.com/mhucka/readmine) | Thorough, clear, self-describing template | Table of contents, Introduction, Installation, Usage, Getting help, Contributing, License, Acknowledgments |
| [Amazing GitHub Template](https://github.com/dec0dOS/amazing-github-template) | README + LICENSE + CONTRIBUTING + CODE_OF_CONDUCT + SECURITY + Issues + PR templates | Full project scaffolding |
| [Zalando's README Template](https://github.com/zalando/zalando-howto-open-source/blob/master/READMEtemplate.md) | Simple template covering all basics | Title, Description, Installation, Usage, Contributing, License |

### Dynamic Content Tools

| Tool | Description |
|------|-------------|
| [GitHub Readme Stats](https://github.com/anuraghazra/github-readme-stats) | Dynamically generated customizable GitHub cards: stats, top languages, WakaTime |
| [Hall-of-fame](https://github.com/sourcerer-io/hall-of-fame) | Auto-updating contributor recognition, updates every hour |
| [README Typing SVG](https://github.com/DenverCoder1/readme-typing-svg#readme) | Dynamically generated SVG that gives the appearance of typing and deleting text |
| [user-statistician](https://github.com/cicirello/user-statistician) | GitHub Action generating SVG of detailed user activity for profile READMEs |
| [GitHub PR Stats](https://github.com/f14XuanLv/github-pr-stats#readme) | Dynamic SVG tables displaying GitHub pull requests with dual modes |
| [Github Licenses Stats](https://github.com/lheintzmann1/github-licenses-stats#readme) | Dynamic SVG showing top licenses used across your GitHub repositories |
| [Telegram Card](https://github.com/Malith-Rukshan/telegram-card#readme) | Dynamic preview card generator for Telegram channels, groups, and bots |

---

## GIF Creation Tools

Embedding an animated GIF in your README quickly demonstrates what your project does and catches the reader's eye.

| Tool | Platform | Notes |
|------|----------|-------|
| [Gifski](https://github.com/sindresorhus/Gifski) | macOS | More vivid colors, keeps size low |
| [ScreenToGif](https://github.com/NickeManarin/ScreenToGif/) | Windows | Open source, customizable UI, easily editable GIFs |
| [vhs](https://github.com/charmbracelet/vhs) | Cross-platform | Scriptable terminal GIF generator, beautiful output |
| [terminalizer](https://github.com/faressoft/terminalizer) | Cross-platform | Record terminal, generate GIF or share web player |
| [LICEcap](https://www.cockos.com/licecap/) | Windows, macOS | Classic favorite, simple and reliable |
| [Giphy Capture](https://giphy.com/apps/giphycapture) | macOS | Easy upload to giphy.com |
| [ttystudio](https://github.com/chjj/ttystudio) | Cross-platform | Terminal-to-GIF "minus the headaches" |

([Awesome README - Creating GIFs](https://github.com/matiassingers/awesome-readme#creating-gifs))

---

## Feedback & Review Tools

| Tool | Description |
|------|-------------|
| [maintainer.io](https://maintainer.io/) | Free README standardization and feedback — click "Book an audit" |

---

## Companion Documents

A great README is the centerpiece, but a complete documentation suite includes several companion files.

### ARCHITECTURE.md

For projects in the 10k–200k lines of code range, an `ARCHITECTURE.md` document bridges the gap between occasional contributors and core developers.

**The problem it solves:** It takes ~2× more time to write a patch if you're unfamiliar with a project, but ~10× more time to figure out *where* to change the code. An architecture document provides the mental map that core developers have but newcomers lack.

([matklad, 2021](https://matklad.github.io/2021/02/06/ARCHITECTURE.md.html))

**What to include:**

1. **Bird's eye overview** of the problem being solved
2. **Codemap** — coarse-grained modules and their relationships. Should answer "where's the thing that does X?" and "what does the thing I'm looking at do?"
3. **Architectural invariants** — especially absences (e.g., "nothing in the model layer depends on views"). These are hard to divine from reading code.
4. **Boundaries** between layers and systems. Good boundaries have "measure zero" — you won't find them by randomly looking at code.
5. **Cross-cutting concerns** — logging, error handling, authentication, etc.

**Rules for ARCHITECTURE.md:**
- Keep it short — every recurring contributor will read it
- Only specify things unlikely to frequently change
- Name important files, modules, and types but don't directly link them (links go stale)
- Encourage symbol search to find mentioned entities
- Revisit a couple of times a year — don't try to keep it synchronized with every code change

**Notable examples:**
- [rust-analyzer architecture](https://github.com/rust-analyzer/rust-analyzer/blob/d7c99931d05e3723d878bea5dc26766791fa4e69/docs/dev/architecture.md) — the example matklad cites
- [esbuild architecture](https://github.com/evanw/esbuild/blob/main/docs/architecture.md) — great use of graphics for visualizations and project structure
- [Tauri architecture](https://github.com/tauri-apps/tauri/blob/dev/ARCHITECTURE.md) — well-made source code map, discusses architecture considerations
- [Redis README](https://github.com/redis/redis/blob/unstable/README.md) — good source code map, overviews of key files
- [Neovim src README](https://github.com/neovim/neovim/blob/master/src/nvim/README.md) — describes main processes and lifecycle
- [VS Code Source Code Organization](https://github.com/microsoft/vscode/wiki/Source-Code-Organization) — good use of high-level diagrams
- [Flutter Engine Architecture](https://github.com/flutter/flutter/blob/master/docs/about/The-Engine-architecture.md) — high-level diagrams showing the stack and its parts
- [Oh My Zsh Design](https://github.com/ohmyzsh/ohmyzsh/wiki/Design) — describes initialization process, environment requirements
- [Linux cryptography architecture](https://github.com/torvalds/linux/blob/master/Documentation/crypto/architecture.rst) — calls out different component types, invariants, structure with diagrams
- [GitLab architecture](https://gitlab.com/gitlab-org/charts/gitlab/-/tree/master/doc/architecture) — calls out design decisions

([Awesome README - Architecture Examples](https://github.com/matiassingers/awesome-readme#architecture-examples))

### SECURITY.md

Create a `SECURITY.md` file in your project's root directory outlining the process for reporting vulnerabilities. This signals that you take security seriously and provides a clear channel for responsible disclosure.

```markdown
# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability, please **do not** open a public issue.
Instead, email [security@myproject.dev](mailto:security@myproject.dev).

We will respond within 48 hours and work with you on a fix and disclosure timeline.

## Supported Versions

| Version | Supported |
|---------|-----------|
| 2.x | ✅ Active |
| 1.x | ⚠️ Security fixes only |
| 0.x | ❌ End of life |
```

([Daytona, 2024](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project))

### CONTRIBUTING.md

Separate detailed contribution guidelines into their own file. Keep the README's contributing section brief and link to the full guide.

**What CONTRIBUTING.md should cover:**
- Development environment setup
- How to build and test
- Code style and conventions
- Pull request process
- How to find something to work on
- Code of conduct reference

### CHANGELOG.md

Use GitHub's Releases tab instead of maintaining a separate changelog file. If you do maintain a changelog, keep it as a separate file — don't put it in the README.

([Elegant READMEs](https://www.yegor256.com/2019/04/23/elegant-readme.html))

### CODE_OF_CONDUCT.md

The [Contributor Covenant](https://www.contributor-covenant.org/) is the most widely-adopted code of conduct. Copy it, add your contact email, and reference it from your README and CONTRIBUTING.md.

---

## The Complete Documentation Suite

For a fully mature open source project, the documentation suite should include:

```
project/
├── README.md              # Front door — what, why, quick start
├── ARCHITECTURE.md         # Code map for contributors
├── CONTRIBUTING.md         # How to contribute
├── CODE_OF_CONDUCT.md      # Community standards
├── SECURITY.md             # Vulnerability reporting
├── LICENSE                 # Legal terms
├── CHANGELOG.md            # (or use GitHub Releases)
├── docs/                   # Detailed documentation
│   ├── installation.md
│   ├── usage.md
│   ├── api/
│   └── guides/
└── examples/               # Runnable example code
```

Not every project needs all of these from day one. Start with README + LICENSE. Add CONTRIBUTING.md when you want contributions. Add ARCHITECTURE.md when the codebase exceeds ~10k lines. Add SECURITY.md when you have users depending on your project.

---

*Previous: [Exemplary READMEs](exemplary-readmes.md)*
*Next: [Checklist & Source Index](checklist-and-sources.md) — the complete checklist and all sources*
