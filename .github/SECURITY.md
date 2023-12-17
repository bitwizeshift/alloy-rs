# Security Policy

## Reporting a Security Vulnerability

This project has a small surface-area for general vulnerability exploitation,
since it does not directly process any user data in any way, nor does it work
with encrypted

As such, if you discover a security vulnerability within this project, please
just open a [Github Issue] using the [Security Vulnerability] template.
I appreciate your responsible disclosure and will make every effort to quickly
address the issue.

When doing so, please provide the following details in your email:

* Your affiliation (if applicable).
* A detailed description of the vulnerability, including information on how to
  reproduce it.
* Any potential impact of the vulnerability.

Once we receive your report, we will acknowledge the receipt of the
vulnerability within **1 week**, and strive to provide regular updates on our
progress.

[Github Issue]: https://github.com/bitwizeshift/alloy-rs/issues/new
[Security Vulnerability]:

## Security Practices

### Dependency Vulnerability Scanning

We conduct weekly vulnerability scanning via [`cargo audit`] to identify and
address potential security issues from a CVE database.
This proactive approach helps us stay ahead of potential threats brought in
from outdated and compromised dependencies.

[`cargo audit`]: https://crates.io/crates/cargo-audit

### Static CodeQL Analysis

We use [`rust clippy`] to ensure the overall quality code integrity within the
codebase. This helps identify and address issues related to code structure,
maintainability, and security best practices.

This is connected to CodeQL analysis, which detects and address security
vulnerabilities in the codebase. CodeQL provides a powerful static analysis
tool that aids in finding security-related issues early in the development
process.

[`rust clippy`]: https://github.com/rust-lang/rust-clippy#clippy

### License Scanning

We use [`cargo deny`] to ensure that all 3rd-party dependencies use an
OSI-approved license that is compatible with this project, and [`cargo about`]
to generate a license manifest that ensures proper attribution is provided for
the authors of any direct or indirect 3rd-party projects.

**Note:** If you believe there may be an issue with attribution in **Alloy**,
please open a [Github Issue] and tell us what is missing! We make no effort to
misrepresent the origin of software, and are thankful for all projects which we
rely on -- and want to be responsible about attributing everyone accordingly.

[`cargo deny`]: https://github.com/EmbarkStudios/cargo-deny
[`cargo about`]: https://github.com/EmbarkStudios/cargo-about

## Updates and Notifications

Security updates and notifications will be provided through the project's
GitHub repository. Users are encouraged to watch the repository for any
announcements regarding security releases or patches.

## Supported Versions

We prioritize addressing security vulnerabilities in the latest stable release
of the project. Users are strongly encouraged to keep their installations
up-to-date with the latest releases.

## License

By participating in this responsible disclosure process, you agree that your
actions comply with applicable laws and regulations. We appreciate your
contribution to the security of this project.

Thank you for helping to keep this project secure!
