use eyre::{bail, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

// not updated anymore, local copy has patches to support the latest wast syntax
//
// const WASM_1: Repo = Repo {
//     url: "https://github.com/WebAssembly/spec",
//     checkout: "wg-1.0",
//     subdir: "test/core",
//     name: "wasm-v1",
// };

const WASM_2: Repo = Repo {
    url: "https://github.com/WebAssembly/spec",
    checkout: "wg-2.0.draft1",
    subdir: "test/core",
    name: "wasm-v2",
};

const WASM_3: Repo = Repo {
    url: "https://github.com/WebAssembly/spec",
    checkout: "wasm-3.0",
    subdir: "test/core",
    name: "wasm-v3",
};

const WASM_MAIN: Repo = Repo {
    url: "https://github.com/WebAssembly/spec",
    checkout: "main",
    subdir: "test/core",
    name: "wasm-latest",
};

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

    let repos = [WASM_2, WASM_3, WASM_MAIN];

    // Process specific spec versions
    for repo in repos {
        process_repo(&repo, &base_dir.join(repo.name))?;
    }

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
    name: &'static str,
    checkout: &'static str,
    subdir: &'static str,
}
