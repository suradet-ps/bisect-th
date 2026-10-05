use crate::util;
use regex::Regex;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

static CODE_BLOCK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)```.*?```").unwrap());
static HEADING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^#{1,6} .*$").unwrap());
static REF_LINK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\[[^\]]+\]:\s+\S+.*$").unwrap());
static INLINE_LINK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[[^\]]*\]\(([^)]+)\)").unwrap());
static REF_SPLIT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r":\s*").unwrap());
static FILE_ANCHOR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([^#]+)#(.+)$").unwrap());

const ORIG_ERROR: &str = "Cannot find upstream cargo-bisect-rustc guide/src directory.\nPlease provide the path using: bisect-th verify --orig <path-to-cargo-bisect-rustc/guide/src>";

pub struct Options {
    pub orig: Option<PathBuf>,
    pub trans: PathBuf,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            orig: None,
            trans: PathBuf::from("guide/src"),
        }
    }
}

pub fn run(options: &Options) -> Result<bool, String> {
    let trans = util::absolute(&options.trans)?;
    let orig = resolve_orig(options.orig.as_deref())?;

    println!("Comparing translation against upstream:");
    println!("  Upstream:    {}", orig.display());
    println!("  Translation: {}", trans.display());

    let orig_files = util::collect_files(&orig, "md")?;
    let total = orig_files.len();
    let mut fail = 0usize;

    for file in &orig_files {
        let rel = file.strip_prefix(&orig).unwrap_or(file);
        let rel_display = util::slash(rel);
        let translated = trans.join(rel);
        if !translated.is_file() {
            println!("[FAIL] {rel_display} : missing translated file");
            fail += 1;
            continue;
        }

        let source = util::read_normalized_md(file)?;
        let target = util::read_normalized_md(&translated)?;

        compare_code_blocks(&source, &target, &rel_display, &mut fail);
        compare_headings(&source, &target, &rel_display, &mut fail);
        compare_ref_links(&source, &target, &rel_display, &mut fail);
        compare_inline_links(&source, &target, &rel_display, &mut fail);
    }

    println!("---");
    println!("Checked {total} files, {fail} problem(s)");
    if fail == 0 {
        println!("ALL OK: code blocks, headings, links match 100%");
    }
    Ok(fail == 0)
}

fn resolve_orig(explicit: Option<&Path>) -> Result<PathBuf, String> {
    if let Some(path) = explicit {
        if path.is_dir() {
            return util::absolute(path);
        }
        return Err(ORIG_ERROR.to_string());
    }
    for candidate in [
        "../cargo-bisect-rustc/guide/src",
        "cargo-bisect-rustc/guide/src",
    ] {
        let path = Path::new(candidate);
        if path.is_dir() {
            return util::absolute(path);
        }
    }
    Err(ORIG_ERROR.to_string())
}

fn compare_code_blocks(source: &str, target: &str, rel: &str, fail: &mut usize) {
    let orig: Vec<&str> = CODE_BLOCK.find_iter(source).map(|m| m.as_str()).collect();
    let trans: Vec<&str> = CODE_BLOCK.find_iter(target).map(|m| m.as_str()).collect();
    if orig.len() != trans.len() {
        println!(
            "[FAIL] {rel} : code block count differs (orig={} trans={})",
            orig.len(),
            trans.len()
        );
        *fail += 1;
        return;
    }
    for (index, (orig_block, trans_block)) in orig.iter().zip(trans.iter()).enumerate() {
        if orig_block != trans_block {
            println!("[FAIL] {rel} : code block #{} differs", index + 1);
            *fail += 1;
        }
    }
}

fn compare_headings(source: &str, target: &str, rel: &str, fail: &mut usize) {
    let orig: Vec<&str> = HEADING.find_iter(source).map(|m| m.as_str()).collect();
    let trans: Vec<&str> = HEADING.find_iter(target).map(|m| m.as_str()).collect();
    if orig.len() != trans.len() {
        println!(
            "[FAIL] {rel} : heading count differs (orig={} trans={})",
            orig.len(),
            trans.len()
        );
        *fail += 1;
        return;
    }
    for (index, (orig_heading, trans_heading)) in orig.iter().zip(trans.iter()).enumerate() {
        if heading_level(orig_heading) != heading_level(trans_heading) {
            println!(
                "[FAIL] {rel} : heading #{} level differs (orig='{}' trans='{}')",
                index + 1,
                orig_heading,
                trans_heading
            );
            *fail += 1;
        }
    }
}

