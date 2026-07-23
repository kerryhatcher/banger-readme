# The Complete README Checklist & Source Index

> A comprehensive checklist for auditing your README, plus the complete index of all sources cited throughout this guide.

---

## The README Checklist

A synthesis of checklists from [Art of README](https://github.com/hackergrrl/art-of-readme#bonus-the-readme-checklist), [Daytona](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project), and [Utrecht University](https://utrechtuniversity.github.io/workshop-computational-reproducibility/chapters/readme-files.html).

### Content

- [ ] Logo or project banner (under 100px height)
- [ ] Badges (CI, coverage, version, license — max 5 per line, same height per line)
- [ ] One-liner describing the purpose of the project
- [ ] Demo GIF or screenshot showing the project in action
- [ ] Feature highlights (unique value propositions)
- [ ] Quick start guide (fewest possible commands, copy-paste ready)
- [ ] Table of contents (for READMEs longer than 2 scrolls)
- [ ] The "Why" — problem it solves, motivation, backstory
- [ ] Clear, runnable example of usage
- [ ] Installation instructions covering all supported platforms
- [ ] API documentation (or link to full docs)
- [ ] Contributing guide (addressed to a junior developer)
- [ ] License file in root directory (GitHub auto-detects it)
- [ ] Support channels (where to get help, report bugs, ask questions)

### Quality

- [ ] No broken links
- [ ] No empty sections or "Coming soon" placeholders
- [ ] Consistent naming conventions throughout
- [ ] Standardized formatting (80-char lines, single empty lines between blocks)
- [ ] Performs cognitive funneling (broad → specific)
- [ ] Caveats and limitations mentioned up-front
- [ ] Doesn't rely on externally hosted images for critical information
- [ ] Necessary background context and links provided
- [ ] Unfamiliar terms linked to informative sources
- [ ] Kept up to date with the project (version numbers, requirements)
- [ ] Addressed to a junior developer, not yourself
- [ ] No duplication of GitHub features (license, contributors, changelog)

### Visual Design

- [ ] Logo at top, under 100px height
- [ ] Badges in logical groups, max 5 per line, same height per line
- [ ] Consistent header levels (## for sections, ### for subsections, no ####)
- [ ] 80-character line width (except badges and links)
- [ ] No indentation — all text starts at left margin
- [ ] Single empty lines between blocks
- [ ] Demo GIF or screenshot showing core functionality
- [ ] Code examples with syntax highlighting
- [ ] Emojis or icons for section headers (consistent style)
- [ ] Collapsible sections for long content
- [ ] Aggressive linkification of terms, projects, and people
- [ ] Mobile-friendly image sizes

### Discovery

- [ ] Compelling one-line repo description (under 120 characters, appears on Trending/Topics)
- [ ] Relevant GitHub Topics selected (from Featured Topics where possible)
- [ ] Mix of broad and narrow topics for discoverability
- [ ] `package.json` keywords (for npm packages)
- [ ] Project website or landing page (GitHub Pages minimum)
- [ ] Social presence (Twitter/X, Discord, Mastodon)

### Project Hygiene

- [ ] `LICENSE` file in root directory
- [ ] `CONTRIBUTING.md` with development setup and PR process
- [ ] `CODE_OF_CONDUCT.md` (Contributor Covenant or custom)
- [ ] `SECURITY.md` for vulnerability reporting
- [ ] CI badge showing passing tests
- [ ] Coverage badge (if applicable)
- [ ] How to run tests documented
- [ ] `examples/` directory with runnable scenario-based code
- [ ] Repository About section fully completed
- [ ] Releases used for version history
- [ ] Issue templates for bugs and feature requests
- [ ] Custom labels for issue triage
- [ ] `good first issue` label on beginner-friendly issues

### The 60-Second Test

Give your README to someone unfamiliar with your project. Can they:

- [ ] Understand what it does in 10 seconds?
- [ ] Install and run it in 60 seconds?
- [ ] Tell if it's actively maintained?
- [ ] Know how to get help?
- [ ] Know if they can use it legally?
- [ ] Know how to contribute?

---

## Source Index

### Primary Sources (directly cited)

1. **[Awesome README](https://github.com/matiassingers/awesome-readme)** — Curated list of 100+ exemplary READMEs with analysis. 21.2k stars, 4k forks. Maintained by Matias Singers and 128 contributors. The definitive reference for README examples.

2. **[Art of README](https://github.com/hackergrrl/art-of-readme)** — Foundational essay on README psychology and structure by Kira (hackergrrl). 7.2k stars. Introduces "cognitive funneling," the pattern-matching brain model, and the altruistic mindset. Archived but timeless.

3. **[Readme Driven Development](https://tom.preston-werner.com/2010/08/23/readme-driven-development.html)** — Seminal 2010 essay by Tom Preston-Werner (GitHub co-founder) advocating writing the README before any code. The philosophical foundation of README-first development.

4. **[How to Write A 4000 Stars GitHub README](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project)** — Daytona's CEO Ivan Burazin on the research and strategy behind their 4k-star first week launch (2024). Practical, data-driven, covers both README content and GitHub platform strategy.

5. **[Elegant READMEs](https://www.yegor256.com/2019/04/23/elegant-readme.html)** — Yegor Bugayenko's opinionated, detailed guide on README formatting, structure, and anti-patterns. Covers badge layout rules, line width, header levels, and what NOT to include.

6. **[Top Ten Reasons Why I Won't Use Your Open Source Project](https://changelog.com/posts/top-ten-reasons-why-i-wont-use-your-open-source-project)** — Adam Stacoviak's classic 2011 Changelog article on what repels potential users. Still cited regularly as the definitive list of open source project failures.

7. **[How To Write A Great README](https://thoughtbot.com/blog/how-to-write-a-great-readme)** — thoughtbot's guide emphasizing writing quality, code examples, badges, and the importance of addressing known issues directly in the README.

8. **[What I Learned From an Old GitHub Project That Won 3,000 Stars in a Week](https://www.freecodecamp.org/news/what-i-learned-from-an-old-github-project-that-won-3-000-stars-in-a-week-628349a5ee14/)** — TOAST UI's KyuWoo Choi on how README improvements drove a 5× star conversion increase, plus GitHub Explore/Trending/Topics strategy. The best quantitative evidence for README impact.

9. **[ARCHITECTURE.md](https://matklad.github.io/2021/02/06/ARCHITECTURE.md.html)** — Alex Kladov (matklad) on why and how to add an architecture document alongside your README. Covers codemaps, invariants, boundaries, and cross-cutting concerns.

10. **[readme-best-practices](https://github.com/jehna/readme-best-practices)** — Copy-paste README template with 7 ready-made sections. 944 stars. Includes an animated hero GIF demo.

11. **[Best-README-Template](https://github.com/othneildrew/Best-README-Template)** — Feature-rich README template with TOC, badges, and all standard sections. Thousands of stars. Includes BLANK_README.md for starting fresh.

12. **[Best Practices for Writing Reproducible Code - README Files](https://utrechtuniversity.github.io/workshop-computational-reproducibility/chapters/readme-files.html)** — Utrecht University's academic perspective on README essentials. Emphasizes plain language, copy-paste-ready commands, and visual elements.

### Secondary Sources & Further Reading

- [Standard Readme](https://github.com/RichardLitt/standard-readme) — Lintable README specification with generator
- [Make a README](https://www.makeareadme.com/) — Interactive guide with live template
- [A Beginners Guide to Writing a Kickass README](https://meakaakka.medium.com/a-beginners-guide-to-writing-a-kickass-readme-7ac01da88ab3) — Akash Nimare's practical guide
- [15 Essential Sections Every README Needs](https://dev.to/georgekobaidze/15-essential-sections-every-readme-needs-give-your-project-what-it-deserves-fie) — George Kobaidze on dev.to
- [How to Write a Beginner-Friendly README](https://www.readmecodegen.com/blog/beginner-friendly-readme-guide-open-source-projects) — readmecodegen.com
- [Best Practices for Writing README Files for GitHub Projects](https://medium.com/@sidragillani/best-practices-for-writing-readme-files-for-github-projects-fe89f76d0e02) — Sidra Gillani on Medium
- [Two Open Source Projects with Great Documentation](https://johnjago.com/great-docs/) — John Jago
- [How to Design an Attractive GitHub Profile README](https://medium.com/design-bootcamp/how-to-design-an-attractive-github-profile-readme-3618d6c53783) — Piyush Malhotra
- [Awesome GitHub Profile README](https://github.com/abhisheknaiidu/awesome-github-profile-readme) — 30.6k stars, curated profile README examples
- [Awesome README Templates](https://github.com/elangosundar/awesome-README-templates) — 1.1k stars, diverse template collection
- [Common Readme](https://github.com/hackergrrl/common-readme) — A common readme style for Node, with guide and generator
- [Zalando's README Template](https://github.com/zalando/zalando-howto-open-source/blob/master/READMEtemplate.md) — Simple template covering all basics
- [Amazing GitHub Template](https://github.com/dec0dOS/amazing-github-template) — Full project scaffolding (README + LICENSE + CONTRIBUTING + CODE_OF_CONDUCT + SECURITY)

### Notable Example READMEs

- [gofiber/fiber](https://github.com/gofiber/fiber#readme) — Gold standard for web frameworks
- [httpie/httpie](https://github.com/httpie/httpie#readme) — Gold standard for CLI tools
- [PostHog/posthog](https://github.com/PostHog/posthog#readme) — Gold standard for full applications
- [doomemacs/doomemacs](https://github.com/doomemacs/doomemacs#readme) — Personality-driven, beautifully organized
- [Day8/re-frame](https://github.com/Day8/re-frame#readme) — Essay-style deep documentation
- [NASA/ogma](https://github.com/NASA/ogma#readme) — Clean, comprehensive, institutional quality
- [dbt-labs/dbt-core](https://github.com/dbt-labs/dbt-core#readme) — Newcomer-friendly, community-focused
- [lobehub/lobe-chat](https://github.com/lobehub/lobe-chat#readme) — Modern, visually stunning, ecosystem-oriented
- [Grigorij-Dudnik/Clean-Coder-AI](https://github.com/Grigorij-Dudnik/Clean-Coder-AI#readme) — Trailer video, motion GIFs, schema diagrams
- [GyulyVGC/sniffnet](https://github.com/GyulyVGC/sniffnet#readme) — Custom badges, tabular downloads, social footer
- [create-go-app/cli](https://github.com/create-go-app/cli#readme) — Video screencast, terminal demos, FAQ
- [choojs/choo](https://github.com/choojs/choo#readme) — Menu above the fold, FAQ, backers
- [release-it/release-it](https://github.com/release-it/release-it#readme) — Demo GIF, expandable TOC, detailed releases
- [skydio/revup](https://github.com/Skydio/revup#readme) — Tutorial GIFs for each stage
- [dutrevis/spark-resources-metrics-plugin](https://github.com/dutrevis/spark-resources-metrics-plugin#readme) — Mermaid diagrams, expandable blocks
- [emalderson/thephish](https://github.com/emalderson/ThePhish#readme) — UML diagrams, issue/PR templates
- [gitpoint/git-point](https://github.com/gitpoint/git-point#readme) — App Store badges, clean screenshots
- [thelounge/thelounge](https://github.com/thelounge/thelounge#readme) — Step-by-step for multiple scenarios
- [vhesener/Closures](https://github.com/vhesener/Closures#readme) — Cognitive funnel, animated examples, color-coordinated
- [sourcerer-io/sourcerer-app](https://github.com/sourcerer-io/sourcerer-app#readme) — Clean animated screenshot, customized CTA badge

### Tools Referenced

- [readme-md-generator](https://github.com/kefranabg/readme-md-generator) — CLI README generator
- [Readme Forge](https://readme-forge.github.io/) — Component-based README generator
- [Make a README](https://www.makeareadme.com/) — Interactive guide with live template
- [Standard Readme](https://github.com/RichardLitt/standard-readme) — Lintable README spec
- [READMINE](https://github.com/mhucka/readmine) — Self-describing README template
- [Best-README-Template](https://github.com/othneildrew/Best-README-Template) — Feature-rich template
- [Amazing GitHub Template](https://github.com/dec0dOS/amazing-github-template) — Full project scaffolding
- [StackEdit](https://stackedit.io/) — Online Markdown editor
- [GitHub Readme Stats](https://github.com/anuraghazra/github-readme-stats) — Dynamic stats cards
- [maintainer.io](https://maintainer.io/) — Free README feedback
- [Gifski](https://github.com/sindresorhus/Gifski) — High-quality GIF creation
- [ScreenToGif](https://github.com/NickeManarin/ScreenToGif/) — Open source GIF editor
- [vhs](https://github.com/charmbracelet/vhs) — Scriptable terminal GIFs
- [terminalizer](https://github.com/faressoft/terminalizer) — Terminal recording
- [LICEcap](https://www.cockos.com/licecap/) — Quick screen captures
- [GPRM](https://github.com/VishwaGauravIn/github-profile-readme-maker) — Profile README maker
- [Hall-of-fame](https://github.com/sourcerer-io/hall-of-fame) — Auto-updating contributors
- [README Typing SVG](https://github.com/DenverCoder1/readme-typing-svg) — Typing animation SVG
- [user-statistician](https://github.com/cicirello/user-statistician) — User activity SVG
- [GitHub PR Stats](https://github.com/f14XuanLv/github-pr-stats) — PR statistics SVG
- [Github Licenses Stats](https://github.com/lheintzmann1/github-licenses-stats) — License statistics SVG
- [Telegram Card](https://github.com/Malith-Rukshan/telegram-card) — Telegram preview cards

---

## Document Map

This research is organized into the following files:

| File | Content |
|------|---------|
| [readme-best-practices-research.md](readme-best-practices-research.md) | **Index & overview** — start here |
| [why-readmes-matter.md](why-readmes-matter.md) | The data, psychology, and philosophy |
| [anatomy-of-a-readme.md](anatomy-of-a-readme.md) | Essential structure, header, and body |
| [project-hygiene.md](project-hygiene.md) | License, contributing, CoC, tests, support |
| [aesthetics-and-visual-design.md](aesthetics-and-visual-design.md) | Formatting, visual elements, linkification |
| [emerging-patterns.md](emerging-patterns.md) | Trends 2024–2026 |
| [anti-patterns.md](anti-patterns.md) | What repels users, common mistakes |
| [exemplary-readmes.md](exemplary-readmes.md) | Curated examples by category |
| [tools-and-companion-docs.md](tools-and-companion-docs.md) | Generators, templates, ARCHITECTURE.md, etc. |
| [checklist-and-sources.md](checklist-and-sources.md) | **This file** — complete checklist and source index |

---

*Research compiled July 2026. The README landscape evolves — the [Awesome README](https://github.com/matiassingers/awesome-readme) list is actively maintained and is the best ongoing resource for discovering new exemplary READMEs.*
