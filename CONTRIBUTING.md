# Contributing to Villagefolk

Thanks for helping improve Villagefolk. The project focuses on giving Sunrise villagers persistent, configurable names and making those names easy for server owners to manage.

## Before you start

- Search open and closed issues for an existing report or proposal.
- For a larger behavior change, open an issue first so we can agree on the scope.
- Keep changes focused on villager names, their configuration, or how players see and manage them.
- Preserve stable names across saves, world reloads, and name-tag renames unless a change explicitly discusses that behavior.
- Villagefolk is server-side. New features should keep multiplayer clients free of a required mod install where possible.

## Report a bug or suggest a feature

Use the Bug report or Feature request forms in GitHub Issues. Include the Sunrise build and Villagefolk version, whether the issue happens in single-player or on a dedicated server, and clear reproduction steps. For configuration issues, include a minimal example with any private information removed.

For bugs, attach relevant logs or a screenshot when they help explain the problem. Please avoid attaching entire worlds or unrelated personal files.

## Development setup

Install Rust and Sunrise's `cargo sunrise-mod` tool. The project pins the Sunrise build and native SDK in `sunrise-mod.toml`; use that pinned toolchain when building or checking changes.

From the repository root:

```sh
cargo sunrise-mod check
cargo sunrise-mod build
cargo test
```

See [README.md](README.md) for the mod's configuration and usage, and [SPECS.md](SPECS.md) for its current scope and behavior. If a change affects how the mod behaves in game, describe the manual steps needed to verify it in your pull request.

## Branch workflow

- `main` is the release branch. Do not develop directly on it.
- `dev` is the shared integration branch for accepted work.
- Create short-lived topic branches from the latest `dev` and open pull requests back to `dev`.
- Use branch names such as `feature/profession-name-pools`, `fix/config-reload`, `docs/server-setup`, or `chore/ci-cleanup`.
- Keep pull requests focused, explain the user-visible change, and update the README or specification when behavior or setup changes.
- Do not force-push shared branches. Rebase or merge the latest `dev` into your own topic branch as needed.
- Promote reviewed work from `dev` to `main` with a pull request when it is ready for release.

Releases are made from a clean `main` checkout using Sunrise's release tool, for example `cargo sunrise-mod release patch`. The tool updates the version, creates a release commit and tag, and pushes them to GitHub, which starts the signed release workflow. It is not an ordinary development command; confirm the intended version and release contents before running it. Do not edit release versions or generated signing assets by hand.

## Commit messages

Use a short Conventional Commit style subject:

```text
<type>(optional-scope): <imperative summary>
```

Common types are `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `build`, and `ci`. Examples:

```text
feat(config): add profession-specific name lists
fix(names): preserve names after config reload
docs: explain per-world server settings
```

Keep each commit to one logical change, write the subject in English, and omit the trailing period. Do not include generated files unless the Sunrise tool or the change specifically requires them. Release commits and their sign-off are created by the release tool.

## Licensing

Code, configuration data, and documentation in this repository are under the [PolyForm Noncommercial License 1.0.0](LICENSE). Only contribute material you have the right to submit under that license. Do not add artwork, screenshots, or other third-party material unless you own it or have permission to redistribute it.
