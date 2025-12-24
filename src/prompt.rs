//! Generate `<available_skills>` XML prompt block for agent system prompts.

use std::path::Path;

use crate::error::Result;
use crate::parser::{find_skill_md, read_properties};

/// Escape special HTML characters.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Generate the `<available_skills>` XML block for inclusion in agent prompts.
///
/// This XML format is what Anthropic uses and recommends for Claude models.
/// Skill Clients may format skill information differently to suit their
/// models or preferences.
///
/// # Arguments
///
/// * `skill_dirs` - List of paths to skill directories
///
/// # Returns
///
/// XML string with `<available_skills>` block containing each skill's
/// name, description, and location.
///
/// # Example output
///
/// ```xml
/// <available_skills>
/// <skill>
/// <name>pdf-reader</name>
/// <description>Read and extract text from PDF files</description>
/// <location>/path/to/pdf-reader/SKILL.md</location>
/// </skill>
/// </available_skills>
/// ```
pub fn to_prompt(skill_dirs: &[&Path]) -> Result<String> {
    if skill_dirs.is_empty() {
        return Ok("<available_skills>\n</available_skills>".to_string());
    }

    let mut lines = vec!["<available_skills>".to_string()];

    for skill_dir in skill_dirs {
        let skill_dir = skill_dir
            .canonicalize()
            .unwrap_or_else(|_| skill_dir.to_path_buf());
        let props = read_properties(&skill_dir)?;

        lines.push("<skill>".to_string());
        lines.push("<name>".to_string());
        lines.push(html_escape(&props.name));
        lines.push("</name>".to_string());
        lines.push("<description>".to_string());
        lines.push(html_escape(&props.description));
        lines.push("</description>".to_string());

        if let Some(skill_md_path) = find_skill_md(&skill_dir) {
            lines.push("<location>".to_string());
            lines.push(skill_md_path.to_string_lossy().to_string());
            lines.push("</location>".to_string());
        }

        lines.push("</skill>".to_string());
    }

    lines.push("</available_skills>".to_string());

    Ok(lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_skill(dir: &TempDir, name: &str, description: &str) -> std::path::PathBuf {
        let skill_dir = dir.path().join(name);
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(
            skill_dir.join("SKILL.md"),
            format!(
                r#"---
name: {}
description: {}
---
# {}
"#,
                name, description, name
            ),
        )
        .unwrap();
        skill_dir
    }

    #[test]
    fn test_empty_skill_list() {
        let result = to_prompt(&[]).unwrap();
        assert_eq!(result, "<available_skills>\n</available_skills>");
    }

    #[test]
    fn test_single_skill() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill(&dir, "my-skill", "A test skill");

        let result = to_prompt(&[skill_dir.as_path()]).unwrap();

        assert!(result.contains("<available_skills>"));
        assert!(result.contains("</available_skills>"));
        assert!(result.contains("<skill>"));
        assert!(result.contains("</skill>"));
        assert!(result.contains("<name>"));
        assert!(result.contains("my-skill"));
        assert!(result.contains("</name>"));
        assert!(result.contains("<description>"));
        assert!(result.contains("A test skill"));
        assert!(result.contains("</description>"));
        assert!(result.contains("<location>"));
        assert!(result.contains("SKILL.md"));
        assert!(result.contains("</location>"));
    }

    #[test]
    fn test_multiple_skills() {
        let dir = TempDir::new().unwrap();
        let skill1 = create_skill(&dir, "skill-one", "First skill");
        let skill2 = create_skill(&dir, "skill-two", "Second skill");

        let result = to_prompt(&[skill1.as_path(), skill2.as_path()]).unwrap();

        assert!(result.contains("skill-one"));
        assert!(result.contains("First skill"));
        assert!(result.contains("skill-two"));
        assert!(result.contains("Second skill"));

        // Count skill elements
        let skill_count = result.matches("<skill>").count();
        assert_eq!(skill_count, 2);
    }

    #[test]
    fn test_html_escaping() {
        let dir = TempDir::new().unwrap();
        let skill_dir = dir.path().join("test-skill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(
            skill_dir.join("SKILL.md"),
            r#"---
name: test-skill
description: A skill with <special> & "characters"
---
# Test
"#,
        )
        .unwrap();

        let result = to_prompt(&[skill_dir.as_path()]).unwrap();

        assert!(result.contains("&lt;special&gt;"));
        assert!(result.contains("&amp;"));
        assert!(result.contains("&quot;characters&quot;"));
    }

    #[test]
    fn test_output_format() {
        let dir = TempDir::new().unwrap();
        let skill_dir = create_skill(&dir, "my-skill", "A test skill");

        let result = to_prompt(&[skill_dir.as_path()]).unwrap();
        let lines: Vec<&str> = result.lines().collect();

        // Verify structure: each element on its own line
        assert_eq!(lines[0], "<available_skills>");
        assert_eq!(lines[1], "<skill>");
        assert_eq!(lines[2], "<name>");
        assert_eq!(lines[3], "my-skill");
        assert_eq!(lines[4], "</name>");
    }
}
