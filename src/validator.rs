//! Skill validation logic.

use std::collections::HashMap;
use std::path::Path;

use unicode_normalization::UnicodeNormalization;

use crate::parser::{find_skill_md, parse_frontmatter};

/// Maximum length for skill names.
pub const MAX_SKILL_NAME_LENGTH: usize = 64;

/// Maximum length for skill descriptions.
pub const MAX_DESCRIPTION_LENGTH: usize = 1024;

/// Maximum length for compatibility field.
pub const MAX_COMPATIBILITY_LENGTH: usize = 500;

/// Allowed frontmatter fields per Agent Skills Spec.
const ALLOWED_FIELDS: &[&str] = &[
    "name",
    "description",
    "license",
    "allowed-tools",
    "metadata",
    "compatibility",
];

/// Check if a field is allowed.
fn is_allowed_field(field: &str) -> bool {
    ALLOWED_FIELDS.contains(&field)
}

/// Validate skill name format and directory match.
///
/// Skill names support i18n characters (Unicode letters) plus hyphens.
/// Names must be lowercase and cannot start/end with hyphens.
fn validate_name(name: &str, skill_dir: Option<&Path>) -> Vec<String> {
    let mut errors = Vec::new();

    if name.is_empty() || name.trim().is_empty() {
        errors.push("Field 'name' must be a non-empty string".to_string());
        return errors;
    }

    // NFKC normalize the name
    let name = name.trim().nfkc().collect::<String>();

    // Check length
    if name.chars().count() > MAX_SKILL_NAME_LENGTH {
        errors.push(format!(
            "Skill name '{}' exceeds {} character limit ({} chars)",
            name,
            MAX_SKILL_NAME_LENGTH,
            name.chars().count()
        ));
    }

    // Check lowercase
    if name != name.to_lowercase() {
        errors.push(format!("Skill name '{}' must be lowercase", name));
    }

    // Check leading/trailing hyphens
    if name.starts_with('-') || name.ends_with('-') {
        errors.push("Skill name cannot start or end with a hyphen".to_string());
    }

    // Check consecutive hyphens
    if name.contains("--") {
        errors.push("Skill name cannot contain consecutive hyphens".to_string());
    }

    // Check valid characters (alphanumeric or hyphen)
    if !name.chars().all(|c| c.is_alphanumeric() || c == '-') {
        errors.push(format!(
            "Skill name '{}' contains invalid characters. Only letters, digits, and hyphens are allowed.",
            name
        ));
    }

    // Check directory name match
    if let Some(dir) = skill_dir
        && let Some(dir_name) = dir.file_name().and_then(|n| n.to_str())
    {
        let normalized_dir_name = dir_name.nfkc().collect::<String>();
        if normalized_dir_name != name {
            errors.push(format!(
                "Directory name '{}' must match skill name '{}'",
                dir_name, name
            ));
        }
    }

    errors
}

/// Validate description format.
fn validate_description(description: &str) -> Vec<String> {
    let mut errors = Vec::new();

    if description.is_empty() || description.trim().is_empty() {
        errors.push("Field 'description' must be a non-empty string".to_string());
        return errors;
    }

    if description.len() > MAX_DESCRIPTION_LENGTH {
        errors.push(format!(
            "Description exceeds {} character limit ({} chars)",
            MAX_DESCRIPTION_LENGTH,
            description.len()
        ));
    }

    errors
}

/// Validate compatibility format.
fn validate_compatibility(compatibility: &str) -> Vec<String> {
    let mut errors = Vec::new();

    if compatibility.len() > MAX_COMPATIBILITY_LENGTH {
        errors.push(format!(
            "Compatibility exceeds {} character limit ({} chars)",
            MAX_COMPATIBILITY_LENGTH,
            compatibility.len()
        ));
    }

    errors
}

/// Validate that only allowed fields are present.
fn validate_metadata_fields(metadata: &HashMap<String, serde_yaml::Value>) -> Vec<String> {
    let mut errors = Vec::new();

    let extra_fields: Vec<_> = metadata
        .keys()
        .filter(|k| !is_allowed_field(k.as_str()))
        .collect();

    if !extra_fields.is_empty() {
        let mut sorted_extra: Vec<_> = extra_fields.iter().map(|s| s.as_str()).collect();
        sorted_extra.sort();
        let mut sorted_allowed: Vec<_> = ALLOWED_FIELDS.to_vec();
        sorted_allowed.sort();
        errors.push(format!(
            "Unexpected fields in frontmatter: {}. Only {:?} are allowed.",
            sorted_extra.join(", "),
            sorted_allowed
        ));
    }

    errors
}

