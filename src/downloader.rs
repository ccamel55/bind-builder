use std::path::{Path, PathBuf};
use std::process::Command;

pub fn clone_repo(
    path: &Path,
    id: &str,
    url: &str,
    tag: &str,
) -> PathBuf {

    let args = [
        "clone", url,
        "--depth", "1",
        "--branch", tag,
        "--recurse",
        id
    ];

    Command::new("git")
        .args(args)
        .current_dir(path)
        .status()
        .expect("Could not clone repo, is Git installed?");

    return path.join(id)
}