//! Integration tests using the pdf/ skill directory.

use std::path::Path;

use skills_ref::{read_properties, to_prompt, validate};

/// Path to the pdf skill directory.
fn pdf_skill_dir() -> &'static Path {
    Path::new("pdf")
}

#[test]
fn test_validate_pdf_skill() {
    let errors = validate(pdf_skill_dir());
    assert!(errors.is_empty());
}

#[test]
fn test_read_properties_pdf_skill() {
    let props = read_properties(pdf_skill_dir()).unwrap();

    assert_eq!(props.name, "pdf");
    assert!(props.description.contains("PDF"));
    assert!(props.license.is_some());
    assert!(props.license.as_ref().unwrap().contains("Proprietary"));
}

#[test]
fn test_to_prompt_pdf_skill() {
    let output = to_prompt(&[pdf_skill_dir()]).unwrap();

    assert!(output.contains("<available_skills>"));
    assert!(output.contains("</available_skills>"));
    assert!(output.contains("<skill>"));
    assert!(output.contains("<name>"));
    assert!(output.contains("pdf"));
    assert!(output.contains("</name>"));
    assert!(output.contains("<description>"));
    assert!(output.contains("</description>"));
    assert!(output.contains("<location>"));
    assert!(output.contains("SKILL.md"));
    assert!(output.contains("</location>"));
}

#[test]
fn test_to_prompt_empty_list() {
    let output = to_prompt(&[]).unwrap();
    assert_eq!(output, "<available_skills>\n</available_skills>");
}

#[test]
fn test_json_serialization() {
    let props = read_properties(pdf_skill_dir()).unwrap();
    let json = serde_json::to_string_pretty(&props).unwrap();

    // Verify JSON format
    assert!(json.contains("\"name\""));
    assert!(json.contains("\"description\""));
    assert!(json.contains("\"license\""));

    // Verify it's valid JSON that can be parsed back
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["name"], "pdf");
}
