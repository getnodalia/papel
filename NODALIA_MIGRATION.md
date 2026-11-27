# Nodalia migration: Papel

Category: secondary app.
Status: renamed to Papel throughout; Flatpak and the weekly upstream sync removed, as of 2026-11-26.

## Current state

Pure-Rust PDF editor (egui), MIT OR Apache-2.0. Cargo git dependencies come from getnodalia/foundation and nodalia-design. The gate, FreeBSD,
Nix, packaging lint, AGENTS.md and git-sources checks call getnodalia/foundation's reusable
workflows pinned by commit. SonarCloud was removed; CodeQL and Scorecard remain.

## Desired state

No workflow, dependency or document depends on a repository outside Nodalia; Papel has its own release workflow; the
interface follows Foundation's [NDL standard](https://github.com/getnodalia/foundation/blob/main/standards/ndl.md).

## Inherited dependencies still pending

None.


## Technical debt

- Every name is Papel's, internal ones included (crates `papel-*`, binaries `papel` and
  `papel-cli`, app id `dev.danielmiguelt.Papel`, `PAPEL_*` environment variables). There is no
  upstream sync and no rename tooling.

## Reusable Nodalia components

getnodalia/foundation (vendored crates, fonts, translations, reusable workflows);
getnodalia/nodalia-design (theme, icons).

## NDL work pending

Check the interface against Foundation's NDL standard: hamburger menu with About, Preferences and
Keyboard shortcuts; theme selector only in Preferences; NDL window controls on Linux only; global
menu exported over D-Bus on Linux.

## Risks and priorities

1. None in CI: releases run Papel's own `release-*.yml` through Foundation's `release-tag` and
   `release-publish`, and Windows ARM64 runs Papel's own `windows-arm64.yml`.

## Recommended order

1. NDL review of the interface.

## What the first agent should do

1. Read AGENTS.md and this file.
2. Pick the first item of "Recommended order".

## Releases

Papel releases through `release-auto.yml` like every Nodalia app (Foundation's `release-tag` and
`release-publish`, with `release-linux.yml`, `release-macos.yml` and `release-windows.yml`). The
packages are named `papel-X.Y.Z-…`.
