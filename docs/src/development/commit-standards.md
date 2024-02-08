# Commit Standards

The **Alloy** project has some some requirements on how git commit
messages are formed.

## General

* Commits should be small, granular, and easy to follow and revert. Ideally,
  the same SOC practices that would be applied to software development should
  be applied to commits; each commit identifies a separtion of concerns.

## Commit Titles

* must be in imperitive, present-tense

* must be no longer than 50 characters

* must be prefixed by an [emoji key](#emoji-key) to indicate what the change
  is, followed by a space

  * If a commit may use more than one emoji to identify the change, it might
    indicate that the change is _not granular enough_, and you may be a
    candidate to be broken down into smaller commits for easier consumption

## Commit Messages

* Must not exceed 72-character wide

  * Exceptions are made if a message contains a link or other figure that
    exceeds the 72 character rule by nature

* Must indicate the rationale for the change, what was changed, and why

  * In general, more details are always better to help identify the cause of
    changes in a repository

## Emoji Key

Emojis are used to prefix commit titles in order to simplify categorization
of git log messages.

Use the table below to identify which prefixes should be used for the
respective change:

| Emoji | Reason                                                              |
|---|-------------------------------------------------------------------------|
| 🔖 | Version Tag                                                            |
| 📖 | Documentation / Textual Changes                                        |
| 📇 | Metadata (README, LICENSE, repo docs, etc)                             |
| 🚦 | Continuous Integration                                                 |
| ✨ | New Feature                                                             |
| ✏ | Rename                                                                  |
| 🔨 | Refactor                                                               |
| ⚠️ | Deprecation                                                             |
| 🗑️ | Removal                                                               |
| 🎨 | Cosmetic                                                               |
| 🩹 | Bug fix                                                                |
| 🧹 | Code Cleanup (includes moving types/files around)                      |
| ⏱ | Tuning / Performance                                                    |
| 🎯 | Testing (unit, benchmark, integration, etc)                            |
| 🔧 | Tooling                                                                |
| 🔐 | Security                                                               |
| ♿ | Accessibility                                                           |
| 🌐 | Localization / Internationalization                                    |
| 🚧 | WIP                                                                    |

**Note:** This list may be incomplete, and not cover all possible areas that
would be needed. Please feel free to start a discussion if new tags would be
more appropriate. Similarly, if there are more appropriate emojis to use as
tags, feel free to provide suggestions!

## Identifying Release-impacting Changes

Rather than misusing git through the likes of _"[conventional commits]"_ or some
other equivalent, **Alloy** uses [git trailers] to identify changes that
impact different change categories. Specifically, commits may contain the
following trailer(s) which will influence release generation:

* `Change-Category:` -- Indicates the type of change being made. May be one of
  `feature`, `fix`/`bugfix`. Optionally supports the word `breaking` (*) to
  indicate a deviation from currently promised behavior.
* `System:` -- Identifies the affected system (optional). When specified, is
  free-form -- but messages will be grouped together in the release notes to
  identify this.

When related to a ticket, trailers may also contain `Fixes:`, `Closes:`, or any
of the [GitHub trailers].

[GitHub trailers]: https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/using-keywords-in-issues-and-pull-requests#linking-a-pull-request-to-an-issue
[git trailers]: https://git-scm.com/docs/git-interpret-trailers
[conventional commits]: https://www.conventionalcommits.org/en/v1.0.0/

### What constitutes a breaking change

**Note:** This project is far from being ready for a "breaking" change as it
is pre-1.0, so this section is forward-thinking.

A "breaking change" for **Alloy** is defined as something that, either knowingly
or unknowingly, changes software behavior from the perspective of the
_game engine_ itself. This dstinction is important, as it means that a change
in behavior is not enough to constitute a "breaking" change, if the change is
aligning behavior with what the expectation should be.

For example, if **Alloy** declares the behavior of a function to do a specific
thing, and it does not properly achieve the documented behavior -- then fixing
this behavior will only be considered a _bug fix_ and not a breaking change.
Users who rely on the consequence of a bug will suffer breakages, but that is
by design.

If, on the other hand, the change were to change documented behavior -- this
will fall into the category of a "breaking change", since it deviates from
existing behavior.
