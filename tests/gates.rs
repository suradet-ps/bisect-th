use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const SAMPLE: &str = "# Title\n\n## Section\n\n```rust\nfn main() {}\n```\n\n```text\nhello\n```\n\n[ref]: https://example.com/a\n[local]: other.md#anchor\n\nInline [one](https://one.example) and [two](other.md#x) and [anchor](#some-anchor).\n";

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_bisect-th"))
}

fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn write(path: &Path, contents: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, contents).unwrap();
}

fn run(command: &mut Command) -> (bool, String) {
    let output: Output = command.output().unwrap();
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    (output.status.success(), text)
}

#[test]
fn verify_passes_on_identical_trees() {
    let root = scratch("verify-ok");
    let orig = root.join("orig");
    let trans = root.join("trans");
    write(&orig.join("a.md"), SAMPLE);
    write(&trans.join("a.md"), SAMPLE);

    let (passed, output) = run(binary()
        .args(["verify", "--orig"])
        .arg(&orig)
        .arg("--trans")
        .arg(&trans));
    assert!(passed, "{output}");
    assert!(output.contains("Checked 1 files, 0 problem(s)"), "{output}");
    assert!(
        output.contains("ALL OK: code blocks, headings, links match 100%"),
        "{output}"
    );
}

#[test]
fn verify_reports_every_kind_of_problem() {
    let root = scratch("verify-fail");
    let orig = root.join("orig");
    let trans = root.join("trans");
    write(&orig.join("a.md"), SAMPLE);
    write(&orig.join("b.md"), "# B\n");
    let mutated = SAMPLE
        .replace("fn main() {}", "fn main() { }")
        .replace("## Section", "### Section")
        .replace("https://example.com/a", "https://example.com/b")
        .replace("other.md#x", "other2.md#x");
    write(&trans.join("a.md"), &mutated);

    let (passed, output) = run(binary()
        .args(["verify", "--orig"])
        .arg(&orig)
        .arg("--trans")
        .arg(&trans));
    assert!(!passed, "{output}");
    for expected in [
        "[FAIL] a.md : code block #1 differs",
        "[FAIL] a.md : heading #2 level differs",
        "[FAIL] a.md : ref-link #1 url differs",
        "[FAIL] a.md : inline link targets differ (missing=[other.md] extra=[other2.md])",
        "[FAIL] b.md : missing translated file",
        "Checked 2 files, 5 problem(s)",
    ] {
        assert!(
            output.contains(expected),
            "missing {expected:?} in:\n{output}"
        );
    }
}

#[test]
fn check_links_reports_broken_anchors() {
    let root = scratch("links-fail");
    let book = root.join("book");
    write(
        &book.join("index.html"),
        "<h1 id=\"ok\">T</h1>\n\
         <a href=\"#ok\">ok</a>\n\
         <a href=\"#missing\">bad</a>\n\
         <a href=\"page.html#ok\">ok</a>\n\
         <a href=\"page.html#nope\">bad</a>\n\
         <a href=\"gone.html\">bad</a>\n\
         <a href=\"https://example.com/#x\">skip</a>\n",
    );
    write(&book.join("page.html"), "<h1 id=\"ok\">P</h1>\n");

    let (passed, output) = run(binary().args(["check-links", "--book"]).arg(&book));
    assert!(!passed, "{output}");
    for expected in [
        "Checked 4 anchor links",
        "[BROKEN] index.html -> #missing (id not found)",
        "[BROKEN] index.html -> page.html#nope (anchor not found in target)",
        "[BROKEN] index.html -> gone.html (file missing)",
    ] {
        assert!(
            output.contains(expected),
            "missing {expected:?} in:\n{output}"
        );
    }
}

#[test]
fn check_links_passes_when_all_anchors_resolve() {
    let root = scratch("links-ok");
    let book = root.join("book");
    write(
        &book.join("index.html"),
        "<h1 id=\"ok\">T</h1>\n\
         <a href=\"#ok\">ok</a>\n\
         <a href=\"page.html#ok\">ok</a>\n",
    );
    write(&book.join("page.html"), "<h1 id=\"ok\">P</h1>\n");

    let (passed, output) = run(binary().args(["check-links", "--book"]).arg(&book));
    assert!(passed, "{output}");
    assert!(output.contains("Checked 2 anchor links"), "{output}");
    assert!(output.contains("ALL ANCHOR LINKS OK"), "{output}");
}
