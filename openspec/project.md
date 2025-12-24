# Project Context

## Purpose

**skills-ref-rs** is a Rust implementation of the [agentskills](https://github.com/agentskills/agentskills) Python library. It provides both a CLI tool and a library for validating, parsing, and managing "Agent Skills" - structured markdown documents that describe capabilities available to AI assistants.

The project aims for full feature parity with the Python version while leveraging Rust's strengths (memory safety, performance, type system) and following Rust idioms.

### Core Functionality

1. **Validate** - Check that a skill directory contains a valid SKILL.md with proper YAML frontmatter
2. **Read Properties** - Parse SKILL.md frontmatter and extract metadata as JSON
3. **To Prompt** - Generate XML blocks for embedding skill information in agent system prompts

## Tech Stack

- **Language**: Rust (Edition 2024)
- **CLI Framework**: [clap](https://docs.rs/clap) - Command-line argument parsing with derive macros
- **Serialization**: [serde](https://docs.rs/serde) + [serde_json](https://docs.rs/serde_json) - JSON serialization/deserialization
- **YAML Parsing**: [serde_yaml](https://docs.rs/serde_yaml) or similar for frontmatter parsing
- **Async Runtime**: [tokio](https://docs.rs/tokio) (if async operations are needed)
- **Error Handling**: [thiserror](https://docs.rs/thiserror) for custom error types

## Project Conventions

### Code Style

- Follow standard Rust conventions enforced by `rustfmt`
- Use `clippy` for linting with default warnings enabled
- Naming conventions:
  - `snake_case` for functions, variables, and modules
  - `CamelCase` for types, traits, and enums
  - `SCREAMING_SNAKE_CASE` for constants

### Project Structure

```
skills-ref-rs/
├── Cargo.toml              # Package manifest
├── src/
│   ├── lib.rs              # Library root - exports public API
│   ├── main.rs             # CLI entry point (optional, or use bin/)
│   ├── models.rs           # SkillProperties and related data structures
│   ├── parser.rs           # YAML frontmatter parsing logic
│   ├── validator.rs        # Validation rules and constraints
│   ├── prompt.rs           # XML prompt generation
│   └── error.rs            # Custom error types
├── example/                # Reference skill for testing
│   └── SKILL.md
├── agentskills/            # Python reference implementation
└── openspec/               # Project specifications
```

### Architecture Patterns

- **Error Handling**: Use `Result<T, E>` with custom error enums. Avoid panics in library code.
- **Builder Pattern**: Consider for complex struct construction (e.g., SkillProperties)
- **Separation of Concerns**: Keep parsing, validation, and output generation in separate modules
- **Public API**: Expose a clean, minimal public interface from `lib.rs`

### Testing Strategy

- Unit tests in the same file as the code being tested (using `#[cfg(test)]` modules)
- Integration tests in `tests/` directory
- Use the `example/` skill directory as the primary test fixture
- Run tests with `cargo test`
- Aim for high coverage on validation logic

### Documentation

- Document all public items with `///` doc comments
- Include examples in doc comments where helpful
- Generate docs with `cargo doc --open`

## Domain Context

### Agent Skills

An Agent Skill is a markdown document (`SKILL.md`) that describes a capability available to an AI assistant. It consists of:

1. **YAML Frontmatter** - Metadata between `---` delimiters:
   - `name` (required): Lowercase kebab-case identifier (max 64 chars)
   - `description` (required): What the skill does (max 1024 chars)
   - `license` (optional): License information
   - `compatibility` (optional): Model/tool compatibility notes (max 500 chars)
   - `allowed-tools` (optional): List of tool patterns
   - `metadata` (optional): Custom key-value pairs

2. **Markdown Body** - Detailed instructions, guides, and reference documentation

### Validation Rules

- Skill name must match the parent directory name
- Names must be lowercase, kebab-case, alphanumeric with hyphens only
- Unicode is supported (NFKC normalized)
- Field length limits are strictly enforced

### Output Formats

- **JSON**: For `read-properties` command output
- **XML**: For `to-prompt` command, generates `<available_skills>` blocks for system prompts

## Reference Implementation

The Python reference implementation is in `agentskills/skills-ref/`. Key modules to reference:

| Python Module | Purpose |
|---------------|---------|
| `models.py` | `SkillProperties` dataclass |
| `parser.py` | Frontmatter parsing, `find_skill_md()`, `read_properties()` |
| `validator.py` | Validation logic and constraints |
| `prompt.py` | XML generation |
| `cli.py` | Click-based CLI |
| `errors.py` | Exception hierarchy |
