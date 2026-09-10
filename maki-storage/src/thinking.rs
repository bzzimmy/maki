use std::fs;

use crate::{StateDir, StorageError, atomic_write, sessions::StoredThinking};

const THINKING_FILE: &str = "thinking";

pub fn persist_thinking(dir: &StateDir, thinking: &StoredThinking) -> Result<(), StorageError> {
    fs::create_dir_all(dir.path())?;
    atomic_write(
        &dir.path().join(THINKING_FILE),
        &serde_json::to_vec(thinking)?,
    )
}

pub fn read_thinking(dir: &StateDir) -> Option<StoredThinking> {
    let raw = fs::read(dir.path().join(THINKING_FILE)).ok()?;
    serde_json::from_slice(&raw).ok()
}

#[cfg(test)]
mod tests {
    use super::{THINKING_FILE, persist_thinking, read_thinking};
    use crate::{
        StateDir,
        sessions::{Effort, StoredThinking},
    };
    use std::fs;
    use tempfile::TempDir;
    use test_case::test_case;

    #[test_case(StoredThinking::Off; "off")]
    #[test_case(StoredThinking::Adaptive; "adaptive")]
    #[test_case(StoredThinking::Effort { level: Effort::High }; "effort")]
    #[test_case(StoredThinking::Budget { tokens: 4096 }; "budget")]
    fn round_trip_and_overwrite(thinking: StoredThinking) {
        let tmp = TempDir::new().unwrap();
        let dir = StateDir::from_path(tmp.path().join("state"));
        assert_eq!(read_thinking(&dir), None);
        persist_thinking(&dir, &thinking).unwrap();
        assert_eq!(read_thinking(&dir), Some(thinking));
        persist_thinking(&dir, &StoredThinking::Off).unwrap();
        assert_eq!(read_thinking(&dir), Some(StoredThinking::Off));
    }

    #[test_case(""; "empty")]
    #[test_case(" \n"; "whitespace")]
    #[test_case("{"; "malformed")]
    #[test_case(r#"{"kind":"unknown"}"#; "unknown_kind")]
    #[test_case(r#"{"kind":"budget"}"#; "missing_tokens")]
    fn invalid_preference_is_ignored(raw: &str) {
        let tmp = TempDir::new().unwrap();
        let dir = StateDir::from_path(tmp.path().to_path_buf());
        fs::write(dir.path().join(THINKING_FILE), raw).unwrap();
        assert_eq!(read_thinking(&dir), None);
    }

    #[test]
    fn write_failure_is_returned() {
        let tmp = TempDir::new().unwrap();
        let dir = StateDir::from_path(tmp.path().to_path_buf());
        fs::create_dir(dir.path().join(THINKING_FILE)).unwrap();
        assert!(persist_thinking(&dir, &StoredThinking::Adaptive).is_err());
        assert_eq!(read_thinking(&dir), None);
    }
}