/// Validate parsed skill metadata.
///
/// This is the core validation function that works on already-parsed metadata,
/// avoiding duplicate file I/O when called from the parser.
///
/// # Arguments
///
/// * `metadata` - Parsed YAML frontmatter dictionary
/// * `skill_dir` - Optional path to skill directory (for name-directory match check)
///
/// # Returns
///
/// List of validation error messages. Empty list means valid.
pub fn validate_metadata(
    metadata: &HashMap<String, serde_yaml::Value>,
    skill_dir: Option<&Path>,
) -> Vec<String> {
    let mut errors = Vec::new();

    // Check for unexpected fields
    errors.extend(validate_metadata_fields(metadata));

    // Validate name
    if !metadata.contains_key("name") {
        errors.push("Missing required field in frontmatter: name".to_string());
    } else if let Some(name) = metadata.get("name").and_then(|v| v.as_str()) {
        errors.extend(validate_name(name, skill_dir));
    } else {
        errors.push("Field 'name' must be a non-empty string".to_string());
    }

    // Validate description
    if !metadata.contains_key("description") {
        errors.push("Missing required field in frontmatter: description".to_string());
    } else if let Some(desc) = metadata.get("description").and_then(|v| v.as_str()) {
        errors.extend(validate_description(desc));
    } else {
        errors.push("Field 'description' must be a non-empty string".to_string());
    }

    // Validate compatibility if present
    if let Some(compat) = metadata.get("compatibility").and_then(|v| v.as_str()) {
        errors.extend(validate_compatibility(compat));
    }

    errors
}

