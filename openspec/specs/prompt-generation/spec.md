# prompt-generation Specification

## Purpose
TBD - created by archiving change implement-skills-ref-rs. Update Purpose after archive.
## Requirements
### Requirement: Available Skills XML Generation
The system SHALL generate an `<available_skills>` XML block for agent system prompts.

#### Scenario: Single skill directory
- **WHEN** `to_prompt()` is called with one skill directory
- **THEN** the output SHALL contain an `<available_skills>` block with one `<skill>` element
- **AND** the `<skill>` element SHALL contain `<name>`, `<description>`, and `<location>` elements

#### Scenario: Multiple skill directories
- **WHEN** `to_prompt()` is called with multiple skill directories
- **THEN** the output SHALL contain one `<skill>` element per directory in order

#### Scenario: Empty skill list
- **WHEN** `to_prompt()` is called with an empty list
- **THEN** the output SHALL be `<available_skills>\n</available_skills>`

#### Scenario: HTML escaping applied
- **WHEN** the skill name or description contains HTML special characters (`<`, `>`, `&`, `"`, `'`)
- **THEN** the system SHALL escape these characters in the output

#### Scenario: Location uses absolute path
- **WHEN** generating the `<location>` element
- **THEN** the path SHALL be the resolved absolute path to the SKILL.md file

### Requirement: XML Output Format
The system SHALL produce XML in the expected format.

#### Scenario: Correct element structure
- **WHEN** generating XML for a skill
- **THEN** the output SHALL match this structure:
```xml
<available_skills>
<skill>
<name>
skill-name
</name>
<description>
Skill description text
</description>
<location>
/path/to/skill/SKILL.md
</location>
</skill>
</available_skills>
```

#### Scenario: Newline separated elements
- **WHEN** generating XML output
- **THEN** each element (opening tag, content, closing tag) SHALL be on its own line

