## Context

This is a greenfield Rust implementation of an existing Python library. The Python reference implementation provides clear contracts for behavior, making this primarily a translation exercise with Rust-specific adaptations.

### Constraints

- Must maintain feature parity with Python `agentskills/skills-ref`
- Must follow Rust idioms (Result types, traits, ownership)
- CLI interface should match Python behavior (same commands, arguments, exit codes)
- Unicode handling must be equivalent (NFKC normalization)

## Goals / Non-Goals

### Goals
- Clean, idiomatic Rust code following `rustfmt` and `clippy` standards
- Library usable both as a crate and via CLI
- Comprehensive error messages matching Python behavior
- Full i18n support for skill names (Unicode letters, NFKC normalization)

### Non-Goals
- Async operations (not needed for file I/O at this scale)
- Plugin system or extensibility
- Configuration files beyond SKILL.md parsing

## Decisions

### Error Handling Strategy
**Decision**: Use `thiserror` derive macro for custom error enum.
**Rationale**: Provides ergonomic error creation with automatic `std::error::Error` implementation. Matches the Python exception hierarchy (SkillError -> ParseError, ValidationError).

```rust
#[derive(Debug, thiserror::Error)]
pub enum SkillError {
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Validation error: {message}")]
    Validation { message: String, errors: Vec<String> },
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

### YAML Parser Choice
**Decision**: Use `serde_yaml` for YAML parsing.
**Rationale**: Well-maintained, integrates seamlessly with serde ecosystem. The Python version uses `strictyaml` which is more restrictive, but `serde_yaml` covers all needed functionality.

### Struct Design for SkillProperties
**Decision**: Use a flat struct with `Option<T>` for optional fields.
**Rationale**: Matches Python dataclass structure directly. Serde handles JSON serialization with `skip_serializing_if` for cleaner output.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillProperties {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<String>,
    #[serde(rename = "allowed-tools", skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}
```

### CLI Framework
**Decision**: Use `clap` with derive macros.
**Rationale**: Standard choice for Rust CLIs, provides automatic help generation, type-safe argument parsing, and subcommand support.

### Unicode Normalization
**Decision**: Use `unicode-normalization` crate for NFKC.
**Rationale**: Matches Python's `unicodedata.normalize("NFKC", ...)` behavior exactly. Required for i18n skill names.

## Module Structure

```
src/
├── lib.rs          # Public API exports
├── main.rs         # CLI entry point  
├── error.rs        # SkillError enum
├── models.rs       # SkillProperties struct
├── parser.rs       # find_skill_md(), parse_frontmatter(), read_properties()
├── validator.rs    # validate(), validate_metadata(), field validators
└── prompt.rs       # to_prompt() XML generation
```

## Risks / Trade-offs

### Risk: YAML parser behavior differences
- **Mitigation**: Test with edge cases from Python test suite. `serde_yaml` is permissive but handles the YAML subset we need.

### Risk: Unicode edge cases
- **Mitigation**: Port all i18n tests from Python (Chinese, Russian, NFKC normalization). Use `unicode-normalization` crate.

### Trade-off: Strictness level
- Python uses `strictyaml` (very strict), Rust uses `serde_yaml` (permissive)
- Accept this difference; validate fields explicitly after parsing

## Migration Plan

Not applicable - greenfield implementation.

## Open Questions

None - Python reference implementation provides clear specification.
