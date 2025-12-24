//! CLI for skills-ref-rs library.

use std::path::{Path, PathBuf};
use std::process;

use clap::{Parser, Subcommand};

use skills_ref::{read_properties, to_prompt, validate};

#[derive(Parser)]
#[command(name = "skills-ref-rs")]
#[command(about = "Reference library for Agent Skills")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a skill directory.
    ///
    /// Checks that the skill has a valid SKILL.md with proper frontmatter,
    /// correct naming conventions, and required fields.
    Validate {
        /// Path to the skill directory or SKILL.md file
        skill_path: PathBuf,
    },

    /// Read and print skill properties as JSON.
    ///
    /// Parses the YAML frontmatter from SKILL.md and outputs the
    /// properties as JSON.
    #[command(name = "read-properties")]
    ReadProperties {
        /// Path to the skill directory or SKILL.md file
        skill_path: PathBuf,
    },

    /// Generate <available_skills> XML for agent prompts.
    ///
    /// Accepts one or more skill directories.
    #[command(name = "to-prompt")]
    ToPrompt {
        /// Paths to skill directories or SKILL.md files
        #[arg(required = true)]
        skill_paths: Vec<PathBuf>,
    },
}

/// Check if a path points directly to a SKILL.md or skill.md file.
fn is_skill_md_file(path: &Path) -> bool {
    path.is_file()
        && path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.to_lowercase() == "skill.md")
            .unwrap_or(false)
}

/// Resolve a skill path - if it's a SKILL.md file, return its parent directory.
fn resolve_skill_path(path: PathBuf) -> PathBuf {
    if is_skill_md_file(&path) {
        path.parent().map(|p| p.to_path_buf()).unwrap_or(path)
    } else {
        path
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { skill_path } => {
            let skill_path = resolve_skill_path(skill_path);
            let errors = validate(&skill_path);

            if errors.is_empty() {
                println!("Valid skill: {}", skill_path.display());
            } else {
                eprintln!("Validation failed for {}:", skill_path.display());
                for error in errors {
                    eprintln!("  - {}", error);
                }
                process::exit(1);
            }
        }

        Commands::ReadProperties { skill_path } => {
            let skill_path = resolve_skill_path(skill_path);

            match read_properties(&skill_path) {
                Ok(props) => {
                    let json = serde_json::to_string_pretty(&props).unwrap();
                    println!("{}", json);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }

        Commands::ToPrompt { skill_paths } => {
            let resolved_paths: Vec<PathBuf> =
                skill_paths.into_iter().map(resolve_skill_path).collect();

            let path_refs: Vec<&std::path::Path> =
                resolved_paths.iter().map(|p| p.as_path()).collect();

            match to_prompt(&path_refs) {
                Ok(output) => {
                    println!("{}", output);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}
