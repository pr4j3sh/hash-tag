# hash-tag

> <#>

Markdown to HTML Parser written in `rust`, built with wasm, for direct usage in `javascript`.

### Features

- Parses headings, blockquotes, inline code, codeblock, paragraphs, links, bold, italics, unordered lists and ordered list.
- Visualize outputs using frames ui.

## Usage

### Using `cargo`

```bash
cargo install hash-tag
```

- Use `run`

```bash
hash-tag path/to/file.md
```

> Generates a `index.html` file

- Specify output file using `-o` flag

```bash
hash-tag path/to/file.md -o path/to/file.html
```

- Visualize output file using `-v` flag

```bash
hash-tag path/to/file.md -v path/to/view.html
```

> This uses [frames ui](https://pr4j3sh.github.io/ui/).

### Using `npm`

- Create a node environment

```bash
mkdir test
cd test
npm init -y
```

- Add `"type": "module",` to `package.json` file.

- Install [@pr4j3sh/hash-tag](https://www.npmjs.com/package/@pr4j3sh/hash-tag) npm package

```bash
npm install @pr4j3sh/hash-tag
```

- Create a new file `index.js` and write the code as:

```js
import * as wasm from "@pr4j3sh/hash-tag";

const html = wasm.parse("## heading 2\n");
console.log(html);
```

- Run using

```bash
node index.js
```

- Outputs

```html
<h2>heading 2</h2>
```

## References

- [Rust Documentation](https://www.rust-lang.org/learn/get-started)
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
- [wasm-pack](https://github.com/rustwasm/wasm-pack)
- [frames ui](https://pr4j3sh.github.io/ui/)
- [@pr4j3sh/frames](https://github.com/pr4j3sh/frames)
