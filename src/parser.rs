//! YAML frontmatter parsing for SKILL.md files.

use std::collections::HashMap;
use std::path::Path;

use crate::error::{Result, SkillError};
use crate::models::SkillProperties;

/// Find the SKILL.md file in a skill directory.
///
/// Prefers SKILL.md (uppercase) but accepts skill.md (lowercase).
///
/// # Arguments
///
/// * `skill_dir` - Path to the skill directory
///
/// # Returns
///
/// Path to the SKILL.md file, or None if not found.
pub fn find_skill_md(skill_dir: &Path) -> Option<std::path::PathBuf> {
    for name in ["SKILL.md", "skill.md"] {
        let path = skill_dir.join(name);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

/// Parse YAML frontmatter from SKILL.md content.
///
/// # Arguments
///
/// * `content` - Raw content of SKILL.md file
///
/// # Returns
///
/// Tuple of (metadata dict, markdown body)
///
/// # Errors
///
/// Returns `ParseError` if frontmatter is missing or invalid.
pub fn parse_frontmatter(content: &str) -> Result<(HashMap<String, serde_yaml::Value>, String)> {
    if !content.starts_with("---") {
        return Err(SkillError::parse(
            "SKILL.md must start with YAML frontmatter (---)",
        ));
    }

    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(SkillError::parse(
            "SKILL.md frontmatter not properly closed with ---",
        ));
    }

    let frontmatter_str = parts[1];
    let body = parts[2].trim().to_string();

    let metadata: HashMap<String, serde_yaml::Value> = serde_yaml::from_str(frontmatter_str)
        .map_err(|e| SkillError::parse(format!("Invalid YAML in frontmatter: {}", e)))?;

    Ok((metadata, body))
}

/// Read skill properties from SKILL.md frontmatter.
///
/// This function parses the frontmatter and returns properties.
/// It does NOT perform full validation. Use `validate()` for that.
///
/// # Arguments
///
/// * `skill_dir` - Path to the skill directory
///
/// # Returns
///
/// SkillProperties with parsed metadata
///
/// # Errors
///
/// * `ParseError` - If SKILL.md is missing or has invalid YAML
/// * `ValidationError` - If required fields (name, description) are missing
pub fn read_properties(skill_dir: &Path) -> Result<SkillProperties> {
    let skill_md = find_skill_md(skill_dir).ok_or_else(|| {
        SkillError::parse(format!("SKILL.md not found in {}", skill_dir.display()))
    })?;

    let content = std::fs::read_to_string(&skill_md)?;
    let (metadata, _) = parse_frontmatter(&content)?;

    // Check required fields
    if !metadata.contains_key("name") {
        return Err(SkillError::validation(
            "Missing required field in frontmatter: name",
        ));
    }
    if !metadata.contains_key("description") {
        return Err(SkillError::validation(
            "Missing required field in frontmatter: description",
        ));
    }

    // Extract and validate name
    let name = extract_string(&metadata, "name")
        .ok_or_else(|| SkillError::validation("Field 'name' must be a non-empty string"))?;
    if name.trim().is_empty() {
        return Err(SkillError::validation(
            "Field 'name' must be a non-empty string",
        ));
    }

    // Extract and validate description
    let description = extract_string(&metadata, "description")
        .ok_or_else(|| SkillError::validation("Field 'description' must be a non-empty string"))?;
    if description.trim().is_empty() {
        return Err(SkillError::validation(
            "Field 'description' must be a non-empty string",
        ));
    }

    // Extract optional fields
    let license = extract_string(&metadata, "license");
    let compatibility = extract_string(&metadata, "compatibility");
    let allowed_tools = extract_string(&metadata, "allowed-tools");

    // Extract metadata field
    let skill_metadata = extract_metadata(&metadata);

    Ok(SkillProperties {
        name: name.trim().to_string(),
        description: description.trim().to_string(),
        license,
        compatibility,
        allowed_tools,
        metadata: skill_metadata,
    })
}

/// Extract a string value from a YAML mapping.
fn extract_string(metadata: &HashMap<String, serde_yaml::Value>, key: &str) -> Option<String> {
    metadata.get(key).and_then(|v| match v {
        serde_yaml::Value::String(s) => Some(s.clone()),
        _ => None,
    })
}

/// Extract the metadata field as a HashMap<String, String>.
fn extract_metadata(
    metadata: &HashMap<String, serde_yaml::Value>,
) -> Option<HashMap<String, String>> {
    metadata.get("metadata").and_then(|v| match v {
        serde_yaml::Value::Mapping(m) => {
            let map: HashMap<String, String> = m
                .iter()
                .filter_map(|(k, v)| {
                    let key = match k {
                        serde_yaml::Value::String(s) => s.clone(),
                        _ => k.as_str()?.to_string(),
                    };
                    let value = match v {
                        serde_yaml::Value::String(s) => s.clone(),
                        _ => format!("{:?}", v),
                    };
                    Some((key, value))
                })
                .collect();
            if map.is_empty() { None } else { Some(map) }
        }
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_skill_dir(dir: &TempDir, name: &str, content: &str) -> std::path::PathBuf {
        let skill_dir = dir.path().join(name);
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(skill_dir.join("SKILL.md"), content).unwrap();
        skill_dir
    }

    #[test]
    fn test_find_skill_md_uppercase() {
        let dir = TempDir::new().unwrap();
        let skill_dir = dir.path().join("my-skill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(skill_dir.join("SKILL.md"), "test").unwrap();

        let result = find_skill_md(&skill_dir);
        assert!(result.is_some());
        assert!(result.unwrap().ends_with("SKILL.md"));
    }

    #[test]
    fn test_find_skill_md_lowercase() {
        let dir = TempDir::new().unwrap();
        let skill_dir = dir.path().join("my-skill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(skill_dir.join("skill.md"), "test").unwrap();

        let result = find_skill_md(&skill_dir);
        assert!(result.is_some());
        // On case-insensitive filesystems (macOS), this might return SKILL.md
        let path = result.unwrap();
        let filename = path.file_name().unwrap().to_str().unwrap().to_lowercase();
        assert_eq!(filename, "skill.md");
    }

    #[test]
    fn test_find_skill_md_prefers_uppercase() {
        let dir = TempDir::new().unwrap();
        let skill_dir = dir.path().join("my-skill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(skill_dir.join("SKILL.md"), "test").unwrap();
        std::fs::write(skill_dir.join("skill.md"), "test").unwrap();

        let result = find_skill_md(&skill_dir);
        assert!(result.is_some());
        assert!(result.unwrap().ends_with("SKILL.md"));
    }

    #[test]
    fn test_find_skill_md_not_found() {
        let dir = TempDir::new().unwrap();
        let skill_dir = dir.path().join("my-skill");
        std::fs::create_dir_all(&skill_dir).unwrap();

        let result = find_skill_md(&skill_dir);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_frontmatter_valid() {
        let content = r#"---
name: my-skill
description: A test skill
---
# Body
"#;
        let (metadata, body) = parse_frontmatter(content).unwrap();
        assert_eq!(metadata.get("name").unwrap().as_str().unwrap(), "my-skill");
        assert_eq!(
            metadata.get("description").unwrap().as_str().unwrap(),
            "A test skill"
        );
        assert_eq!(body, "# Body");
    }

    #[test]
    fn test_parse_frontmatter_missing_start() {
        let content = "name: my-skill\n---\n# Body";
        let result = parse_frontmatter(content);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("must start with YAML frontmatter")
        );
    }

    #[test]
    fn test_parse_frontmatter_unclosed() {
        let content = "---\nname: my-skill\n# Body";
        let result = parse_frontmatter(content);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("not properly closed")
        );
    }

    #[test]
    fn test_read_properties_valid() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill_dir(
            &dir,
            "my-skill",
            r#"---
name: my-skill
description: A test skill
license: MIT
---
# Body
"#,
        );

        let props = read_properties(&skill_dir).unwrap();
        assert_eq!(props.name, "my-skill");
        assert_eq!(props.description, "A test skill");
        assert_eq!(props.license, Some("MIT".to_string()));
    }

    #[test]
    fn test_read_properties_with_metadata() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill_dir(
            &dir,
            "my-skill",
            r#"---
name: my-skill
description: A test skill
metadata:
  author: Test
  version: "1.0"
---
# Body
"#,
        );

        let props = read_properties(&skill_dir).unwrap();
        let metadata = props.metadata.unwrap();
        assert_eq!(metadata.get("author").unwrap(), "Test");
        assert_eq!(metadata.get("version").unwrap(), "1.0");
    }

    #[test]
    fn test_read_properties_missing_name() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill_dir(
            &dir,
            "my-skill",
            r#"---
description: A test skill
---
# Body
"#,
        );

        let result = read_properties(&skill_dir);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("name"));
    }

    #[test]
    fn test_read_properties_missing_description() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill_dir(
            &dir,
            "my-skill",
            r#"---
name: my-skill
---
# Body
"#,
        );

        let result = read_properties(&skill_dir);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("description"));
    }
}
