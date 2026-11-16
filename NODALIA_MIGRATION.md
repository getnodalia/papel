# Nodalia migration: Papel

Category: secondary app.
Status: renamed to Papel throughout, as of 2026-11-15. Evidence: the pull request that adds `scripts/rebrand.py`.

## Current state

Pure-Rust PDF editor (egui), MIT OR Apache-2.0. Cargo git dependencies come from getnodalia/foundation and nodalia-design. The gate, FreeBSD,
Nix, packaging lint, AGENTS.md and git-sources checks call getnodalia/foundation's reusable
workflows pinned by commit. SonarCloud was removed; CodeQL and Scorecard remain.

## Desired state

No workflow, dependency or document depends on storytold; Papel has its own release workflow; the
interface follows Foundation's [NDL standard](https://github.com/getnodalia/foundation/blob/main/standards/ndl.md).

## Inherited dependencies still pending

| Dependency | Where | Replacement | Blocked on |
|---|---|---|---|
| `storytold/craft-common-actions` | `.github/workflows/windows-arm64.yml` | Foundation reusable workflows (release, refresh-contributors, windows-arm64) | writing and testing them |
| `storytold/pdfcraft` | `.github/workflows/upstream-sync.yml` | none: the intended weekly upstream sync, renamed by `scripts/rebrand.py` | — |
| `storytold/winprint.rs` | `Cargo.toml` `[patch]` | getnodalia/foundation's vendor branches already serve it; the `[patch]` names the source it overrides | — |

## Technical debt

- Every name is Papel's, internal ones included (crates `papel-*`, binaries `papel` and
  `papel-cli`, app id `dev.danielmiguelt.Papel`, `PAPEL_*` environment variables). Upstream's tree
  arrives renamed by `scripts/rebrand.py` (`docs/upstream.md`). The shared translation catalog keeps
  its name, `pdfcraft.tsv`.

## Reusable Nodalia components

getnodalia/foundation (vendored crates, fonts, translations, reusable workflows);
getnodalia/nodalia-design (theme, icons).

## NDL work pending

Check the interface against Foundation's NDL standard: hamburger menu with About, Preferences and
Keyboard shortcuts; theme selector only in Preferences; NDL window controls on Linux only; global
menu exported over D-Bus on Linux.

## Risks and priorities

1. Releases still run the inherited release workflow.
2. Windows ARM64 packaging still runs the inherited workflow.

## Recommended order

1. A Foundation release workflow, tested with a dry run, then Papel's release on it.
2. A Foundation Windows ARM64 workflow.
3. NDL review of the interface.

## What the first agent should do

1. Read AGENTS.md and this file.
2. Pick the first item of "Recommended order".

## Releases

Papel releases through `release-auto.yml` like every Nodalia app (Foundation's `release-tag` and
`release-publish`, with `release-linux.yml`, `release-macos.yml` and `release-windows.yml`). The
packages are named `papel-X.Y.Z-…`.
