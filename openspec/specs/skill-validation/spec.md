# skill-validation Specification

## Purpose
TBD - created by archiving change implement-skills-ref-rs. Update Purpose after archive.
## Requirements
### Requirement: Skill Name Validation
The system SHALL validate that skill names follow the required format.

#### Scenario: Valid lowercase kebab-case name
- **WHEN** the name is `my-skill` (lowercase, kebab-case)
- **THEN** validation SHALL pass with no errors

#### Scenario: Uppercase rejected
- **WHEN** the name is `MySkill` (contains uppercase)
- **THEN** validation SHALL return an error containing "lowercase"

#### Scenario: Name length limit
- **WHEN** the name exceeds 64 characters
- **THEN** validation SHALL return an error containing "exceeds" and "character limit"

#### Scenario: Leading hyphen rejected
- **WHEN** the name starts with a hyphen (`-my-skill`)
- **THEN** validation SHALL return an error containing "cannot start or end with a hyphen"

#### Scenario: Trailing hyphen rejected
- **WHEN** the name ends with a hyphen (`my-skill-`)
- **THEN** validation SHALL return an error containing "cannot start or end with a hyphen"

#### Scenario: Consecutive hyphens rejected
- **WHEN** the name contains consecutive hyphens (`my--skill`)
- **THEN** validation SHALL return an error containing "consecutive hyphens"

#### Scenario: Invalid characters rejected
- **WHEN** the name contains characters other than letters, digits, and hyphens (`my_skill`)
- **THEN** validation SHALL return an error containing "invalid characters"

#### Scenario: Directory name mismatch
- **WHEN** the skill directory name does not match the skill name
- **THEN** validation SHALL return an error containing "must match skill name"

#### Scenario: Unicode NFKC normalization
- **WHEN** the name uses decomposed Unicode (e.g., `cafe\u0301`)
- **THEN** the system SHALL normalize to NFKC before comparison
- **AND** the normalized form SHALL match the directory name

### Requirement: Internationalization Support
The system SHALL support Unicode characters in skill names.

#### Scenario: Chinese characters accepted
- **WHEN** the name uses Chinese characters (e.g., `技能`)
- **THEN** validation SHALL pass with no errors

#### Scenario: Russian lowercase accepted
- **WHEN** the name uses Russian lowercase letters (e.g., `навык`)
- **THEN** validation SHALL pass with no errors

#### Scenario: Russian uppercase rejected
- **WHEN** the name uses Russian uppercase letters (e.g., `НАВЫК`)
- **THEN** validation SHALL return an error containing "lowercase"

#### Scenario: Russian with hyphens accepted
- **WHEN** the name uses Russian letters with hyphens (e.g., `мой-навык`)
- **THEN** validation SHALL pass with no errors

### Requirement: Description Validation
The system SHALL validate that skill descriptions meet requirements.

#### Scenario: Valid description
- **WHEN** the description is a non-empty string under 1024 characters
- **THEN** validation SHALL pass with no errors

#### Scenario: Empty description rejected
- **WHEN** the description is empty or whitespace-only
- **THEN** validation SHALL return an error

#### Scenario: Description length limit
- **WHEN** the description exceeds 1024 characters
- **THEN** validation SHALL return an error containing "exceeds" and "1024"

### Requirement: Compatibility Validation
The system SHALL validate the optional compatibility field.

#### Scenario: Valid compatibility
- **WHEN** the compatibility field is a string under 500 characters
- **THEN** validation SHALL pass with no errors

#### Scenario: Compatibility length limit
- **WHEN** the compatibility field exceeds 500 characters
- **THEN** validation SHALL return an error containing "exceeds" and "500"

### Requirement: Allowed Fields Validation
The system SHALL reject unexpected frontmatter fields.

#### Scenario: Only allowed fields accepted
- **WHEN** the frontmatter contains only allowed fields (name, description, license, allowed-tools, metadata, compatibility)
- **THEN** validation SHALL pass with no errors

#### Scenario: Unexpected fields rejected
- **WHEN** the frontmatter contains an unknown field
- **THEN** validation SHALL return an error containing "Unexpected fields"

### Requirement: Directory Validation
The system SHALL validate the skill directory itself.

#### Scenario: Nonexistent path
- **WHEN** the provided path does not exist
- **THEN** validation SHALL return an error containing "does not exist"

#### Scenario: Not a directory
- **WHEN** the provided path is a file, not a directory
- **THEN** validation SHALL return an error containing "Not a directory"

#### Scenario: Missing SKILL.md
- **WHEN** the directory does not contain SKILL.md
- **THEN** validation SHALL return an error containing "Missing required file: SKILL.md"

