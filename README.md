<!-- template:start -->
> [!NOTE]
> **This repository is a template.** Create a repository from it, open the new repository in Claude Code and run
> `/project-setup`. It asks one question, the project's name and what it is for, fills in what is known and removes
> this note. The other placeholders, such as the stack, are filled in later, as the work makes them known. With
> another AI agent, ask it to follow `.claude/skills/project-setup/SKILL.md`; by hand, replace the project's name,
> purpose and owner where you find `{{`, and delete this note.
>
> What comes with it, so that no agent needs the same things explained twice:
>
> - `AGENTS.md` and `CLAUDE.md`: what an agent reads first, what it must never do, and when a change is done.
> - `documentation/`: rules, requirements, tests, a maintainer's guide, terms, decisions and a development log in
>   parts of at most 1000 lines.
> - `.claude/skills/`: procedures for the log, `TESTS.md`, `MAINTAINER.md`, `TERMINOLOGY.md` and a check of the rules,
>   and `ub-mode`, principles for writing code, whose tools crapper, dryer and mutator stay outside the project.
> - `.claude/settings.json`: Claude Code may not commit, push or change `REQUIREMENTS.md`.
> - `.github/`: contributing, conduct, security and accessibility notes, issue and pull request forms, and a
>   workflow that runs `.github/scripts/check-rules.sh` and the tests on every push.
<!-- template:end -->

[![GitHub last commit](https://img.shields.io/github/last-commit/{{GITHUB_OWNER}}/{{REPO_NAME}})](#)
[![language](https://img.shields.io/badge/language-{{LANGUAGE}}-FFFF00)](#)
[![license](https://img.shields.io/badge/license-MIT-007FFF)](LICENSE)
[![testing](https://img.shields.io/badge/testing-yes-C3FF00)](documentation/TESTS.md)
[![maintainable](https://img.shields.io/badge/maintainable-yes-00FF72)](documentation/MAINTAINER.md)

# {{PROJECT_NAME}}

## Description

{{PROJECT_DESCRIPTION}}

## Usage

{{USAGE}}

## Contributing

Anyone is welcome to open an [issue](https://github.com/{{GITHUB_OWNER}}/{{REPO_NAME}}/issues) to submit a bug report or suggest a new feature. Alternatively, you may fork or copy the repository and take it in any direction you wish.

If you wish to contribute work into this repository, please consult [CONTRIBUTING.md](.github/CONTRIBUTING.md) before any commits or pull requests.

## Maintaining

To keep {{PROJECT_NAME}} working and up to date, see [MAINTAINER.md](documentation/MAINTAINER.md).

## Acknowledgements

Development of {{PROJECT_NAME}} was accelerated with {{AI_MODEL}}.
