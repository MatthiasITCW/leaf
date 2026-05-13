use std::path::PathBuf;

use anyhow::{bail, Context, Result};

#[cfg(target_os = "windows")]
const PS1_COMPLETION: &str = include_str!("../completions/leaf.ps1");
const ZSH_COMPLETION: &str = include_str!("../completions/leaf.zsh");
const BASH_COMPLETION: &str = include_str!("../completions/leaf.bash");
const FISH_COMPLETION: &str = include_str!("../completions/leaf.fish");

#[allow(dead_code)]
enum Shell {
    Pwsh,
    Zsh,
    Bash,
    Fish,
}

fn detect_shell() -> Result<Shell> {
    if let Ok(shell) = std::env::var("SHELL") {
        let basename = std::path::Path::new(&shell)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        match basename {
            "zsh" => return Ok(Shell::Zsh),
            "bash" => return Ok(Shell::Bash),
            "fish" => return Ok(Shell::Fish),
            _ => {}
        }
    }

    #[cfg(target_os = "windows")]
    return Ok(Shell::Pwsh);

    #[cfg(not(target_os = "windows"))]
    {
        for (path, shell) in [
            ("/bin/zsh", Shell::Zsh),
            ("/bin/bash", Shell::Bash),
            ("/bin/fish", Shell::Fish),
        ] {
            if std::path::Path::new(path).exists() {
                return Ok(shell);
            }
        }
        bail!("Cannot detect shell. Set $SHELL to bash, zsh, or fish")
    }
}

fn completion_dir() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let base = std::env::var("APPDATA").context("Cannot determine APPDATA directory")?;
        Ok(PathBuf::from(base).join("leaf").join("completions"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        let home = std::env::var("HOME").context("Cannot determine HOME directory")?;
        Ok(PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("leaf")
            .join("completions"))
    }
}

fn fish_completion_dir() -> Result<PathBuf> {
    let home = std::env::var("HOME").context("Cannot determine HOME directory")?;
    Ok(PathBuf::from(home)
        .join(".config")
        .join("fish")
        .join("completions"))
}

fn write_completion(dir: &std::path::Path, filename: &str, content: &str) -> Result<PathBuf> {
    std::fs::create_dir_all(dir)
        .with_context(|| format!("Cannot create directory: {}", dir.display()))?;
    let path = dir.join(filename);
    std::fs::write(&path, content)
        .with_context(|| format!("Cannot write completion file: {}", path.display()))?;
    Ok(path)
}

fn rc_path(shell: &Shell) -> Result<PathBuf> {
    match shell {
        Shell::Zsh => {
            let home = std::env::var("HOME").context("Cannot determine HOME directory")?;
            Ok(PathBuf::from(home).join(".zshrc"))
        }
        Shell::Bash => {
            let home = std::env::var("HOME").context("Cannot determine HOME directory")?;
            Ok(PathBuf::from(home).join(".bashrc"))
        }
        Shell::Pwsh | Shell::Fish => {
            bail!("No RC file for this shell")
        }
    }
}

#[cfg(target_os = "windows")]
fn pwsh_profile_paths() -> Result<Vec<PathBuf>> {
    let base = std::env::var("USERPROFILE").context("Cannot determine USERPROFILE directory")?;
    let base = PathBuf::from(base).join("Documents");
    Ok(vec![
        base.join("PowerShell")
            .join("Microsoft.PowerShell_profile.ps1"),
        base.join("WindowsPowerShell")
            .join("Microsoft.PowerShell_profile.ps1"),
    ])
}

fn add_source_line(rc: &std::path::Path, line: &str) -> Result<bool> {
    if let Some(parent) = rc.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    let content = std::fs::read_to_string(rc).unwrap_or_default();
    if content.contains(line) {
        return Ok(false);
    }
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(rc)
        .with_context(|| format!("Cannot open {}", rc.display()))?;
    use std::io::Write;
    if !content.is_empty() && !content.ends_with('\n') {
        writeln!(file)?;
    }
    writeln!(file, "{line}")?;
    Ok(true)
}

pub(crate) fn install_completions() -> Result<()> {
    let shell = detect_shell()?;

    match shell {
        Shell::Pwsh => {
            #[cfg(target_os = "windows")]
            {
                let dir = completion_dir()?;
                let dest = write_completion(&dir, "leaf.ps1", PS1_COMPLETION)?;
                println!("Completion file installed: {}", dest.display());

                let source_line = format!(". {}", dest.display());
                for rc in pwsh_profile_paths()? {
                    if add_source_line(&rc, &source_line)? {
                        println!("Added to {}", rc.display());
                    } else {
                        println!("Already configured in {}", rc.display());
                    }
                }
                println!("\nRestart PowerShell to activate completions.");
            }
            #[cfg(not(target_os = "windows"))]
            bail!("PowerShell completion is only supported on Windows");
        }
        Shell::Zsh | Shell::Bash => {
            let (filename, content) = match shell {
                Shell::Zsh => ("_leaf", ZSH_COMPLETION),
                _ => ("leaf.bash", BASH_COMPLETION),
            };
            let dest = write_completion(&completion_dir()?, filename, content)?;
            println!("Completion file installed: {}", dest.display());

            let source_line = format!("source {}", dest.display());
            let rc = rc_path(&shell)?;
            if add_source_line(&rc, &source_line)? {
                println!("Added to {}", rc.display());
            } else {
                println!("Already configured in {}", rc.display());
            }
            println!("\nRestart your shell or run: source {}", rc.display());
        }
        Shell::Fish => {
            let dest = write_completion(&fish_completion_dir()?, "leaf.fish", FISH_COMPLETION)?;
            println!("Completion file installed: {}", dest.display());
            println!("\nCompletions are available in new fish sessions automatically.");
        }
    }

    Ok(())
}
