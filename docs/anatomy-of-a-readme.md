# Anatomy of a Standout README

> The essential structure, the header that hooks, and the body that converts interest into commitment.

---

## The Complete Section Map

Based on analysis of the [Awesome README](https://github.com/matiassingers/awesome-readme) list (21.2k stars, 4k forks, 128 contributors), the [readme-best-practices](https://github.com/jehna/readme-best-practices) template (944 stars), and the [Best-README-Template](https://github.com/othneildrew/Best-README-Template) (thousands of stars), a great README follows this structure:

| # | Section | Purpose | Time to Read |
|---|---------|---------|-------------|
| 1 | **Logo & Banner** | Visual identity, brand recognition | 1 second |
| 2 | **Badges** | Trust signals at a glance (CI, coverage, version, license) | 2 seconds |
| 3 | **One-Liner / Elevator Pitch** | What the project does in a single sentence | 5 seconds |
| 4 | **Demo (GIF/Screenshot)** | Show, don't tell — visual proof of value | 10 seconds |
| 5 | **Feature Highlights** | Key differentiators, unique value propositions | 15 seconds |
| 6 | **Quick Start** | Fewest possible commands to get running | 20 seconds |
| 7 | **Table of Contents** | Navigation for longer READMEs | 5 seconds |
| 8 | **The "Why"** | Problem it solves, motivation, backstory | 30 seconds |
| 9 | **Installation** | Detailed setup instructions | 1–2 minutes |
| 10 | **Usage / Examples** | Code snippets showing real-world use | 2–5 minutes |
| 11 | **API Reference** | (or link to full docs) | varies |
| 12 | **Contributing Guide** | How to help, development setup | 2–5 minutes |
| 13 | **License** | Legal clarity | 5 seconds |
| 14 | **Acknowledgements / Credits** | Humans behind the project | 15 seconds |

Sources: [Awesome README](https://github.com/matiassingers/awesome-readme), [readme-best-practices](https://github.com/jehna/readme-best-practices), [Best-README-Template](https://github.com/othneildrew/Best-README-Template), [Daytona guide](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project)

---

## The Header: First Impressions Are Everything

The header segment is critical — it captures most readers' attention. Fail here, and you risk losing them forever. This is the "above the fold" content that appears before any scrolling.

([Daytona, 2024](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project))

### 1. Logo

Begin with your project's logo. This is valuable real estate for making a strong first impression. Many projects overlook this, but it's one of the highest-ROI additions you can make.

**Rules:**
- Keep it under **100px in height** — use the `height` attribute in the `<img>` tag (one dimension is enough for HTML to scale proportionally)
- Place it at the very top, before any text
- Follow with an empty line after the logo
- Don't make it wider than the README content area

```markdown
<img src="https://example.com/logo.png" height="64px"/>

[rest of README starts here...]
```

**Why it matters:** In Daytona's case, they decided their initial logo needed a redesign before launch, and "after some stress about the timing, we successfully now have a shiny new logo." The logo is the visual anchor that makes your project memorable.

([Elegant READMEs, Yegor Bugayenko](https://www.yegor256.com/2019/04/23/elegant-readme.html), [Daytona, 2024](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project))

### 2. Badges

Badges convey vital information about project health, build status, and quality at a glance. They instill confidence that your project adheres to industry best practices.

**The Rules of Badge Layout:**

1. **No more than 5 badges per line** — more creates visual chaos
2. **Group by logical category:**
   - Line 1: CI, code coverage, code quality (technical health)
   - Line 2: Version, downloads, license, community (management/distribution)
3. **All badges on the same line must be the same height** — if a badge has a different height, give it its own line
4. **Separate badge lines with an empty line**
5. **Be judicious** — for each badge, ask: "What real value does this provide to the typical viewer of this README?"

```markdown
[![Build Status](https://travis-ci.org/user/repo.svg)](https://travis-ci.org/user/repo)
[![Coverage Status](https://coveralls.io/repos/user/repo/badge.svg)](https://coveralls.io/r/user/repo)
[![Code Quality](https://api.codacy.com/project/badge/Grade/abc123)](https://www.codacy.com/app/user/repo)

[![npm version](https://badge.fury.io/js/package.svg)](https://badge.fury.io/js/package)
[![Downloads](https://img.shields.io/npm/dm/package.svg)](https://www.npmjs.com/package/package)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
```

**The badge abuse warning:** Badges are easy to abuse. They add visual noise and generally only function when the user is reading your Markdown in a browser online (since images are often hosted elsewhere). Consider: does a CI badge showing build status better serve its audience by emailing maintainers or automatically creating an issue? Always consider whether there's a better flow for that data.

([Daytona, 2024](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project), [Elegant READMEs](https://www.yegor256.com/2019/04/23/elegant-readme.html), [Art of README](https://github.com/hackergrrl/art-of-readme#bonus-other-good-practices))

### 3. The Elevator Pitch

A catchy one-liner that encapsulates what your project is about, followed by a subtitle with additional context. This is the hook that piques interest.

**Format:** A single paragraph — no bullets, no new lines, no indentations. Just plain text that fits the entire idea into one paragraph. Use bold/emphasis to call out key terms.

```markdown
**MyProject** is a [key technology] [type of tool] for [target audience].
Its key benefits include: 1) benefit one, 2) benefit two,
and 3) benefit three.
```

**Real example from Takes (Java web framework):**

```markdown
**Takes** is a
[true object-oriented](https://www.yegor256.com/2014/11/20/seven-virtues-of-good-object.html)
and [immutable](https://www.yegor256.com/2014/06/09/objects-should-be-immutable.html)
Java7 web development framework. Its key benefits,
comparing to all others, include four fundamental principles:
1) not a single `null`, 2) not a single `public` `static`
method, 3) not a single mutable class, and 4) not
a single `instanceof` keyword, type casting, or reflection.
```

**Key points:**
- No heading yet — just logo, badges, and plain text
- Use markdown links to point to relevant blog posts, videos, or documentation
- Fit the entire idea into a single paragraph
- This is not the place for philosophy or prose — save that for later

([Elegant READMEs](https://www.yegor256.com/2019/04/23/elegant-readme.html), [Daytona, 2024](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project))

### 4. Demo: Show, Don't Tell

Humans are visual creatures. A well-executed image, GIF, or animation conveys functionality and value more effectively than words alone.

**What makes a great demo:**
- Shows the **core workflow** in 5–15 seconds
- Demonstrates the **most impressive feature** first
- Is **self-explanatory** — the viewer should understand what's happening without reading captions
- Uses **appropriate speed** — not too fast to follow, not too slow to bore

**GIF creation tools recommended by the community:**

| Tool | Best For | Notes |
|------|----------|-------|
| [Gifski](https://github.com/sindresorhus/Gifski) | High-quality color GIFs | More vivid colors, keeps size low |
| [ScreenToGif](https://github.com/NickeManarin/ScreenToGif/) | Windows screen recording | Open source, customizable UI, easily editable |
| [vhs](https://github.com/charmbracelet/vhs) | Terminal demos | Scriptable, beautiful terminal GIFs |
| [terminalizer](https://github.com/faressoft/terminalizer) | Terminal recording | Generate GIFs or share web player |
| [LICEcap](https://www.cockos.com/licecap/) | Quick captures | Classic favorite, stands test of time |
| [Giphy Capture](https://giphy.com/apps/giphycapture) | Mac quick captures | Easy upload to giphy.com |
| [ttystudio](https://github.com/chjj/ttystudio) | Terminal-to-GIF | "Minus the headaches" |

([Daytona, 2024](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project), [Awesome README - Creating GIFs](https://github.com/matiassingers/awesome-readme#creating-gifs))

### 5. Feature Highlights

Once you've captured attention, list your project's most compelling features. Highlight the unique value propositions that set your project apart.

**Format options:**

**Bulleted list (most common):**
```markdown
## Features

- **🚀 Lightning fast** — benchmarks show 3× faster than alternatives
- **🔌 Plugin system** — extend with 50+ community plugins
- **📦 Zero config** — works out of the box with sensible defaults
- **🌍 i18n ready** — built-in support for 20+ languages
```

**Table format (for comparison-heavy projects):**
```markdown
| Feature | MyProject | Alternative A | Alternative B |
|---------|-----------|---------------|---------------|
| Speed | 3× faster | Baseline | 0.8× |
| Bundle size | 5KB | 45KB | 12KB |
| TypeScript | Native | Plugin | Partial |
```

([Daytona, 2024](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project))

### 6. Quick Start

Provide a streamlined guide with as few commands as possible to get users running. This is the "try it now" moment — the make-or-break point where interest converts to action.

**The golden rule:** A user should be able to copy, paste, and see something working in under 60 seconds.

```markdown
## Quick Start

```bash
# Install
npm install my-project

# Create a new project
npx my-project init my-app

# Start the dev server
cd my-app && npm start
```

Open [http://localhost:3000](http://localhost:3000) to see your app.
```

**Critical rules for Quick Start:**
- Use **copy-paste-ready commands** — assume the reader doesn't know your project yet
- Ensure commands **actually work** — test them in a clean environment
- Show **visible output or result** — the user should see something happen
- Keep it to **3–5 commands maximum**
- If there are prerequisites, list them concisely before the commands

([Daytona, 2024](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project), [Utrecht University](https://utrechtuniversity.github.io/workshop-computational-reproducibility/chapters/readme-files.html))

---

## The Body: From Interest to Commitment

Once a reader has made it past the header, they're interested. Now you need to convert that interest into commitment — either using the project or contributing to it.

### 7. Table of Contents

For READMEs longer than about two scrolls, a table of contents is essential. It provides:
- **Quick navigation** to specific sections
- **A mental map** of what the README covers
- **Professional appearance** — it signals that the document is well-organized

```markdown
## Table of Contents

- [Features](#features)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Usage](#usage)
- [API Reference](#api-reference)
- [Contributing](#contributing)
- [License](#license)
```

**Pro tip:** GitHub automatically generates anchor links for headings. Use all-lowercase, hyphen-separated versions of your heading text.

### 8. The "Why"

Clarify from the reader's perspective why they should engage with your project. Address their needs or challenges directly. This section answers: "Why does this exist? What problem does it solve that other solutions don't?"

**Good "Why" sections:**
- Identify the specific pain point
- Explain why existing solutions fall short
- Describe the insight that led to this approach
- Use concrete examples of the problem

**Bad "Why" sections:**
- "I wanted to learn Rust" (irrelevant to the user)
- "This is a fork of X but better" (vague, defensive)
- Long personal narratives without connecting to user needs

([Daytona, 2024](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project))

### 9. Installation

This is an extended version of the Quick Start. Delve deeper into the how-tos. Cover multiple platforms, package managers, and build methods.

**Structure by platform:**
```markdown
## Installation

### macOS
```bash
brew install my-project
```

### Linux
```bash
curl -fsSL https://install.myproject.dev | bash
```

### Windows
```powershell
winget install MyProject
```

### From Source
```bash
git clone https://github.com/user/my-project
cd my-project
cargo build --release
```
```

**Essential rules:**
- Use **copy-paste-ready commands** — assume the reader doesn't know your project yet ([Utrecht University](https://utrechtuniversity.github.io/workshop-computational-reproducibility/chapters/readme-files.html))
- Include **working commands** — ensure examples actually run in a clean environment
- Address your text to a **junior programmer**, not yourself. Your grandma should be able to understand you. ([Elegant READMEs](https://www.yegor256.com/2019/04/23/elegant-readme.html))
- Don't teach us frameworks. Tell us how to use them in *this specific case*. "Don't tell me 'first, you have to learn Docker.' No, I don't. If I did, I'd have done it myself already. Tell me how to use it in this specific case, and spare me all the rest." ([Elegant READMEs](https://www.yegor256.com/2019/04/23/elegant-readme.html))
- Call out **known issues and workarounds** — if there's a persistent dependency problem, address it right in the README. thoughtbot's capybara-webkit added a notice about Qt dependency issues directly in the README. ([thoughtbot](https://thoughtbot.com/blog/how-to-write-a-great-readme))

### 10. Usage & Code Examples

Developers love code samples. A few lines of syntax-highlighted source are worth a thousand words. `plataformatec/simple_form` does an excellent job of providing code examples alongside explanations for nearly every setting and interface call — and as a result, their README is the top Google hit for nearly all `simple_form` searches.

([thoughtbot](https://thoughtbot.com/blog/how-to-write-a-great-readme))

**Best practices for code examples:**

1. **Include runnable example files in your repo** — e.g., `example.js` that users can actually execute if they clone the repository ([Art of README](https://github.com/hackergrrl/art-of-readme#bonus-other-good-practices))

2. **For CLI tools:** Show command invocations and their output. If you create or modify a file, `cat` it to demonstrate the change before and after. ([Art of README](https://github.com/hackergrrl/art-of-readme#bonus-other-good-practices))

3. **For stateless function libraries:** A Node REPL session of function calls and results can communicate usage more clearly than a source file. ([Art of README](https://github.com/hackergrrl/art-of-readme#bonus-other-good-practices))

4. **Use collapsible sections for long examples:**
```markdown
<details>
<summary>Advanced: Custom configuration</summary>

```js
// Long example here...
```

</details>
```

5. **Show common workflows, not just API calls.** Users want to see how the pieces fit together in a realistic scenario.

### 11. API Reference

The API section should detail objects, functions, signatures, return types, callbacks, and events. But keep the README concise — link to full API docs for exhaustive detail.

**What to include in the README's API section:**
- Which parameters are optional, and their defaults
- Type information where not obvious from convention
- For `opts` object parameters, all keys and values accepted
- Caveats made clear up-front
- A tiny example of each major function's use if not obvious from the Usage section

**What to link out to:**
- Full method signatures
- Edge cases and unusual use cases
- Internal/private APIs
- Generated documentation (Javadoc, TypeDoc, etc.)

**API formatting is highly bikesheddable.** Use whatever format you think is clearest, but be consistent. Some prefer tables, others prefer structured headings. The key is that a developer can find what they need without reading the source code.

([Art of README](https://github.com/hackergrrl/art-of-readme#key-elements), [Art of README - API formatting](https://github.com/hackergrrl/art-of-readme#bonus-other-good-practices))

### 12. Keep It Concise

Avoid unnecessarily long READMEs. They can deter users who may perceive the project as overly complex.

> *"The ideal README is as short as it can be without being any shorter. Detailed documentation is good — make separate pages for it! — but keep your README succinct."*
> — Kira, [Art of README](https://github.com/hackergrrl/art-of-readme#brevity)

**When to move content out of the README:**
- Detailed API documentation → dedicated docs site or wiki
- Changelog → GitHub Releases tab
- Full contributor list → GitHub Contributors tab
- Long tutorials → separate `docs/` directory
- FAQ → separate `FAQ.md` or docs site

**When to keep content in the README:**
- The one-liner description
- Quick start commands
- Core feature list
- Basic usage examples
- Installation instructions
- License identifier
- How to contribute (brief)

---

## Section Ordering: The Cognitive Funnel in Practice

The ordering of sections is not arbitrary. It follows the cognitive funneling principle: broadest, most pertinent details first, progressively narrowing to specific details for committed readers.

### The Ideal Order (with rationale)

```
1. Logo              → Visual anchor, brand
2. Badges            → Trust signals, 2-second scan
3. One-liner         → "What is this?" — 5 seconds
4. Demo GIF          → "Show me" — 10 seconds
5. Features          → "What can it do?" — 15 seconds
6. Quick Start       → "Let me try it" — 30 seconds
7. Table of Contents → Navigation for the committed
8. The Why           → Context for the interested
9. Installation      → Detailed setup
10. Usage/Examples   → Real-world application
11. API Reference    → Technical depth
12. Contributing     → How to help
13. License          → Legal
14. Credits          → Humans
```

### Why This Order Works

Each section answers the natural next question a reader has:

- "What is this?" → Logo, one-liner
- "Is it any good?" → Badges, demo
- "What can it do for me?" → Features
- "How do I try it?" → Quick Start
- "Should I invest time in this?" → The Why, Installation
- "How do I actually use it?" → Usage, API
- "Can I contribute?" → Contributing
- "Can I use this legally?" → License

A reader can bail at any point with the information they need to make that decision. This is the essence of respecting people's time.

([Art of README](https://github.com/hackergrrl/art-of-readme#cognitive-funneling))

---

## Template: The Minimal Viable README

For projects that want to start simple and grow:

```markdown
<img src="logo.png" height="64px"/>

[![CI](https://img.shields.io/github/actions/workflow/status/user/repo/ci.yml)](https://github.com/user/repo/actions)
[![Version](https://img.shields.io/npm/v/package.svg)](https://www.npmjs.com/package/package)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**ProjectName** is a [one-line description of what it does].

## Quick Start

```bash
npm install project-name
npx project-name --help
```

## Features

- Feature one
- Feature two
- Feature three

## Installation

Detailed install instructions...

## Usage

```js
const project = require('project-name')
// Example code...
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT — see [LICENSE](LICENSE) for details.
```

This template covers the essentials and can be expanded as the project grows. The key is that every section provides real value — no placeholders, no "Coming soon."

---

*Previous: [Why the README Matters](why-readmes-matter.md)*
*Next: [Project Hygiene](project-hygiene.md) — trust signals that make or break adoption*
