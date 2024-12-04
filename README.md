# hash-tag

> <#>

Markdown to HTML Parser written in rust.

### Features

- Parses headings, blockquotes, inline code, codeblock, paragraphs, links, bold, italics, unordered lists and ordered list.
- Has debug mode for showing outputs of markdown parsing.
- Visualize outputs using frames ui.

## Installation

- Clone the repository

```bash
git clone https://github.com/pr4j3sh/hash-tag.git
cd hash-tag
```

## Usage

- Use `run`

```bash
cargo run path/to/file.md
```

> Generates a `index.html` file

- Specify output file using `-o` flag

```bash
cargo run path/to/file.md -o path/to/file.html
```

- Visualize output file using `-v` flag

```bash
cargo run path/to/file.md -v path/to/view.html
```

> This uses [frames ui](https://pr4j3sh.github.io/ui/).

- Use `debug` mode using `-d` flag

```bash
cargo run path/to/file.md -d
```

## References

- [Rust Documentation](https://www.rust-lang.org/learn/get-started)
- [frames ui](https://pr4j3sh.github.io/ui/)
- [@pr4j3sh/frames](https://github.com/pr4j3sh/frames)
