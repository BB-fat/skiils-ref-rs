## 1. Project Setup
- [x] 1.1 Update `Cargo.toml` with required dependencies (clap, serde, serde_yaml, serde_json, thiserror, unicode-normalization)
- [x] 1.2 Verify project compiles with `cargo build`

## 2. Core Module Implementation
- [x] 2.1 Create `src/error.rs` with `SkillError` enum (Parse, Validation, Io variants)
- [x] 2.2 Create `src/models.rs` with `SkillProperties` struct and `to_dict()` method
- [x] 2.3 Update `src/lib.rs` to export public API

## 3. Parser Module
- [x] 3.1 Implement `find_skill_md()` function (prefer SKILL.md, accept skill.md)
- [x] 3.2 Implement `parse_frontmatter()` function (extract YAML between `---` delimiters)
- [x] 3.3 Implement `read_properties()` function (parse + basic field validation)
- [x] 3.4 Add unit tests for parser module

## 4. Validator Module
- [x] 4.1 Define validation constants (MAX_SKILL_NAME_LENGTH=64, MAX_DESCRIPTION_LENGTH=1024, MAX_COMPATIBILITY_LENGTH=500, ALLOWED_FIELDS)
- [x] 4.2 Implement `_validate_name()` (lowercase, kebab-case, no leading/trailing hyphens, no consecutive hyphens, NFKC normalization, directory match)
- [x] 4.3 Implement `_validate_description()` (non-empty, length limit)
- [x] 4.4 Implement `_validate_compatibility()` (optional, length limit)
- [x] 4.5 Implement `_validate_metadata_fields()` (reject unexpected fields)
- [x] 4.6 Implement `validate_metadata()` and `validate()` public functions
- [x] 4.7 Add unit tests for validator module (including i18n cases)

## 5. Prompt Module
- [x] 5.1 Implement `to_prompt()` function (generate `<available_skills>` XML block)
- [x] 5.2 Add HTML escaping for name and description
- [x] 5.3 Add unit tests for prompt module

## 6. CLI Implementation
- [x] 6.1 Create `src/main.rs` with clap App structure
- [x] 6.2 Implement `validate` subcommand (return exit code 0/1)
- [x] 6.3 Implement `read-properties` subcommand (JSON output)
- [x] 6.4 Implement `to-prompt` subcommand (multiple skill paths, XML output)
- [x] 6.5 Add helper for SKILL.md file path handling (accept file or directory)

## 7. Integration Testing
- [x] 7.1 Add integration tests using `example/` skill directory
- [x] 7.2 Test CLI commands with `assert_cmd` or similar
- [x] 7.3 Verify output matches Python implementation format

## 8. Final Validation
- [x] 8.1 Run `cargo fmt` and `cargo clippy`
- [x] 8.2 Run full test suite with `cargo test`
- [x] 8.3 Test CLI manually against `example/` directory
- [x] 8.4 Compare output with Python `skills-ref` for consistency
