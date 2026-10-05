use crate::util;
use regex::Regex;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::LazyLock;

static ID: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"id="([^"]+)""#).unwrap());
static HREF: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"href="([^"]*)""#).unwrap());

pub struct Options {
    pub book: PathBuf,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            book: PathBuf::from("guide/book"),
        }
    }
}

pub fn run(options: &Options) -> Result<bool, String> {
    let book = util::absolute(&options.book)?;
    if !book.is_dir() {
        return Err(format!("book directory not found: {}", book.display()));
    }

    let files = util::collect_files(&book, "html")?;
    let mut total = 0usize;
    let mut broken: Vec<String> = Vec::new();

    for file in &files {
        let content = util::read_utf8(file)?;
        let rel = util::slash(file.strip_prefix(&book).unwrap_or(file));
        let ids: HashSet<&str> = ID
            .captures_iter(&content)
            .map(|captures| captures.get(1).unwrap().as_str())
            .collect();

        for captures in HREF.captures_iter(&content) {
            let href = captures.get(1).unwrap().as_str();
            let lower = href.to_ascii_lowercase();
            if href.is_empty() || lower.starts_with("http") || lower.starts_with("javascript") {
                continue;
            }
            if let Some(target) = href.strip_prefix('#') {
                total += 1;
                if !ids.contains(target) {
                    broken.push(format!("{rel} -> #{target} (id not found)"));
                }
            } else if lower.contains(".html") {
                let page = href.split(['#', '?']).next().unwrap_or("");
                let target_path = file.parent().unwrap_or(&book).join(page);
                if !target_path.is_file() {
                    broken.push(format!("{rel} -> {href} (file missing)"));
                    continue;
                }
                if let Some((_, anchor)) = href.split_once('#') {
                    total += 1;
                    let target_content = util::read_utf8(&target_path)?;
                    if !target_content.contains(&format!("id=\"{anchor}\"")) {
                        broken.push(format!("{rel} -> {href} (anchor not found in target)"));
                    }
                }
            }
        }
    }

    println!("Checked {total} anchor links");
    if broken.is_empty() {
        println!("ALL ANCHOR LINKS OK");
        return Ok(true);
    }
    for entry in &broken {
        println!("[BROKEN] {entry}");
    }
    Ok(false)
}
