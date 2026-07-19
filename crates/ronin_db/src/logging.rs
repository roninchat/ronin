//! Persistent file logging helpers and secret redaction.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Replacement text for redacted sensitive material.
pub const REDACTED_PLACEHOLDER: &str = "[REDACTED]";

/// Default max size of a single log file before rotation (5 MiB).
pub const DEFAULT_MAX_LOG_FILE_BYTES: u64 = 5 * 1024 * 1024;

/// Applies Ronin's stable log redaction policy to a text fragment.
///
/// Strips API keys/secrets, prompt/message field values, raw provider JSON
/// payloads (`messages` / `choices`), and credential-bearing URLs.
pub fn redact_log_text(input: &str) -> String {
    let mut out = input.to_string();

    // Credential-bearing URLs: scheme://user:pass@host
    out = redact_userinfo_urls(&out);

    // Query params with secrets
    out = redact_query_secrets(&out);

    // Bearer / sk- API keys
    out = redact_api_keys(&out);

    // password=/api_key=/token=/secret= assignments
    out = redact_assignments(&out);

    // prompt=/content=/message= quoted or unquoted values
    out = redact_content_fields(&out);

    // Raw provider JSON payloads containing messages/choices arrays
    out = redact_provider_payloads(&out);

    out
}

fn redact_userinfo_urls(input: &str) -> String {
    let mut result = String::new();
    let mut rest = input;
    while let Some(idx) = rest.find("://") {
        result.push_str(&rest[..idx + 3]);
        let after = &rest[idx + 3..];
        let authority_end = after.find('/').unwrap_or(after.len());
        let authority = &after[..authority_end];
        if let Some(at) = authority.rfind('@') {
            let cred = &authority[..at];
            if cred.contains(':') {
                result.push_str(REDACTED_PLACEHOLDER);
                result.push('@');
                result.push_str(&authority[at + 1..]);
                rest = &after[authority_end..];
                continue;
            }
        }
        rest = after;
    }
    result.push_str(rest);
    result
}

fn redact_query_secrets(input: &str) -> String {
    let keys = ["api_key=", "token=", "key=", "secret=", "password=", "access_token="];
    let mut out = input.to_string();
    for key in keys {
        out = redact_after_key(&out, key);
    }
    out
}

fn redact_after_key(input: &str, key: &str) -> String {
    let lower_key = key.to_ascii_lowercase();
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    loop {
        let lower_rest = rest.to_ascii_lowercase();
        let Some(idx) = lower_rest.find(&lower_key) else {
            out.push_str(rest);
            break;
        };
        out.push_str(&rest[..idx]);
        out.push_str(&rest[idx..idx + key.len()]);
        let after = &rest[idx + key.len()..];
        let end = after
            .find(|c: char| c.is_whitespace() || c == '&' || c == '"' || c == '\'' || c == '`' || c == ',')
            .unwrap_or(after.len());
        if end > 0 {
            out.push_str(REDACTED_PLACEHOLDER);
            rest = &after[end..];
        } else {
            rest = after;
        }
    }
    out
}

fn redact_api_keys(input: &str) -> String {
    let mut out = redact_bearer(input);
    out = redact_sk_tokens(&out);
    out
}

fn redact_bearer(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    loop {
        let lower = rest.to_ascii_lowercase();
        let Some(idx) = lower.find("bearer ") else {
            out.push_str(rest);
            break;
        };
        out.push_str(&rest[..idx]);
        out.push_str("Bearer ");
        let after = &rest[idx + "bearer ".len()..];
        let end = after
            .find(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == ',')
            .unwrap_or(after.len());
        out.push_str(REDACTED_PLACEHOLDER);
        rest = &after[end..];
    }
    out
}

fn redact_sk_tokens(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    while let Some(idx) = rest.find("sk-") {
        out.push_str(&rest[..idx]);
        let after = &rest[idx..];
        let end = after[3..]
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '-'))
            .map(|i| i + 3)
            .unwrap_or(after.len());
        out.push_str(REDACTED_PLACEHOLDER);
        rest = &after[end..];
    }
    out.push_str(rest);
    out
}

fn redact_assignments(input: &str) -> String {
    let mut out = input.to_string();
    for key in [
        "password=",
        "api_key=",
        "apikey=",
        "token=",
        "secret=",
        "access_token=",
    ] {
        out = redact_after_key(&out, key);
    }
    out
}

fn redact_content_fields(input: &str) -> String {
    let mut out = input.to_string();
    for key in ["prompt=", "content=", "message="] {
        out = redact_field_value(&out, key);
    }
    out
}

fn redact_field_value(input: &str, key: &str) -> String {
    let lower_key = key.to_ascii_lowercase();
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    loop {
        let lower_rest = rest.to_ascii_lowercase();
        let Some(idx) = lower_rest.find(&lower_key) else {
            out.push_str(rest);
            break;
        };
        out.push_str(&rest[..idx]);
        out.push_str(&rest[idx..idx + key.len()]);
        let after = &rest[idx + key.len()..];
        let (redacted_len, skip) = if after.starts_with('"') {
            if let Some(end) = after[1..].find('"') {
                (end + 2, end + 2)
            } else {
                (after.len(), after.len())
            }
        } else if after.starts_with('\'') {
            if let Some(end) = after[1..].find('\'') {
                (end + 2, end + 2)
            } else {
                (after.len(), after.len())
            }
        } else {
            let end = after
                .find(|c: char| c.is_whitespace() || c == ',' || c == '}')
                .unwrap_or(after.len());
            (end, end)
        };
        let _ = redacted_len;
        out.push_str(REDACTED_PLACEHOLDER);
        rest = &after[skip..];
    }
    out
}

