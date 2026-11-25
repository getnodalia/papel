<div align="center">
  <a href="https://getnodalia.com">
    <img src="https://raw.githubusercontent.com/danielmigueltejedor/nodalia-web/main/assets/brand/nodalia-wordmark-badge.svg" width="150" alt="Nodalia">
  </a>
  <br><br>
  <img src="https://raw.githubusercontent.com/danielmigueltejedor/papel/main/assets/app-icon/hicolor/512x512/apps/dev.danielmiguelt.Papel.png" width="192" alt="Papel">
  <h1>Papel</h1>
  <p><strong>The PDF document editor of the Nodalia suite.</strong></p>

  <p>
    <img src="https://img.shields.io/badge/status-early%20stage-orange" alt="Early stage">
    <img src="https://img.shields.io/badge/Rust-native-dea584?logo=rust&logoColor=black" alt="Native Rust">
    <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2ea44f" alt="MIT OR Apache-2.0">
  </p>

  <p>
    <a href="#overview">Overview</a> ·
    <a href="#build-and-run">Build</a> ·
    <a href="#project-status">Status</a> ·
    <a href="#the-nodalia-suite">Suite</a> ·
    <a href="#license">License</a>
  </p>
</div>

---

## Overview

**Papel** is the PDF document editor of [Nodalia](https://getnodalia.com): a native app written in Rust,
with no account and no cloud. It runs on Linux, macOS and Windows.

<p align="center">
  <img src="https://raw.githubusercontent.com/getnodalia/nodalia-web/main/assets/screens/papel/papel-dark.jpg" width="900" alt="A document open in Papel in the dark theme, with the all-tools panel and the bookmarks">
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/getnodalia/nodalia-web/main/assets/screens/papel/papel-light.jpg" width="440" alt="The same document in the light theme">
  <img src="https://raw.githubusercontent.com/getnodalia/nodalia-web/main/assets/screens/papel/papel-home-light.jpg" width="440" alt="Papel's start screen, with the recommended tools and the recent files">
</p>

> [!IMPORTANT]
> **Papel is at an early stage.** Version 0.2.2 is out for Linux, macOS and Windows:
> [download it from the releases page](https://github.com/getnodalia/papel/releases/latest).

## What it does

- Reads, annotates, fills, signs, edits and organises PDF documents, offline and with no account.
- A template gallery on Home with thirteen one-page templates (invoice, quote, receipt, letter,
  meeting minutes, forms, timesheet, résumé, planner, certificate and more). Each is a normal PDF
  with real form fields, in English or Spanish and on Letter or A4 paper.
- Every action is a command, so it can be driven from the command line or by agents through MCP.

## Build and run

You need a recent stable Rust toolchain.

```bash
git clone https://github.com/getnodalia/papel.git
cd papel
cargo run --release -p papel
```

## Project status

| Area | State |
|---|---|
| Package and binary names | Done: the desktop binary is called `papel` |
| App icon | Done: portrait of Aldus Manutius |
| Nodalia interface | Done: rounded islands, Papel's accent and the suite's window controls on Linux |
| Shared interface resources | Done: the interface comes from the shared Nodalia Design Language |
| Installable packages | AppImage, .deb, .rpm and .tar.gz; a DMG; a Windows installer and portable zip |

## App icon

Nodalia app icons are black-and-white portraits of historical figures over the app colour.
Papel's is Aldus Manutius, who created the pocket book, the first portable document. Provenance and licence are in [`assets/app-icon/`](assets/app-icon/).

## The Nodalia suite

Papel is one of the applications of [Nodalia](https://getnodalia.com), an independent software
project developed in public by [Daniel Miguel Tejedor](https://danielmigueltejedor.com).

## License

`MIT OR Apache-2.0`. See [`LICENSE-MIT`](LICENSE-MIT), [`LICENSE-APACHE`](LICENSE-APACHE) and
[`NOTICE`](NOTICE).
