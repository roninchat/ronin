//! Composer completion logic for `@` context commands, memories, and file paths.

use std::path::{Path, PathBuf};

/// Returns the whitespace-delimited token ending at `cursor` and its start byte.
pub fn token_before_cursor(text: &str, cursor: usize) -> (usize, &str) {
    let cursor = cursor.min(text.len());
    let start = text[..cursor]
        .rfind(char::is_whitespace)
        .map(|i| i + 1)
        .unwrap_or(0);
    (start, &text[start..cursor])
}

/// A context command completion: `(command, label, token_start, token_end)`.
pub type CommandCompletion = (String, String, usize, usize);

/// Suggests an `@` context command matching the token at the cursor.
pub fn command_completion(text: &str, cursor: usize) -> Option<CommandCompletion> {
    let (start, token) = token_before_cursor(text, cursor);
    if !token.starts_with('@') {
        return None;
    }
    let tl = token.to_ascii_lowercase();
    [
        ("@file:", "Attach file"),
        ("@memory:", "Attach memory"),
        ("@artifact:", "Attach artifact"),
        ("@screenshot", "Capture screenshot"),
        ("@clipboard", "Attach clipboard"),
    ]
    .iter()
    .find(|(c, _)| c.to_ascii_lowercase().starts_with(&tl) && *c != tl)
    .map(|(c, l)| (c.to_string(), l.to_string(), start, cursor))
}

/// Extracts the `@memory:` prefix at the cursor, if the token is a memory ref.
pub fn memory_completion_prefix(text: &str, cursor: usize) -> Option<&str> {
    let (_, token) = token_before_cursor(text, cursor);
    token.strip_prefix("@memory:")
}

/// Filters `(id, title)` memory candidates against a typed prefix.
pub fn filter_memory_completions(
    prefix: &str,
    memories: impl IntoIterator<Item = (String, String)>,
) -> Vec<(String, String)> {
    let prefix_lower = prefix.to_lowercase();
    let mut matches: Vec<(String, String)> = memories
        .into_iter()
        .filter(|(id, title)| {
            id.starts_with(prefix) || title.to_lowercase().contains(&prefix_lower)
        })
        .collect();
    matches.truncate(8);
    matches
}

/// Extracts the `@artifact:` prefix at the cursor, if the token is an artifact ref.
pub fn artifact_completion_prefix(text: &str, cursor: usize) -> Option<&str> {
    let (_, token) = token_before_cursor(text, cursor);
    token.strip_prefix("@artifact:")
}

/// Filters `(id, title)` artifact candidates (panel list) against a typed prefix.
pub fn filter_artifact_completions(
    prefix: &str,
    artifacts: impl IntoIterator<Item = (String, String)>,
) -> Vec<(String, String)> {
    filter_memory_completions(prefix, artifacts)
}

/// Returns true when the token at `cursor` is an `@file:` path completion.
pub fn file_path_completion_active(text: &str, cursor: usize) -> bool {
    let (_, token) = token_before_cursor(text, cursor);
    token.starts_with("@file:")
}

/// Suggests directory entries for the `@file:` path token at the cursor.
///
/// Directories are listed first with a trailing `/`, then files, both sorted
/// alphabetically and truncated to 8 entries.
pub fn file_path_completions(text: &str, cursor: usize) -> Vec<String> {
    let (_, token) = token_before_cursor(text, cursor);
    let prefix = match token.strip_prefix("@file:") {
        Some(p) => {
            if p.starts_with('"') {
                p.strip_prefix('"').unwrap_or(p).trim_end_matches('"')
            } else {
                p
            }
        }
        None => return Vec::new(),
    };

    let (dir, file_prefix) = split_path_prefix(prefix);
    let file_prefix_lower = file_prefix.to_ascii_lowercase();

    let mut matches = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name == "." || name == ".." {
                continue;
            }
            let name_lower = name.to_ascii_lowercase();
            if !file_prefix_lower.is_empty() && !name_lower.starts_with(&file_prefix_lower) {
                continue;
            }
            let suffix = if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                "/"
            } else {
                ""
            };
            matches.push(format!("{name}{suffix}"));
        }
    }
    matches.sort_by(|a, b| {
        // Directories first, then alphabetical
        let a_dir = a.ends_with('/');
        let b_dir = b.ends_with('/');
        b_dir.cmp(&a_dir).then_with(|| a.cmp(b))
    });
    matches.truncate(8);
    matches
}

