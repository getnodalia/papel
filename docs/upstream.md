# Papel upstream

| Field | Value |
|---|---|
| Upstream | `storytold/pdfcraft` (renamed from printcraft) |
| Last observed | `a11ea7ad8570871bac9e57cff62b0348a0b077f7` |
| Last evaluated | `fe70ecce73bd982e80887a4d70fdc3f3913e21e4` |
| Last integrated | `fe70ecce73bd982e80887a4d70fdc3f3913e21e4` merged on 2026-11-10 |
| Skipped | the community button and the brand files (Papel has neither) |
| Adapted | every name (`scripts/rebrand.py`), the app icon, the Nodalia interface |
| Conflicting | `README.md` (kept ours), `AGENTS.md` (upstream plus our quality contract) |

## How the sync works

Upstream's code arrives renamed: `scripts/rebrand.py` applies `.github/rebrand.toml` (PdfCraft
becomes Papel, internal names included; links to the shared libraries point at
getnodalia/foundation) to upstream's tree, and the weekly sync
(`.github/workflows/upstream-sync.yml`) records the result on the `upstream-papel` branch. Each
commit there has the previous mirror commit and the upstream commit it renames as parents, and
names that commit in an `Upstream-Commit:` trailer. The sync merges the newest mirror commit
into a branch and opens a pull request; files listed in `.github/upstream-keep` keep this
repository's version.

The mirror starts at the renamed `fe70ecce`, merged into `main` (with this repository's tree)
by the pull request that renamed the app. Before the first sync creates the `upstream-papel`
branch, the workflow finds that commit through its trailer. `python3 scripts/rebrand.py check`
fails when an old name comes back; a line that must keep one says `rebrand: keep`.

What stays named after upstream on purpose: the shared translation catalog `pdfcraft.tsv` (its
name in getnodalia/foundation's `vendor/craft-i18n`), the settings and recovery folders of the
app's earlier name PrintCraft (moved to Papel's on first launch), and the test certificates made
under that name. Papel's version numbers are its own: the `[[pin]]` rules keep them when
upstream's manifest arrives.
