# Security Policy

## Prevention

- [Dependabot](https://docs.github.com/en/code-security/dependabot/dependabot-security-updates/about-dependabot-security-updates) helps us keep our dependencies up-to-date to patch vulnerabilities as soon as possible by creating awareness and automated PRs.
- [Whitesource Bolt for GitHub](https://www.whitesourcesoftware.com/free-developer-tools/bolt/) helps us with identifying vulnerabilities in our dependencies to raise awareness.
- [GitHub's security features](https://github.com/features/security) are constantly monitoring our repo and dependencies:
  - All pull requests (PRs) are using CodeQL to scan our source code for vulnerabilities
  - Dependabot will automatically identify vulnerabilities based on the GitHub Advisory Database and open PRs with patches
- The [Scorecard GitHub Action](https://github.com/ossf/scorecard-action) automates the process by running security checks on the GitHub repository.

## Supported Versions

Security fixes are only applied to the latest commit on the default branch.
As there is no stable release yet, users are encouraged to always update to the latest version.

| Version              | Supported        |
| -------------------- | ---------------- |
| Latest (main branch) | ✅               |
| Tagged pre-releases  | ⚠️ Best effort   |
| Older commits        | ❌ Not supported |

## Reporting a Vulnerability

If you discover a potential security issue in **g1c**, please **do not open a public issue**.
Instead, report it privately by emailing:

📧 **[nicolas.lamirault@gmail.com](mailto:nicolas.lamirault@gmail.com?subject=g1c%3A%20Potential%20Security%20Issue)**

When reporting, please include:

- A clear description of the vulnerability.
- Steps to reproduce (if applicable).
- Affected platform and installation method (e.g., Docker, Homebrew, etc.).
- The commit hash or version tag you tested on.

## Response Process

This project is maintained in personal free time, so there are **no guaranteed response times**.
However:

- Reports will be acknowledged when possible.
- The issue will be investigated as soon as time allows.
- If confirmed, a fix will be developed and merged into the main branch.

Critical or high-impact issues may be prioritized depending on severity.

---

## Disclosure Policy

Please avoid publicly disclosing any details of a security vulnerability until:

- A fix has been published, **or**
- You have received explicit permission to disclose.

Your responsible disclosure helps maintain project integrity and protects other users.

---

_Thank you for taking the time to report security issues responsibly and helping improve g1c._
