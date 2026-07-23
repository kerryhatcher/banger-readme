# Anti-Patterns: What Repels Users

> The mistakes that drive away potential users and contributors — and how to avoid them.

---

## The Classic "Top Ten Reasons I Won't Use Your Open Source Project"

Adam Stacoviak's 2011 Changelog article remains one of the most-cited pieces on open source project failures. Despite being over a decade old, every point is still relevant:

([Changelog - Top Ten Reasons](https://changelog.com/posts/top-ten-reasons-why-i-wont-use-your-open-source-project))

### 1. You Don't Have a Friggin' README

The most fundamental failure. For many projects, especially those hosted on GitHub, the README is the first look someone has at your project. No README = no context = no adoption.

**The fix:** At minimum, include a description, installation instructions, where to get help, contribution guidelines, and a license.

### 2. You Don't Include Tests, Specs, Features, Examples

A project without tests is harder to trust, harder to contribute to, and looks like a hobbyist hack. Tests also cut down on support requests — users can look at the code to understand behavior.

**The fix:** Include a test suite with a CI badge showing it passes. Add an `examples/` directory with scenario-based code.

### 3. You Have No Project Home Page

GitHub project pages are great but suck for SEO and branding. A dedicated home page gives your project an identity beyond its repository.

**The fix:** Use [GitHub Pages](https://pages.github.com/) — it's free, supports custom domains, and is dead simple to set up.

### 4. You Need Design Help

Poor visual presentation undermines credibility. If you're not blessed with design skills, find someone who is.

**The fix:** Pay, trade, or barter with a designer for a logo and basic brand. Avoid spec work and contest sites.

### 5. You Don't Have a Domain Name

A nice home page is great, but a domain name makes it easier for people to find and remember your project.

**The fix:** Shell out $10–15/year for a domain. Use [Domai.nr](https://domai.nr/) to find creative alternatives if the `.com` is taken.

### 6. You Don't Have a Twitter Account (or Social Presence)

A dedicated project account helps you share updates, answer questions, and build community.

**The fix:** Create a project account on the platform where your users are. For developer tools, that's often Twitter/X, Mastodon, or Discord.

### 7. Your Licensing Is Unclear

Just because code is published online doesn't mean it's free for the taking. Users need to know the terms.

**The fix:** Include a `LICENSE` file in the root directory. GitHub auto-detects it.

### 8. You Don't Reach Out to Me

If you don't engage with the community, the community won't engage with your project.

**The fix:** Search for people asking questions your project solves. Answer on Stack Overflow, Reddit, and forums. Create an IRC/Discord channel for your project.

### 9. You Don't Speak About Your Project at Conferences and Meetups

Nothing builds credibility like a conference talk. It positions you as an expert and puts your project in front of an engaged audience.

**The fix:** Submit talk proposals to conferences and meetups in your technology community. If not accepted, organize a birds-of-a-feather session.

### 10. You Didn't Submit It to Discovery Channels

If you don't tell anyone about your project, nobody will know it exists.

**The fix:** Submit to newsletters, podcasts, and discovery platforms in your ecosystem. For JavaScript: JavaScript Weekly, Node Weekly. For Python: PyCoder's Weekly. For general: Hacker News, Reddit, Product Hunt.

---

## Common README Mistakes

Beyond the top ten, these are the specific README anti-patterns that experienced developers cite:

### Empty Sections

Leaving "Coming soon" or "TODO" placeholders in your README signals that the project is unfinished or abandoned. Remove empty sections entirely — create issues with appropriate labels for sections that need content.

([Daytona, 2024](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project))

### Broken Links

Broken links erode trust immediately. They suggest the project isn't maintained. Check links regularly — ideally with an automated link checker in CI.

### Outdated Information

An outdated README causes confusion and erodes trust. If your README says "requires Node 14" but the project now requires Node 20, you're actively misleading users.

([Utrecht University](https://utrechtuniversity.github.io/workshop-computational-reproducibility/chapters/readme-files.html))

### Repeating the Repo Name as a Title

The title is already in the URL. The description is in the GitHub subtitle. Starting your README with `# MyProject` is redundant and wastes the most valuable real estate in the document. Start with a logo instead.

([Elegant READMEs](https://www.yegor256.com/2019/04/23/elegant-readme.html))

### Duplicating GitHub Features in the README

Several things GitHub already handles — don't repeat them:

- **License** — GitHub detects `LICENSE.txt` automatically. Don't repeat it in the README. ([Elegant READMEs](https://www.yegor256.com/2019/04/23/elegant-readme.html))
- **Contributor list** — GitHub has a Contributors tab. Only add an "Acknowledgements" section for key contributors with their blog/Twitter URLs. ([Elegant READMEs](https://www.yegor256.com/2019/04/23/elegant-readme.html))
- **Changelog** — use GitHub's Releases tab instead of maintaining a separate changelog in the README. ([Elegant READMEs](https://www.yegor256.com/2019/04/23/elegant-readme.html))
- **Downloads** — GitHub has a Releases tab for that. Don't list download links in the README. ([Elegant READMEs](https://www.yegor256.com/2019/04/23/elegant-readme.html))

### Badge Abuse

Too many badges create visual noise. Each badge should answer: "What real value does this provide to the typical viewer of this README?" A CI badge showing build status might better serve its audience by emailing maintainers or automatically creating an issue.

([Art of README](https://github.com/hackergrrl/art-of-readme#bonus-other-good-practices))

**Example of badge abuse:**
```markdown
[![Build](...)](...) [![Coverage](...)](...) [![Quality](...)](...)
[![Version](...)](...) [![Downloads](...)](...) [![License](...)](...)
[![Twitter](...)](...) [![Discord](...)](...) [![Stars](...)](...)
[![Forks](...)](...) [![Issues](...)](...) [![PRs](...)](...)
[![Size](...)](...) [![Deps](...)](...) [![Vulns](...)](...)
```
This is visual noise. Nobody needs 15 badges. Pick the 5–8 most important ones.

### Relying on Externally Hosted Images for Critical Information

Your README will outlive your repository host and any of the things you hyperlink to — especially images. If your architecture diagram is hosted on a CDN that goes away, or your screenshot is on a service that shuts down, your README becomes broken. Inline anything essential.

([Art of README](https://github.com/hackergrrl/art-of-readme#bonus-other-good-practices))

### Writing for Yourself Instead of a Newcomer

The most common and damaging anti-pattern. Your README should be addressed to a junior developer who doesn't know your stack, your conventions, or your assumptions.

> *"Don't tell me 'first, you have to learn Docker.' No, I don't. If I did, I'd have done it myself already. Tell me how to use it in this specific case, and spare me all the rest."*
> — Yegor Bugayenko ([Elegant READMEs](https://www.yegor256.com/2019/04/23/elegant-readme.html))

### The "Wall of Text" README

A README that is one long, unbroken block of text with no headers, no code examples, no images, and no formatting. This is the second most common failure mode after "no README at all."

**The fix:** Break content into sections with clear headers. Use code blocks, images, and bullet points. Make it scannable.

### The "Kitchen Sink" README

The opposite problem: a README that tries to be everything — full API docs, complete changelog, detailed architecture, contributor list, FAQ, troubleshooting guide, and philosophical manifesto all in one file.

**The fix:** The README is an introduction, not the complete documentation. Move detailed content to separate files and link to them.

### Inconsistent Naming

Using different names for the project in different places (repo name, README title, package name, CLI command) confuses users and hurts discoverability.

**The fix:** Pick one name and use it consistently everywhere.

([Daytona, 2024](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project))

---

## The "Smell Test" — Red Flags That Make Developers Bounce

Experienced developers develop heuristics for quickly evaluating projects. These are the red flags that cause them to close the tab:

1. **No README at all** — instant bounce
2. **README is just a title and one sentence** — suggests the project is a toy
3. **No installation instructions** — if I can't figure out how to install it in 30 seconds, I'm gone
4. **No code examples** — I need to see what using this looks like
5. **Last commit was 2+ years ago** — abandoned project
6. **No license** — legal risk, can't use it
7. **No tests** — can't trust it, can't contribute safely
8. **README has "Coming soon" sections** — unfinished, unmaintained
9. **Broken images or links** — neglected
10. **README is a novel with no headers** — disrespects my time

---

## How to Audit Your Own README

Go through your README with fresh eyes (or ask someone unfamiliar with the project) and answer these questions:

1. **Can I understand what this does in 10 seconds?** (one-liner + demo)
2. **Can I install and run it in 60 seconds?** (quick start)
3. **Do I know if it's actively maintained?** (badges, recent commits)
4. **Do I know how to get help?** (support channels)
5. **Do I know if I can use it legally?** (license)
6. **Do I know how to contribute?** (contributing guide)
7. **Are there any broken links or images?** (link check)
8. **Are there any empty or "Coming soon" sections?** (remove them)
9. **Is the information current?** (version numbers, requirements)
10. **Would my grandma understand the installation instructions?** (junior developer test)

---

*Previous: [Emerging Patterns & Trends](emerging-patterns.md)*
*Next: [Exemplary READMEs](exemplary-readmes.md) — the best READMEs in the wild, analyzed*
