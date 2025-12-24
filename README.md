# skills-ref-rs

Rust implementation of [agentskills](https://github.com/agentskills/agentskills) for validating, parsing, and managing Agent Skills.

## Installation

```bash
cargo add skills-ref-rs
```

## CLI Usage

```bash
# Validate a skill directory
skills-ref-rs validate ./my-skill

# Read properties as JSON
skills-ref-rs read-properties ./my-skill

# Generate XML prompt block
skills-ref-rs to-prompt ./skill-a ./skill-b
```

## Library Usage

```rust
use skills_ref::{validate, read_properties, to_prompt};
use std::path::Path;

// Validate
let errors = validate(Path::new("my-skill"));
assert!(errors.is_empty());

// Read properties
let props = read_properties(Path::new("my-skill")).unwrap();
println!("{}: {}", props.name, props.description);

// Generate prompt
let xml = to_prompt(&[Path::new("my-skill")]).unwrap();
```

### Validation Rules

- `name`: required, lowercase kebab-case, max 64 chars, must match directory name
- `description`: required, max 1024 chars
- `compatibility`: optional, max 500 chars
- Unicode names supported (NFKC normalized)

## License

MIT
