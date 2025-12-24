# skill-parsing Specification

## Purpose
TBD - created by archiving change implement-skills-ref-rs. Update Purpose after archive.
## Requirements
### Requirement: SKILL.md File Discovery
The system SHALL locate the skill definition file within a skill directory.

#### Scenario: Uppercase SKILL.md preferred
- **WHEN** a directory contains both `SKILL.md` and `skill.md`
- **THEN** the system SHALL return the path to `SKILL.md`

#### Scenario: Lowercase skill.md accepted
- **WHEN** a directory contains only `skill.md` (lowercase)
- **THEN** the system SHALL return the path to `skill.md`

#### Scenario: No skill file found
- **WHEN** a directory contains neither `SKILL.md` nor `skill.md`
- **THEN** the system SHALL return `None`

### Requirement: YAML Frontmatter Parsing
The system SHALL parse YAML frontmatter from SKILL.md content.

#### Scenario: Valid frontmatter extracted
- **WHEN** content starts with `---` and contains a closing `---`
- **THEN** the system SHALL return the parsed YAML as a dictionary and the markdown body

#### Scenario: Missing frontmatter start
- **WHEN** content does not start with `---`
- **THEN** the system SHALL return a ParseError

#### Scenario: Unclosed frontmatter
- **WHEN** content starts with `---` but has no closing `---`
- **THEN** the system SHALL return a ParseError

#### Scenario: Invalid YAML syntax
- **WHEN** the content between `---` delimiters is not valid YAML
- **THEN** the system SHALL return a ParseError

### Requirement: Skill Properties Reading
The system SHALL read and return skill properties from a skill directory.

#### Scenario: Successful properties read
- **WHEN** a valid skill directory is provided
- **THEN** the system SHALL return a `SkillProperties` struct with name, description, and optional fields

#### Scenario: Missing SKILL.md
- **WHEN** the skill directory has no SKILL.md file
- **THEN** the system SHALL return a ParseError

#### Scenario: Missing required name field
- **WHEN** the frontmatter does not contain a `name` field
- **THEN** the system SHALL return a ValidationError

#### Scenario: Missing required description field
- **WHEN** the frontmatter does not contain a `description` field
- **THEN** the system SHALL return a ValidationError

#### Scenario: Empty name value
- **WHEN** the `name` field is empty or whitespace-only
- **THEN** the system SHALL return a ValidationError

#### Scenario: Metadata field conversion
- **WHEN** the frontmatter contains a `metadata` field with key-value pairs
- **THEN** the system SHALL convert all keys and values to strings