fn compare_ref_links(source: &str, target: &str, rel: &str, fail: &mut usize) {
    let orig: Vec<&str> = REF_LINK
        .find_iter(source)
        .map(|m| m.as_str().trim_end())
        .collect();
    let trans: Vec<&str> = REF_LINK
        .find_iter(target)
        .map(|m| m.as_str().trim_end())
        .collect();
    if orig.len() != trans.len() {
        println!(
            "[FAIL] {rel} : ref-link count differs (orig={} trans={})",
            orig.len(),
            trans.len()
        );
        *fail += 1;
        return;
    }
    for (index, (orig_link, trans_link)) in orig.iter().zip(trans.iter()).enumerate() {
        let orig_url = normalize_link(ref_target(orig_link));
        let trans_url = normalize_link(ref_target(trans_link));
        if orig_url != trans_url {
            println!(
                "[FAIL] {rel} : ref-link #{} url differs (orig='{}' trans='{}')",
                index + 1,
                orig_url,
                trans_url
            );
            *fail += 1;
        }
    }
}

fn compare_inline_links(source: &str, target: &str, rel: &str, fail: &mut usize) {
    let orig = inline_targets(source);
    let trans = inline_targets(target);
    let missing: Vec<&str> = orig.difference(&trans).map(String::as_str).collect();
    let extra: Vec<&str> = trans.difference(&orig).map(String::as_str).collect();
    if !missing.is_empty() || !extra.is_empty() {
        println!(
            "[FAIL] {rel} : inline link targets differ (missing=[{}] extra=[{}])",
            missing.join(", "),
            extra.join(", ")
        );
        *fail += 1;
    }
}

fn inline_targets(text: &str) -> BTreeSet<String> {
    INLINE_LINK
        .captures_iter(text)
        .map(|captures| normalize_link(&captures[1]))
        .collect()
}

fn heading_level(heading: &str) -> &str {
    heading.split(' ').next().unwrap_or(heading)
}

fn ref_target(link: &str) -> &str {
    REF_SPLIT.splitn(link, 2).nth(1).unwrap_or("")
}

fn normalize_link(url: &str) -> String {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if trimmed.starts_with('#') {
        return "#anchor".to_string();
    }
    if let Some(captures) = FILE_ANCHOR.captures(trimmed) {
        return captures[1].to_string();
    }
    trimmed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchors_collapse_to_placeholder() {
        assert_eq!(normalize_link("#section"), "#anchor");
        assert_eq!(normalize_link("  #ส่วน"), "#anchor");
    }

    #[test]
    fn file_anchors_keep_only_the_file() {
        assert_eq!(normalize_link("usage.md#regression"), "usage.md");
        assert_eq!(
            normalize_link("https://example.com/a#b"),
            "https://example.com/a"
        );
        assert_eq!(normalize_link("a#b#c"), "a");
    }

    #[test]
    fn plain_urls_are_trimmed() {
        assert_eq!(
            normalize_link("  https://example.com  "),
            "https://example.com"
        );
        assert_eq!(normalize_link("   "), "");
    }

    #[test]
    fn heading_level_is_the_leading_hashes() {
        assert_eq!(heading_level("### Foo"), "###");
        assert_eq!(heading_level("##  Foo"), "##");
        assert_eq!(heading_level("#"), "#");
    }

    #[test]
    fn ref_target_splits_on_first_colon() {
        assert_eq!(
            ref_target("[ref]: https://example.com/a"),
            "https://example.com/a"
        );
        assert_eq!(
            ref_target("[ref]:   https://example.com"),
            "https://example.com"
        );
        assert_eq!(ref_target("[ref]: a:b"), "a:b");
    }
}
