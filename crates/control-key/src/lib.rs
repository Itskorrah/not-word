//! The control channel's key file, one rule for the app (which writes it) and the MCP bridge
//! (which reads it): `<settings>/control-key.<instance>`. See `docs/control-protocol.md`, Keys.

use std::path::{Path, PathBuf};

/// The app's settings folder (preferences, logs, key files): macOS
/// `~/Library/Application Support/Not Word`, Windows `%APPDATA%\NotWord`, otherwise
/// `$XDG_CONFIG_HOME/not-word` or `~/.config/not-word`.
pub fn settings_dir() -> Option<PathBuf> {
    settings_dir_named("Not Word", "NotWord", "not-word")
}

/// The upstream settings directory, for reading existing preferences and for bridging a
/// still-running older app. New writes always use [`settings_dir`].
pub fn legacy_settings_dir() -> Option<PathBuf> {
    settings_dir_named("WordCraft", "WordCraft", "wordcraft")
}

fn settings_dir_named(macos: &str, windows: &str, unix: &str) -> Option<PathBuf> {
    if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Library/Application Support").join(macos))
    } else if cfg!(windows) {
        std::env::var_os("APPDATA").map(|a| PathBuf::from(a).join(windows))
    } else {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
            .map(|c| c.join(unix))
    }
}

/// Read existing upstream preferences only when the independently branded file has not
/// been written yet. The old file is never moved, overwritten or deleted.
pub fn settings_file_for_read(name: &str) -> Option<PathBuf> {
    Some(preferred_file(&settings_dir()?, legacy_settings_dir().as_deref(), name))
}

fn preferred_file(current: &Path, legacy: Option<&Path>, name: &str) -> PathBuf {
    let path = current.join(name);
    if !path.exists()
        && let Some(old) = legacy.map(|d| d.join(name)).filter(|p| p.is_file())
    {
        return old;
    }
    path
}

/// The `instance-id=` value of a `/.flatpak-info` file, only when it is `[A-Za-z0-9_-]+` (it
/// becomes part of a file name).
pub fn parse_instance_id(info: &str) -> Option<String> {
    let v = info.lines().find_map(|l| l.strip_prefix("instance-id="))?.trim();
    (!v.is_empty() && v.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')).then(|| v.to_string())
}

/// The Flatpak instance id, or `None` outside Flatpak.
pub fn instance_id() -> Option<String> {
    parse_instance_id(&std::fs::read_to_string("/.flatpak-info").ok()?)
}

/// `<settings>/control-key.<instance>`, with `<instance>` the Flatpak instance id or
/// `local-<port>`.
pub fn key_path(settings: &Path, port: u16) -> PathBuf {
    key_path_for(settings, instance_id().as_deref(), port)
}

fn key_path_for(settings: &Path, instance: Option<&str>, port: u16) -> PathBuf {
    match instance {
        Some(id) => settings.join(format!("control-key.{id}")),
        None => settings.join(format!("control-key.local-{port}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn instance_id_only_safe_values() {
        assert_eq!(parse_instance_id("[Instance]\ninstance-id=abc_123-X\n").as_deref(), Some("abc_123-X"));
        assert_eq!(parse_instance_id("instance-id= 42 \n").as_deref(), Some("42"));
        // It ends up in a file name.
        assert_eq!(parse_instance_id("instance-id=../etc\n"), None);
        assert_eq!(parse_instance_id("instance-id=a/b\n"), None);
        assert_eq!(parse_instance_id("instance-id=a b\n"), None);
        assert_eq!(parse_instance_id("instance-id=..\n"), None);
        assert_eq!(parse_instance_id("instance-id=\n"), None);
        assert_eq!(parse_instance_id("app=ai.storyteller.wordcraft\n"), None);
    }

    #[test]
    fn key_file_per_instance() {
        let dir = Path::new("/settings/wordcraft");
        assert_eq!(key_path_for(dir, Some("1234"), 7981), dir.join("control-key.1234"));
        assert_eq!(key_path_for(dir, None, 7981), dir.join("control-key.local-7981"));
        assert_eq!(key_path_for(dir, None, 7982), dir.join("control-key.local-7982"));
        if instance_id().is_none() {
            assert_eq!(key_path(dir, 7981), dir.join("control-key.local-7981"));
        }
    }

    #[test]
    fn settings_dir_is_the_apps_folder() {
        if let Some(d) = settings_dir() {
            let name = d.file_name().and_then(|n| n.to_str()).unwrap_or("");
            assert!(name == "not-word" || name == "Not Word" || name == "NotWord", "{}", d.display());
        }
    }

    #[test]
    fn legacy_settings_dir_retains_the_upstream_folder() {
        if let Some(dir) = legacy_settings_dir() {
            let expected = if cfg!(windows) || cfg!(target_os = "macos") { "WordCraft" } else { "wordcraft" };
            assert_eq!(dir.file_name().and_then(|n| n.to_str()), Some(expected));
            assert_ne!(Some(dir), settings_dir());
        }
    }

    #[test]
    fn existing_new_preferences_win_and_legacy_files_are_preserved() {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "not-word-migration-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        let current = root.join("not-word");
        let legacy = root.join("wordcraft");
        std::fs::create_dir_all(&current).unwrap();
        std::fs::create_dir_all(&legacy).unwrap();
        let old = legacy.join("ui.json");
        std::fs::write(&old, b"legacy").unwrap();
        assert_eq!(preferred_file(&current, Some(&legacy), "ui.json"), old);
        let new = current.join("ui.json");
        std::fs::write(&new, b"new").unwrap();
        assert_eq!(preferred_file(&current, Some(&legacy), "ui.json"), new);
        assert_eq!(std::fs::read(&old).unwrap(), b"legacy");
        assert_eq!(preferred_file(&current, Some(&legacy), "missing.json"), current.join("missing.json"));
        std::fs::remove_dir_all(root).unwrap();
    }
}