fn redact_provider_payloads(input: &str) -> String {
    // If a JSON-looking blob includes "messages" or "choices", redact from first { to matching }.
    if !(input.contains("\"messages\"") || input.contains("\"choices\"")) {
        return input.to_string();
    }
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    while let Some(start) = rest.find('{') {
        // Only redact objects that look like provider payloads.
        let candidate = &rest[start..];
        let end = find_matching_brace(candidate).unwrap_or(candidate.len());
        let obj = &candidate[..end];
        out.push_str(&rest[..start]);
        if obj.contains("\"messages\"") || obj.contains("\"choices\"") {
            out.push_str(REDACTED_PLACEHOLDER);
        } else {
            out.push_str(obj);
        }
        rest = &candidate[end..];
    }
    out.push_str(rest);
    out
}

fn find_matching_brace(s: &str) -> Option<usize> {
    let mut depth = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i + 1);
                }
            }
            _ => {}
        }
    }
    None
}

/// Options for optional persistent file logging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileLogOptions {
    /// When true, also write rotated logs under [`FileLogOptions::log_dir`].
    pub enabled: bool,
    /// Directory for log files (typically `~/.cache/ronin/logs`).
    pub log_dir: PathBuf,
    /// Rotate the active file once it exceeds this many bytes.
    pub max_file_bytes: u64,
}

impl Default for FileLogOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            log_dir: PathBuf::from("."),
            max_file_bytes: DEFAULT_MAX_LOG_FILE_BYTES,
        }
    }
}

/// Resolves the default Ronin log directory under an XDG cache base.
pub fn default_log_dir(cache_home: &Path) -> PathBuf {
    cache_home.join("ronin").join("logs")
}

/// Size-rotating, redacting log file writer for Ronin diagnostics.
pub struct RotatingLogWriter {
    dir: PathBuf,
    max_file_bytes: u64,
    file: Mutex<Option<File>>,
    current_name: Mutex<String>,
}

impl RotatingLogWriter {
    /// Creates a writer that appends to `ronin.log` under `dir`, rotating by size.
    pub fn open(dir: impl Into<PathBuf>, max_file_bytes: u64) -> io::Result<Self> {
        let dir = dir.into();
        fs::create_dir_all(&dir)?;
        let path = dir.join("ronin.log");
        let file = OpenOptions::new().create(true).append(true).open(&path)?;
        Ok(Self {
            dir,
            max_file_bytes: max_file_bytes.max(32),
            file: Mutex::new(Some(file)),
            current_name: Mutex::new("ronin.log".into()),
        })
    }

    /// Appends a redacted line (adds trailing newline if missing).
    pub fn append_line(&self, line: &str) -> io::Result<()> {
        let redacted = redact_log_text(line);
        let mut payload = redacted;
        if !payload.ends_with('\n') {
            payload.push('\n');
        }
        self.write_all(payload.as_bytes())
    }

    pub(crate) fn write_all(&self, buf: &[u8]) -> io::Result<()> {
        let mut guard = self.file.lock().unwrap_or_else(|e| e.into_inner());
        let file = guard
            .as_mut()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "log file closed"))?;
        let meta = file.metadata()?;
        if meta.len() + buf.len() as u64 > self.max_file_bytes {
            self.rotate_locked(&mut guard)?;
        }
        if let Some(file) = guard.as_mut() {
            file.write_all(buf)?;
            file.flush()?;
        }
        Ok(())
    }

    fn rotate_locked(&self, guard: &mut Option<File>) -> io::Result<()> {
        let _ = guard.take();
        let active = self.dir.join("ronin.log");
        let rotated = self.dir.join("ronin.log.1");
        if active.exists() {
            let _ = fs::remove_file(&rotated);
            fs::rename(&active, &rotated)?;
        }
        let file = OpenOptions::new().create(true).append(true).open(&active)?;
        *guard = Some(file);
        if let Ok(mut name) = self.current_name.lock() {
            *name = "ronin.log".into();
        }
        Ok(())
    }

    /// Flushes the active log file.
    pub fn flush_writer(&self) -> io::Result<()> {
        let mut guard = self.file.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(file) = guard.as_mut() {
            file.flush()?;
        }
        Ok(())
    }

    /// Active log file path (`…/ronin.log`).
    pub fn active_path(&self) -> PathBuf {
        self.dir.join("ronin.log")
    }
}

impl Write for &RotatingLogWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let text = String::from_utf8_lossy(buf);
        let redacted = redact_log_text(&text);
        self.write_all(redacted.as_bytes())?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut guard = self.file.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(file) = guard.as_mut() {
            file.flush()?;
        }
        Ok(())
    }
}
