# Project Hygiene: Trust Signals

> The infrastructure of trust — license, contributing guide, code of conduct, tests, and support channels that make or break adoption.

---

## Why Hygiene Matters

While captivating narratives and visually stunning presentations are essential, true sustainability in the open-source realm hinges on impeccable project hygiene. These fundamental elements serve as the backbone of a thriving, collaborative ecosystem. A project with a beautiful README but no license, no tests, and no contributing guide is a house with a fresh coat of paint and no foundation.

([Daytona, 2024](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project))

---

## License

### The Most Fundamental Trust Signal

Just because code is published online doesn't mean it's free for the taking in the public domain. For someone to use your code, they need to know the stipulations for doing so. Make it easy.

([Changelog - Top Ten Reasons](https://changelog.com/posts/top-ten-reasons-why-i-wont-use-your-open-source-project))

### How to Handle Licensing

1. **Create a `LICENSE` file in the root directory.** GitHub automatically detects it and displays the license type in the repository sidebar.
2. **Don't repeat the license in the README.** It's pure noise — if someone wants to know the license, they know where to click. ([Elegant READMEs](https://www.yegor256.com/2019/04/23/elegant-readme.html))
3. **Exception: non-permissive licenses.** If you have a non-permissive license, stick it at the very top of the README to prevent confusion. Most developers assume MIT/BSD/Apache and may not check. ([Art of README](https://github.com/hackergrrl/art-of-readme#key-elements))
4. **Include a license badge** in your badge row — it's a quick visual signal.

### Common License Choices

| License | Use Case | Permissiveness |
|---------|----------|---------------|
| MIT | "Do what you want, just keep the notice" | Very permissive |
| Apache 2.0 | Like MIT but with patent grant | Permissive |
| BSD (2-Clause) | Minimal restrictions, like MIT | Very permissive |
| GPLv3 | Copyleft — modifications must also be open source | Restrictive |
| AGPLv3 | Like GPLv3 but covers network use too | Very restrictive |
| Unlicense | Public domain dedication | No restrictions |

**The practical rule:** Most open source projects use MIT. It's the path of least resistance for adoption. If you have strong feelings about copyleft, use GPL — but be aware it will limit your user base.

---

## Contributing Guide

### Why You Need One

A well-crafted contributing guide ensures contributors follow your project's best practices, raise useful issues, and submit well-formed pull requests. It makes it easier for maintainers to manage the project and reduces the friction that causes potential contributors to walk away.

> *"You don't want your potential contributor to walk away. That's why this part of the entire README is the most important one."*
> — Yegor Bugayenko ([Elegant READMEs](https://www.yegor256.com/2019/04/23/elegant-readme.html))

### What to Include

**In the README (brief):**
```markdown
## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for
detailed guidelines on setting up your development environment,
making changes, and submitting pull requests.
```

**In CONTRIBUTING.md (detailed):**

1. **Development environment setup**
   - Required tools and versions
   - How to install dependencies
   - How to build the project
   - How to run tests
   - How to run in hot-reload/dev mode

2. **Making changes**
   - Branch naming conventions
   - Commit message format (e.g., Conventional Commits)
   - Code style and linting
   - Testing requirements

3. **Submitting changes**
   - How to create a pull request
   - What to expect in review
   - CI/CD pipeline explanation

4. **Finding something to work on**
   - Link to "good first issue" label
   - Link to project roadmap
   - Areas that need help

### The Junior Developer Rule

Address your contributing guide to a junior programmer, not yourself. Don't assume knowledge of your stack:

> *"Imagine that you are talking to a junior developer who doesn't even know what Java and Maven are (if your project is using them). You should explain how to install the right tools, how to build the project, how to make changes, how to run it in a hot-reload mode, how to create the fork, and what to expect when the fork is submitted."*
> — Yegor Bugayenko ([Elegant READMEs](https://www.yegor256.com/2019/04/23/elegant-readme.html))

### Specific Commands, Not Greedy Ones

Use specific commands instead of greedy ones:
- ✅ `git add src/auth.js src/auth.test.js` — intentional, reviewable
- ❌ `git add .` — risks adding unwanted files

([Daytona, 2024](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project))

---

## Code of Conduct

A Code of Conduct defines how the project will engage and interact with the community, setting expectations for respectful contributions. It signals that the project is professionally managed and inclusive.

### Quick Setup

The [Contributor Covenant](https://www.contributor-covenant.org/) is the most widely-adopted code of conduct for open source projects. It's a drop-in solution:

1. Copy the [Contributor Covenant text](https://www.contributor-covenant.org/version/2/1/code_of_conduct/) into `CODE_OF_CONDUCT.md`
2. Add your contact email for reporting violations
3. Link to it from your README and CONTRIBUTING.md

```markdown
## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code.
```

([Daytona, 2024](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project))

---

## Tests & Examples

### Why Tests Matter for Adoption

A project without tests is harder to trust, harder to contribute to, and looks more like a hobbyist hack. More than that, tests cut down on support requests — users can look at the code to understand behavior.

> *"I don't have to ask you or Google, I can look at the code."*
> — Adam Stacoviak ([Changelog - Top Ten Reasons](https://changelog.com/posts/top-ten-reasons-why-i-wont-use-your-open-source-project))

### What to Signal in Your README

1. **CI badge showing tests pass** — the green checkmark is a powerful trust signal
2. **Coverage badge** — shows you care about quality
3. **How to run tests** — in the contributing section:
   ```bash
   # Run the test suite
   npm test

   # Run a specific test file
   npm test -- --grep "auth"
   ```
4. **Examples directory** — an `examples/` folder with scenario-based code shows rather than tells. Express.js does this well with its [examples folder](https://github.com/visionmedia/express/tree/master/examples/).

([Changelog - Top Ten Reasons](https://changelog.com/posts/top-ten-reasons-why-i-wont-use-your-open-source-project))

---

## Support Channels

Provide clear channels for users and contributors to seek assistance, report issues, or provide feedback. Responsive and accessible support mechanisms enhance the user experience and foster community and long-term engagement.

### What to Include

```markdown
## Getting Help

- **Documentation:** [docs.myproject.dev](https://docs.myproject.dev)
- **Questions:** [GitHub Discussions](https://github.com/user/repo/discussions)
- **Bug Reports:** [GitHub Issues](https://github.com/user/repo/issues)
- **Chat:** [Discord](https://discord.gg/invite) or [Slack](https://slack.myproject.dev)
- **Security Issues:** See [SECURITY.md](SECURITY.md) for responsible disclosure
```

### Channel Selection Guide

| Channel | Best For | When to Use |
|---------|----------|-------------|
| GitHub Discussions | Q&A, ideas, community | Public, searchable, async |
| GitHub Issues | Bug reports, feature requests | Tracked, linked to code |
| Discord/Slack | Real-time chat, community | Quick questions, social |
| Mailing List | Announcements, long-form | Formal, archival |
| Stack Overflow | Technical Q&A | SEO-friendly, reputation system |

([Daytona, 2024](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project))

---

## GitHub Repository Settings

Beyond the README, your repository's settings page communicates hygiene at a glance.

### About Section

Ensure all sections are completed thoroughly:
- **Description** — a concise one-liner (appears on Trending, Topics, and search results)
- **Website** — link to your project homepage or docs
- **Topics** — relevant tags from [GitHub's Featured Topics](https://github.com/topics)

Deselect **Releases**, **Packages**, and **Deployments** if they're not applicable — empty sections look abandoned.

### Releases

Use GitHub's Releases feature to enable users to gauge the project's activity level based on the most recent release date. Each release should include:
- Version number (semantic versioning)
- Release notes summarizing changes
- Binary artifacts for download (if applicable)

### Issues

**Custom Labels:** Create labels tailored to your project's needs:
- `good first issue` — beginner-friendly tasks to encourage new contributors
- `help wanted` — issues where community help is especially welcome
- `bug`, `enhancement`, `documentation` — standard triage categories
- `blocked`, `needs-triage`, `ready-for-dev` — workflow states

**Issue Templates:** Customize GitHub's issue templates to ensure reported issues follow a consistent structure. Include:
- Bug report template (steps to reproduce, expected vs actual behavior, environment)
- Feature request template (problem statement, proposed solution, alternatives considered)

([Daytona, 2024](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project))

---

## The Hygiene Checklist

- [ ] `LICENSE` file in root directory (GitHub auto-detects it)
- [ ] License badge in README badge row
- [ ] `CONTRIBUTING.md` with development setup, making changes, and submitting changes
- [ ] Contributing section in README linking to full guide
- [ ] `CODE_OF_CONDUCT.md` (Contributor Covenant or custom)
- [ ] CI badge showing passing tests
- [ ] Coverage badge (if applicable)
- [ ] How to run tests documented
- [ ] `examples/` directory with runnable scenario-based code
- [ ] Support channels documented (Discussions, Issues, Chat, Docs)
- [ ] `SECURITY.md` for vulnerability reporting
- [ ] Repository About section fully completed
- [ ] GitHub Topics selected from Featured Topics
- [ ] Releases used for version history
- [ ] Issue templates for bugs and feature requests
- [ ] Custom labels for issue triage
- [ ] `good first issue` label on beginner-friendly issues

---

*Previous: [Anatomy of a README](anatomy-of-a-readme.md)*
*Next: [Aesthetics & Visual Design](aesthetics-and-visual-design.md) — making your README beautiful*
