# Change: Implement skills-ref-rs Rust Library

## Why

The project requires a Rust implementation of the Python `agentskills` library to provide both a CLI tool and library for validating, parsing, and managing Agent Skills. Rust offers memory safety, better performance, and strong typing while maintaining feature parity with the Python reference implementation.

## What Changes

- **NEW**: Create `error.rs` - Custom error types using `thiserror`
- **NEW**: Create `models.rs` - `SkillProperties` struct with serialization
- **NEW**: Create `parser.rs` - YAML frontmatter parsing and `read_properties()`
- **NEW**: Create `validator.rs` - Validation rules (name format, field limits, directory match)
- **NEW**: Create `prompt.rs` - XML `<available_skills>` block generation
- **NEW**: Create `main.rs` - CLI with `validate`, `read-properties`, `to-prompt` commands
- **MODIFY**: Update `lib.rs` - Export public API
- **MODIFY**: Update `Cargo.toml` - Add dependencies (clap, serde, serde_yaml, serde_json, thiserror, unicode-normalization)

## Impact

- Affected specs: skill-parsing, skill-validation, prompt-generation, cli
- Affected code: `src/*.rs`, `Cargo.toml`
- Dependencies: External crates for CLI, serialization, YAML parsing
- Test fixtures: `example/` directory contains reference SKILL.md

## Success Criteria

1. All three CLI commands (`validate`, `read-properties`, `to-prompt`) work correctly
2. Validation rules match Python implementation behavior (name format, length limits, i18n support)
3. JSON output from `read-properties` matches Python format
4. XML output from `to-prompt` matches Python format
5. Unit tests cover core validation logic
6. Integration tests use `example/` skill directory
