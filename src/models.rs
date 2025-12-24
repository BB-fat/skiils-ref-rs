//! Data models for Agent Skills.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Properties parsed from a skill's SKILL.md frontmatter.
///
/// # Fields
///
/// * `name` - Skill name in kebab-case (required)
/// * `description` - What the skill does and when the model should use it (required)
/// * `license` - License for the skill (optional)
/// * `compatibility` - Compatibility information for the skill (optional)
/// * `allowed_tools` - Tool patterns the skill requires (optional, experimental)
/// * `metadata` - Key-value pairs for client-specific properties (optional)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillProperties {
    /// Skill name in kebab-case (required).
    pub name: String,

    /// What the skill does and when the model should use it (required).
    pub description: String,

    /// License for the skill (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,

    /// Compatibility information for the skill (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<String>,

    /// Tool patterns the skill requires (optional, experimental).
    #[serde(rename = "allowed-tools", skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<String>,

    /// Key-value pairs for client-specific properties (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl SkillProperties {
    /// Create a new SkillProperties with required fields only.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            license: None,
            compatibility: None,
            allowed_tools: None,
            metadata: None,
        }
    }

    /// Convert to a dictionary (HashMap), excluding None values.
    ///
    /// This matches the Python `to_dict()` method behavior.
    pub fn to_dict(&self) -> HashMap<String, serde_json::Value> {
        let mut result = HashMap::new();

        result.insert(
            "name".to_string(),
            serde_json::Value::String(self.name.clone()),
        );
        result.insert(
            "description".to_string(),
            serde_json::Value::String(self.description.clone()),
        );

        if let Some(ref license) = self.license {
            result.insert(
                "license".to_string(),
                serde_json::Value::String(license.clone()),
            );
        }

        if let Some(ref compatibility) = self.compatibility {
            result.insert(
                "compatibility".to_string(),
                serde_json::Value::String(compatibility.clone()),
            );
        }

        if let Some(ref allowed_tools) = self.allowed_tools {
            result.insert(
                "allowed-tools".to_string(),
                serde_json::Value::String(allowed_tools.clone()),
            );
        }

        if let Some(ref metadata) = self.metadata {
            let meta_map: serde_json::Map<String, serde_json::Value> = metadata
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            result.insert("metadata".to_string(), serde_json::Value::Object(meta_map));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_skill_properties() {
        let props = SkillProperties::new("my-skill", "A test skill");
        assert_eq!(props.name, "my-skill");
        assert_eq!(props.description, "A test skill");
        assert!(props.license.is_none());
        assert!(props.metadata.is_none());
    }

    #[test]
    fn test_to_dict_minimal() {
        let props = SkillProperties::new("my-skill", "A test skill");
        let dict = props.to_dict();

        assert_eq!(dict.len(), 2);
        assert_eq!(dict.get("name").unwrap(), "my-skill");
        assert_eq!(dict.get("description").unwrap(), "A test skill");
    }

    #[test]
    fn test_to_dict_with_optional_fields() {
        let mut props = SkillProperties::new("my-skill", "A test skill");
        props.license = Some("MIT".to_string());
        props.compatibility = Some("Python 3.11+".to_string());
        props.allowed_tools = Some("Bash(git:*)".to_string());

        let mut metadata = HashMap::new();
        metadata.insert("author".to_string(), "Test".to_string());
        props.metadata = Some(metadata);

        let dict = props.to_dict();

        assert_eq!(dict.len(), 6);
        assert_eq!(dict.get("license").unwrap(), "MIT");
        assert_eq!(dict.get("compatibility").unwrap(), "Python 3.11+");
        assert_eq!(dict.get("allowed-tools").unwrap(), "Bash(git:*)");
    }

    #[test]
    fn test_json_serialization() {
        let props = SkillProperties::new("my-skill", "A test skill");
        let json = serde_json::to_string(&props).unwrap();

        // Should not contain optional fields when None
        assert!(!json.contains("license"));
        assert!(!json.contains("metadata"));
    }
}
