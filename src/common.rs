use std::ffi::OsStr;
use std::path::{Path,PathBuf};

pub fn get_module_dir() -> Option<PathBuf> {
    let canonical_path = std::env::current_exe().ok()?
        .canonicalize().ok()?;
    Some(canonical_path.parent()?.to_path_buf())
}

pub fn get_parent_dir(path: &Path) -> Option<PathBuf> {
    let canonical_path = path.canonicalize().ok()?;
    Some(canonical_path.parent()?.to_path_buf())
}

pub fn get_path_str(path: &Path) -> String {
    path
    .to_string_lossy()
    .to_string()
}

pub fn get_unique_path(path: &Path) -> PathBuf {
    if !path.exists() {
        return path.to_path_buf();
    }
    let ext = path.extension().unwrap_or(OsStr::new("")).to_string_lossy();
    let path_without_ext = path.with_extension("").to_string_lossy().into_owned();
    (1u32..).find_map(|n| {
        let u_path = PathBuf::from(
            if ext.is_empty() {
                format!("{} ({})", path_without_ext, n)
            } else {
                format!("{} ({}).{}", path_without_ext, n, ext)
            }
        );
        (!u_path.exists()).then_some(u_path)
    }).unwrap_or_else(|| unreachable!("は？"))
}
