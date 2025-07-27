use std::{env::current_dir, path::PathBuf}; // current_exe

/// Returns the parent folder where the bin is located
pub fn _root_path() -> PathBuf {
    current_dir().unwrap().parent().unwrap().to_path_buf()
}

pub fn _rel_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../local")
}

/// Open technically anything, just pass a link or path.
///
/// # Example
/// ```rs
/// use utils::snips::open_url;
///
/// open_url("http://localhost:4989/");
/// ```
pub fn open_url<S>(url: S)
where
    S: AsRef<str>,
{
    match std::env::consts::OS {
        "windows" => std::process::Command::new("cmd")
            .arg("/C")
            .arg("start")
            .arg(url.as_ref())
            .output()
            .expect("failed to execute process"),
        "macos" => std::process::Command::new("open")
            .arg(url.as_ref())
            .output()
            .expect("failed to execute process"),
        _ => std::process::Command::new("xdg-open")
            .arg(url.as_ref())
            .output()
            .expect("failed to execute process"),
    };
}
