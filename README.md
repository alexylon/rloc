# rcount

Count Rust lines of code in a project. Recursively finds all `.rs` files, skipping `target/` and `.git/`.

Reports two totals:
- **Pure Rust code** — excludes tests, lint attributes, and comments
- **All Rust code** — all non-blank, non-comment Rust lines

## Usage

```bash
# Count lines in current directory
rcount

# Count lines in another project
rcount path/to/project
```
