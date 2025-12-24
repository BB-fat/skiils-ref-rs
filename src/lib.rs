//! # skills-ref-rs
//!
//! Rust implementation of the agentskills library for validating, parsing,
//! and managing Agent Skills - structured markdown documents that describe
//! capabilities available to AI assistants.
//!
//! ## Core Functionality
//!
//! 1. **Validate** - Check that a skill directory contains a valid SKILL.md
//! 2. **Read Properties** - Parse SKILL.md frontmatter and extract metadata as JSON
//! 3. **To Prompt** - Generate XML blocks for embedding skill information in agent prompts
//!
//! ## Example
//!
//! ```no_run
//! use skills_ref::{read_properties, validate, to_prompt};
//! use std::path::Path;
//!
//! // Validate a skill directory
//! let errors = validate(Path::new("my-skill"));
//! if errors.is_empty() {
//!     println!("Skill is valid!");
//! }
//!
//! // Read skill properties
//! let props = read_properties(Path::new("my-skill")).unwrap();
//! println!("Skill name: {}", props.name);
//!
//! // Generate XML prompt
//! let xml = to_prompt(&[Path::new("my-skill")]).unwrap();
//! println!("{}", xml);
//! ```

pub mod error;
pub mod models;
pub mod parser;
pub mod prompt;
pub mod validator;

// Re-export main types and functions for convenience
pub use error::{Result, SkillError};
pub use models::SkillProperties;
pub use parser::{find_skill_md, parse_frontmatter, read_properties};
pub use prompt::to_prompt;
pub use validator::{validate, validate_metadata};
