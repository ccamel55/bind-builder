use std::path::PathBuf;
use std::process::Command;
use crate::variables::manifest_dir;

pub fn clone_repo(
    id: &str,
    url: &str,
    tag: &str,
) -> PathBuf {

    let path = manifest_dir();
    let repo_dir = path.join(id);

    let args = [
        "clone", url,
        "--depth", "1",
        "--branch", tag,
        "--recurse",
        id
    ];

    // Todo: Update the repo blah blah...
    if repo_dir.exists() {
        return repo_dir;
    }

    // Clone
    Command::new("git")
        .args(args)
        .current_dir(path.clone())
        .status()
        .expect("Could not clone repo, is Git installed?");

    repo_dir
}