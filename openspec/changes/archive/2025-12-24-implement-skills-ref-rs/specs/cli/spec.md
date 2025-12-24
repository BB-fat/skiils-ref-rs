## ADDED Requirements

### Requirement: Validate Command
The CLI SHALL provide a `validate` subcommand to check skill directories.

#### Scenario: Valid skill returns exit code 0
- **WHEN** `skills-ref-rs validate <path>` is run on a valid skill directory
- **THEN** the CLI SHALL print "Valid skill: <path>" to stdout
- **AND** exit with code 0

#### Scenario: Invalid skill returns exit code 1
- **WHEN** `skills-ref-rs validate <path>` is run on an invalid skill directory
- **THEN** the CLI SHALL print "Validation failed for <path>:" to stderr
- **AND** print each error message prefixed with "  - " to stderr
- **AND** exit with code 1

#### Scenario: Accept SKILL.md file path
- **WHEN** the path points directly to a SKILL.md file
- **THEN** the CLI SHALL validate the parent directory

### Requirement: Read Properties Command
The CLI SHALL provide a `read-properties` subcommand to extract skill metadata.

#### Scenario: Successful properties read
- **WHEN** `skills-ref-rs read-properties <path>` is run on a valid skill directory
- **THEN** the CLI SHALL print the properties as formatted JSON to stdout
- **AND** exit with code 0

#### Scenario: Parse error returns exit code 1
- **WHEN** `skills-ref-rs read-properties <path>` fails to parse
- **THEN** the CLI SHALL print "Error: <message>" to stderr
- **AND** exit with code 1

#### Scenario: JSON output format
- **WHEN** outputting properties
- **THEN** the JSON SHALL include `name` and `description` fields
- **AND** optional fields (license, compatibility, allowed-tools, metadata) SHALL only appear if present
- **AND** the `allowed-tools` field SHALL use the hyphenated key name

### Requirement: To Prompt Command
The CLI SHALL provide a `to-prompt` subcommand to generate XML for agent prompts.

#### Scenario: Single skill path
- **WHEN** `skills-ref-rs to-prompt <path>` is run
- **THEN** the CLI SHALL print the `<available_skills>` XML block to stdout

#### Scenario: Multiple skill paths
- **WHEN** `skills-ref-rs to-prompt <path1> <path2> ...` is run
- **THEN** the CLI SHALL generate XML with all skills in order

#### Scenario: At least one path required
- **WHEN** `skills-ref-rs to-prompt` is run with no paths
- **THEN** the CLI SHALL show an error about missing required arguments

#### Scenario: Error returns exit code 1
- **WHEN** any skill fails to parse
- **THEN** the CLI SHALL print "Error: <message>" to stderr
- **AND** exit with code 1

### Requirement: Version and Help
The CLI SHALL provide standard --version and --help options.

#### Scenario: Version flag
- **WHEN** `skills-ref-rs --version` is run
- **THEN** the CLI SHALL print the version number

#### Scenario: Help flag
- **WHEN** `skills-ref-rs --help` is run
- **THEN** the CLI SHALL print usage information including all subcommands
