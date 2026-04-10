use eyre::{bail, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: update <path>");
        return Ok(());
    }

    let base_dir = args
        .get(1)
        .expect("Expected base directory as first argument")
        .parse::<PathBuf>()?;

    let repos = [
        Repo {
            url: "https://github.com/WebAssembly/spec",
            checkout: "wg-2.0",
            subdir: "test/core",
            dest: "wasm-v2",
        },
        Repo {
            url: "https://github.com/WebAssembly/spec",
            checkout: "wg-3.0",
            subdir: "test/core",
            dest: "wasm-v3",
        },
        Repo {
            url: "https://github.com/WebAssembly/spec",
            checkout: "main",
            subdir: "test/core",
            dest: "wasm-latest",
        },
    ];

    // Process specific spec versions
    for repo in repos {
        process_repo(&repo, &base_dir.join(repo.dest))?;
    }

    // Process latest proposals
    process_latest(&base_dir)?;

    // Process proposals
    load_proposals(&base_dir.join("proposals"))?;

    Ok(())
}

fn load_proposals(dest: &Path) -> Result<()> {
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "https://github.com/WebAssembly/testsuite",
            temp_path.to_str().unwrap(),
        ])
        .status()?;
    let source_path = temp_path.join("proposals");
    fs::create_dir_all(dest)?;

    for entry in fs::read_dir(source_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let proposal_name = entry.file_name();
            if proposal_name == "wasm-3.0" {
                continue;
            }

            let dest_proposal_path = dest.join(proposal_name);

            // remove the existing proposal directory if it exists
            if dest_proposal_path.exists() {
                fs::remove_dir_all(&dest_proposal_path)?;
            }

            fs::create_dir_all(&dest_proposal_path)?;
            copy_wast_files(&path, &dest_proposal_path)?;
        }
    }

    Ok(())
}

fn process_latest(base_dir: &Path) -> Result<()> {
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "--branch",
            "main",
            "https://github.com/WebAssembly/spec",
            temp_path.to_str().unwrap(),
        ])
        .status()?;

    let core_path = temp_path.join("test/core");

    let latest_dest = base_dir.join("wasm-latest");
    if latest_dest.exists() {
        fs::remove_dir_all(&latest_dest)?;
    }
    fs::create_dir_all(&latest_dest)?;
    copy_wast_files(&core_path, &latest_dest)?;

    let proposals_dest = base_dir.join("proposals");
    fs::create_dir_all(&proposals_dest)?;

    for entry in fs::read_dir(&core_path)? {
        let entry = entry?;
        let proposal_source = entry.path();
        if !proposal_source.is_dir() {
            continue;
        }

        const SKIP_PROPOSALS: &[&str] = &["memory64"];
        if SKIP_PROPOSALS
            .iter()
            .any(|skip| proposal_source.file_name().unwrap_or_default() == *skip)
        {
            continue;
        }

        let has_wast_files = fs::read_dir(&proposal_source)?.any(|proposal_entry| {
            proposal_entry
                .ok()
                .map(|proposal_entry| {
                    proposal_entry.path().is_file()
                        && proposal_entry
                            .path()
                            .extension()
                            .is_some_and(|ext| ext == "wast")
                })
                .unwrap_or(false)
        });

        if !has_wast_files {
            continue;
        }

        let proposal_dest = proposals_dest.join(entry.file_name());
        if proposal_dest.exists() {
            fs::remove_dir_all(&proposal_dest)?;
        }

        fs::create_dir_all(&proposal_dest)?;
        copy_wast_files(&proposal_source, &proposal_dest)?;
    }

    Ok(())
}

fn process_repo(repo: &Repo, dest: &Path) -> Result<()> {
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "--branch",
            repo.checkout,
            repo.url,
            temp_path.to_str().unwrap(),
        ])
        .status()?;

    // Copy only `.wast` files from the subdirectory to the destination
    let source_path = temp_path.join(repo.subdir);

    if dest.exists() {
        fs::remove_dir_all(dest)?;
    }

    fs::create_dir_all(dest)?;
    copy_wast_files(&source_path, dest)?;

    Ok(())
}

fn copy_wast_files(source: &Path, dest: &Path) -> Result<()> {
    if !source.is_dir() {
        bail!("Source path is not a directory: {:?}", source);
    }

    if !dest.is_dir() {
        bail!("Destination path is not a directory: {:?}", dest);
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "wast") {
            let dest_file_path = dest.join(entry.file_name());
            if let Some(parent) = dest_file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&path, &dest_file_path)?;
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Repo {
    url: &'static str,
    dest: &'static str,
    checkout: &'static str,
    subdir: &'static str,
}
