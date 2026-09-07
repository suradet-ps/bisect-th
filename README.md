# bisect-th

```
██████╗ ██╗███████╗███████╗ ██████╗████████╗        ████████╗██╗  ██╗
██╔══██╗██║██╔════╝██╔════╝██╔════╝╚══██╔══╝        ╚══██╔══╝██║  ██║
██████╔╝██║███████╗█████╗  ██║        ██║    █████╗    ██║   ███████║
██╔══██╗██║╚════██║██╔══╝  ██║        ██║    ╚════╝    ██║   ██╔══██║
██████╔╝██║███████║███████╗╚██████╗   ██║              ██║   ██║  ██║
╚═════╝ ╚═╝╚══════╝╚══════╝ ╚═════╝   ╚═╝              ╚═╝   ╚═╝  ╚═╝
```

---

## ◆ PULSE

A rustc regression has a first nightly, and a first PR - bisect-th is
the Thai bridge to that exact moment. This is the complete Thai
translation of the official `cargo-bisect-rustc` guide: 21 chapters
built with mdbook, terminology locked by a single glossary, and every
code block byte-identical to the original. The links are checked
against the built book (117 anchors), the structure mirrors the
upstream repo file-for-file, and the license travels with the text.
Built for the Thai-speaking student of rustc:
[github.com/suradet-ps/bisect-th](https://github.com/suradet-ps/bisect-th).

| แปลครบ 21 บท ▣ | Glossary ▣ | ลิงก์ 117/117 ▣ | Build ผ่าน ▣ |
|---|---|---|---|

*v1.0.0 - translation, glossary, verification, and the static build
are all sealed.*

> Built with mdbook 0.5 + Markdown, translated from
> [rust-lang/cargo-bisect-rustc](https://github.com/rust-lang/cargo-bisect-rustc),
> verified by script and rendered as static HTML - a guide with the
> pages on the page.
>
> **suradet-ps**, artifact keeper

---

## ◆ IGNITION

One runtime, three commands.

```
⟫ git clone https://github.com/suradet-ps/bisect-th.git
⟫ cd bisect-th
⟫ cargo install mdbook
⟫ mdbook serve guide --open
```

Open [http://localhost:3000](http://localhost:3000).

```
⟫ mdbook build guide          # static HTML into guide/book
⟫ powershell scripts/verify-translation.ps1   # byte-exact check vs upstream
⟫ powershell scripts/check-links.ps1          # all anchors in the built book
```

<details>
<summary>Translating a chapter</summary>

A chapter is a file: `guide/src/<chapter>.md`, listed in
`guide/src/SUMMARY.md`. The glossary lives in `GLOSSARY.md` - a term
is chosen once and reused everywhere. Code blocks, commands, links,
and filenames stay verbatim; only prose and headings are translated.
Heading anchors follow mdbook's slug rules (Thai tone marks are
stripped), so anchors are copied from the built HTML, never guessed.

</details>

---

## ◆ ANATOMY

One stack, zero JS, several quiet helpers.

- **Translates** - the complete guide: introduction, installation,
  usage, tutorial, boundaries, rustup, git bisect, alt builds, and 11
  example chapters - Thai prose over untouched code.
- **Glossaries** - `GLOSSARY.md` locks the vocabulary (regression =
  รีเกรสชัน, baseline = เบสไลน์, toolchain = ทูลเชน), so chapter nine
  agrees with chapter two.
- **Verifies** - `scripts/verify-translation.ps1` diffs every code
  block, heading level, and link URL against the upstream repo -
  byte-exact or it does not pass.
- **Checks** - `scripts/check-links.ps1` walks the built book and
  resolves every anchor link against real heading ids - 117 of them,
  all reachable.
- **Builds** - mdbook renders static HTML into `guide/book/`, no
  JavaScript, no runtime, readable offline and searchable by your
  editor of choice.
- **Licenses** - MIT OR Apache-2.0, inherited from upstream, with the
  LICENSE files shipped beside the text.

---

## ◆ RITUALS

**The core ceremony** - the translation pass:

1. Open a chapter in `guide/src/`. The upstream file sits beside it
   in the original repo - structure is a contract.
2. Translate the prose; keep every code block and command as the
   original wrote it.
3. Consult `GLOSSARY.md` for every term that already has a canon.
   New terms get proposed in the glossary first.
4. Build, verify, check. The book builds clean, the diff is
   byte-exact, and the anchors resolve.

**The ceremony of the anchor** - mdbook slugs strip Thai tone marks
(`การเขียนสคริปต์` becomes `การเขียนสคริปต`). Anchors are read from
the built HTML, written into the source, and re-verified - a guessed
anchor is a broken link waiting to happen.

**The ceremony of the code block** - a translated command that is not
byte-identical to the original is a regression, not a translation.
The verifier is the conscience of the repo.

---

## ◆ ECHOES

**Where this artifact is heading**

```
P1 ▸ SUMMARY + introduction, installation, usage ──────────────────── ▸ sealed
P2 ▸ tutorial, boundaries, rust-src-repo, rustup ──────────────────── ▸ sealed
P3 ▸ git-bisect, alt builds, all 11 example chapters ──────────────── ▸ sealed
P4 ▸ glossary, license, link verification, mdbook build ───────────── ▸ sealed
```

**Raising the artifact** - the honest path lives in `GLOSSARY.md`
(term canon), `scripts/` (the verification gate), and `guide/book.toml`
(book config). New chapters follow the frontmatter-free contract of
the SUMMARY. Open an issue first to discuss a change.

**Status** - on every change: `mdbook build guide` must pass, the
translation verifier must report byte-exact code blocks across all 22
files, and the link checker must report `ALL ANCHOR LINKS OK`.
[Watch the gates](scripts).

---

```
  ─────────────────────────────────────────
   ทุกรีเกรสชันมี nightly แรกของมัน
   ทุกหนังสือมีหน้าแรกของมัน
  ─────────────────────────────────────────
```

Translated from the [cargo-bisect-rustc](https://github.com/rust-lang/cargo-bisect-rustc)
guide, which is licensed under the [MIT OR Apache-2.0](LICENSE-MIT).