/// Validate a skill directory.
///
/// # Arguments
///
/// * `skill_dir` - Path to the skill directory
///
/// # Returns
///
/// List of validation error messages. Empty list means valid.
pub fn validate(skill_dir: &Path) -> Vec<String> {
    // Check path exists
    if !skill_dir.exists() {
        return vec![format!("Path does not exist: {}", skill_dir.display())];
    }

    // Check it's a directory
    if !skill_dir.is_dir() {
        return vec![format!("Not a directory: {}", skill_dir.display())];
    }

    // Find SKILL.md
    let skill_md = match find_skill_md(skill_dir) {
        Some(path) => path,
        None => return vec!["Missing required file: SKILL.md".to_string()],
    };

    // Read and parse content
    let content = match std::fs::read_to_string(&skill_md) {
        Ok(c) => c,
        Err(e) => return vec![format!("Failed to read {}: {}", skill_md.display(), e)],
    };

    let metadata = match parse_frontmatter(&content) {
        Ok((m, _)) => m,
        Err(e) => return vec![e.to_string()],
    };

    validate_metadata(&metadata, Some(skill_dir))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_skill(dir: &TempDir, name: &str, content: &str) -> std::path::PathBuf {
        let skill_dir = dir.path().join(name);
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(skill_dir.join("SKILL.md"), content).unwrap();
        skill_dir
    }

    #[test]
    fn test_valid_skill() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill(
            &dir,
            "my-skill",
            r#"---
name: my-skill
description: A test skill
---
# My Skill
"#,
        );
        let errors = validate(&skill_dir);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_nonexistent_path() {
        let dir = TempDir::new().unwrap();
        let errors = validate(&dir.path().join("nonexistent"));
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("does not exist"));
    }

    #[test]
    fn test_not_a_directory() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("file.txt");
        std::fs::write(&file_path, "test").unwrap();
        let errors = validate(&file_path);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("Not a directory"));
    }

    #[test]
    fn test_missing_skill_md() {
        let dir = TempDir::new().unwrap();
        let skill_dir = dir.path().join("my-skill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        let errors = validate(&skill_dir);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("Missing required file: SKILL.md"));
    }

    #[test]
    fn test_invalid_name_uppercase() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill(
            &dir,
            "MySkill",
            r#"---
name: MySkill
description: A test skill
---
Body
"#,
        );
        let errors = validate(&skill_dir);
        assert!(errors.iter().any(|e| e.contains("lowercase")));
    }

    #[test]
    fn test_name_too_long() {
        let dir = TempDir::new().unwrap();
        let long_name = "a".repeat(70);
        let skill_dir = create_skill(
            &dir,
            &long_name,
            &format!(
                r#"---
name: {}
description: A test skill
---
Body
"#,
                long_name
            ),
        );
        let errors = validate(&skill_dir);
        assert!(
            errors
                .iter()
                .any(|e| e.contains("exceeds") && e.contains("character limit"))
        );
    }

    #[test]
    fn test_name_leading_hyphen() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill(
            &dir,
            "-my-skill",
            r#"---
name: -my-skill
description: A test skill
---
Body
"#,
        );
        let errors = validate(&skill_dir);
        assert!(
            errors
                .iter()
                .any(|e| e.contains("cannot start or end with a hyphen"))
        );
    }

    #[test]
    fn test_name_consecutive_hyphens() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill(
            &dir,
            "my--skill",
            r#"---
name: my--skill
description: A test skill
---
Body
"#,
        );
        let errors = validate(&skill_dir);
        assert!(errors.iter().any(|e| e.contains("consecutive hyphens")));
    }

    #[test]
    fn test_name_invalid_characters() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill(
            &dir,
            "my_skill",
            r#"---
name: my_skill
description: A test skill
---
Body
"#,
        );
        let errors = validate(&skill_dir);
        assert!(errors.iter().any(|e| e.contains("invalid characters")));
    }

    #[test]
    fn test_name_directory_mismatch() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill(
            &dir,
            "wrong-name",
            r#"---
name: correct-name
description: A test skill
---
Body
"#,
        );
        let errors = validate(&skill_dir);
        assert!(errors.iter().any(|e| e.contains("must match skill name")));
    }

    #[test]
    fn test_unexpected_fields() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill(
            &dir,
            "my-skill",
            r#"---
name: my-skill
description: A test skill
unknown_field: should not be here
---
Body
"#,
        );
        let errors = validate(&skill_dir);
        assert!(errors.iter().any(|e| e.contains("Unexpected fields")));
    }

    #[test]
    fn test_valid_with_all_fields() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill(
            &dir,
            "my-skill",
            r#"---
name: my-skill
description: A test skill
license: MIT
metadata:
  author: Test
---
Body
"#,
        );
        let errors = validate(&skill_dir);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_allowed_tools_accepted() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill(
            &dir,
            "my-skill",
            r#"---
name: my-skill
description: A test skill
allowed-tools: Bash(jq:*) Bash(git:*)
---
Body
"#,
        );
        let errors = validate(&skill_dir);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_i18n_chinese_name() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill(
            &dir,
            "技能",
            r#"---
name: 技能
description: A skill with Chinese name
---
Body
"#,
        );
        let errors = validate(&skill_dir);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_i18n_russian_name_with_hyphens() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill(
            &dir,
            "мой-навык",
            r#"---
name: мой-навык
description: A skill with Russian name
---
Body
"#,
        );
        let errors = validate(&skill_dir);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_i18n_russian_lowercase_valid() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill(
            &dir,
            "навык",
            r#"---
name: навык
description: A skill with Russian lowercase name
---
Body
"#,
        );
        let errors = validate(&skill_dir);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_i18n_russian_uppercase_rejected() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill(
            &dir,
            "НАВЫК",
            r#"---
name: НАВЫК
description: A skill with Russian uppercase name
---
Body
"#,
        );
        let errors = validate(&skill_dir);
        assert!(errors.iter().any(|e| e.contains("lowercase")));
    }

    #[test]
    fn test_description_too_long() {
        let dir = TempDir::new().unwrap();
        let long_desc = "x".repeat(1100);
        let skill_dir = create_skill(
            &dir,
            "my-skill",
            &format!(
                r#"---
name: my-skill
description: {}
---
Body
"#,
                long_desc
            ),
        );
        let errors = validate(&skill_dir);
        assert!(
            errors
                .iter()
                .any(|e| e.contains("exceeds") && e.contains("1024"))
        );
    }

    #[test]
    fn test_valid_compatibility() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill(
            &dir,
            "my-skill",
            r#"---
name: my-skill
description: A test skill
compatibility: Requires Python 3.11+
---
Body
"#,
        );
        let errors = validate(&skill_dir);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_compatibility_too_long() {
        let dir = TempDir::new().unwrap();
        let long_compat = "x".repeat(550);
        let skill_dir = create_skill(
            &dir,
            "my-skill",
            &format!(
                r#"---
name: my-skill
description: A test skill
compatibility: {}
---
Body
"#,
                long_compat
            ),
        );
        let errors = validate(&skill_dir);
        assert!(
            errors
                .iter()
                .any(|e| e.contains("exceeds") && e.contains("500"))
        );
    }

    #[test]
    fn test_nfkc_normalization() {
        let dir = TempDir::new().unwrap();
        // Use decomposed form: 'cafe' + combining acute accent (U+0301)
        let decomposed_name = "cafe\u{0301}"; // 'café' with combining accent
        let composed_name = "café"; // precomposed form

        let skill_dir = dir.path().join(composed_name);
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(
            skill_dir.join("SKILL.md"),
            format!(
                r#"---
name: {}
description: A test skill
---
Body
"#,
                decomposed_name
            ),
        )
        .unwrap();

        let errors = validate(&skill_dir);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }
}