/// Resolves a typed path prefix into `(directory to list, file name prefix)`.
fn split_path_prefix(prefix: &str) -> (PathBuf, String) {
    let home = || std::env::var("HOME").unwrap_or_else(|_| "/".into());
    let prefix_path = Path::new(prefix);

    if prefix.is_empty() || prefix == "/" {
        // Empty or just "/" — list home
        return (PathBuf::from(home()), String::new());
    }

    if prefix.ends_with('/') {
        // Explicit directory — list its contents
        let d = prefix_path.to_path_buf();
        if d.is_dir() {
            return (d, String::new());
        }
        // Try resolving via HOME
        let home = home();
        let full = PathBuf::from(&home).join(prefix_path);
        if full.is_dir() {
            return (full, String::new());
        }
        return (
            full.parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from(&home)),
            full.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string(),
        );
    }

    if prefix_path.is_dir() {
        // Path is an existing directory — list its contents
        return (prefix_path.to_path_buf(), String::new());
    }

    // Path is a partial — get parent dir and file name prefix
    let parent = prefix_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from(home()));
    let dir = if prefix.starts_with('/') {
        parent
    } else if parent.as_os_str().is_empty() {
        // Just a filename — search cwd first, fallback to HOME
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    } else if parent.is_dir() {
        parent
    } else {
        let full = PathBuf::from(home()).join(&parent);
        if full.is_dir() {
            full
        } else {
            parent
        }
    };
    let fname = prefix_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();
    (dir, fname)
}

/// Builds the directory portion inserted when a file completion is accepted.
///
/// Returns the typed base directory with a trailing `/`, falling back to the
/// user's home directory when the base is empty or not yet a directory.
pub fn completion_dir_prefix(base: &str) -> String {
    let home = || std::env::var("HOME").unwrap_or_else(|_| "/".into());
    let base_path = Path::new(base);
    if base.ends_with('/') {
        if base == "/" || base.is_empty() {
            format!("{}/", home())
        } else {
            base.to_string()
        }
    } else if base.is_empty() {
        format!("{}/", home())
    } else if base_path.is_dir() {
        format!("{base}/")
    } else {
        base_path
            .parent()
            .and_then(|p| p.to_str())
            .filter(|d| !d.is_empty())
            .map(|d| format!("{d}/"))
            .unwrap_or_else(|| format!("{}/", home()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_completion_should_suggest_file_command_for_at_prefix() {
        let text = "hello @fi";
        let result = command_completion(text, text.len());
        assert_eq!(
            result,
            Some(("@file:".to_string(), "Attach file".to_string(), 6, 9))
        );
    }

    #[test]
    fn command_completion_should_return_none_without_at_token() {
        let text = "hello world";
        assert_eq!(command_completion(text, text.len()), None);
    }

    #[test]
    fn command_completion_should_return_none_when_command_fully_typed() {
        let text = "@clipboard";
        assert_eq!(command_completion(text, text.len()), None);
    }

    #[test]
    fn filter_memory_completions_should_match_id_prefix_and_title_substring() {
        let memories = vec![
            ("abc123".to_string(), "Coffee preferences".to_string()),
            ("def456".to_string(), "Work notes".to_string()),
        ];
        let matches = filter_memory_completions("coffee", memories);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].0, "abc123");
    }

    #[test]
    fn command_completion_should_suggest_artifact_command() {
        let text = "see @art";
        let result = command_completion(text, text.len());
        assert_eq!(
            result,
            Some((
                "@artifact:".to_string(),
                "Attach artifact".to_string(),
                4,
                8
            ))
        );
    }

    #[test]
    fn command_completion_should_suggest_screenshot_command() {
        let text = "look @screen";
        let result = command_completion(text, text.len());
        assert_eq!(
            result,
            Some((
                "@screenshot".to_string(),
                "Capture screenshot".to_string(),
                5,
                12
            ))
        );
    }

    #[test]
    fn artifact_completion_prefix_should_extract_typed_prefix() {
        let text = "use @artifact:ref";
        assert_eq!(artifact_completion_prefix(text, text.len()), Some("ref"));
    }

    #[test]
    fn filter_artifact_completions_should_match_panel_list_by_id_or_title() {
        let artifacts = vec![
            ("art-aaa".to_string(), "API client".to_string()),
            ("art-bbb".to_string(), "UI mock".to_string()),
        ];
        let by_title = filter_artifact_completions("api", artifacts.clone());
        assert_eq!(by_title.len(), 1);
        assert_eq!(by_title[0].0, "art-aaa");

        let by_id = filter_artifact_completions("art-b", artifacts);
        assert_eq!(by_id.len(), 1);
        assert_eq!(by_id[0].0, "art-bbb");
    }

    #[test]
    fn token_before_cursor_should_return_last_whitespace_delimited_token() {
        let text = "one two three";
        let (start, token) = token_before_cursor(text, text.len());
        assert_eq!(start, 8);
        assert_eq!(token, "three");
    }
}
