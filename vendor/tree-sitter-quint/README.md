# tree-sitter-quint

A [Tree-sitter](https://tree-sitter.github.io) grammar for the
[Quint](https://quint-lang.org) specification language.

## Scope

This is a **shallow** grammar: it tokenizes Quint source into the categories
needed for syntax highlighting (keywords, storage modifiers, builtin types,
constants, operators, strings, numbers, comments, identifiers) but does not
build a full expression or type tree.

It mirrors the categories of the official
[VS Code TextMate grammar](https://github.com/informalsystems/quint/blob/main/vscode/quint-vscode/syntaxes/quint.tmLanguage.json),
plus a few additions the TextMate grammar misses:

- The `match` keyword
- The `Map` builtin type
- The `->` operator
- Prime-suffixed identifiers (`x'`)

If you need a deeper grammar (folding, outline, refactoring), this can be
extended without breaking the highlight queries — the named leaf nodes
(`keyword`, `storage_modifier`, etc.) are stable.

## Status

Verified against 255 real-world `.qnt` files (zero parse errors). If you
find a Quint construct that triggers errors, please open an issue with a
minimal example.

## Captures

The grammar emits these named nodes for highlight queries:

| Node               | Matches                                                        |
| ------------------ | -------------------------------------------------------------- |
| `keyword`          | `module import from export as if else match not or and implies iff all any leadsTo` |
| `storage_modifier` | `type assume const var val nondet def pure action temporal run` |
| `storage_type`     | `Tup Rec Set List Map int str bool`                            |
| `constant`         | `false true Bool Int Nat`                                      |
| `operator`         | `=> -> <= >= != == < > = + - * / % ^ .`                        |
| `string`           | `"..."` (with `escape_sequence` children for `\.`)             |
| `number`           | decimal and hex integer literals (with `_` separators)         |
| `line_comment`     | `// ...`                                                       |
| `block_comment`    | `/* ... */`                                                    |
| `hashbang`         | `#!...` at start of file                                       |
| `identifier`       | everything else                                                |

A canonical [`queries/highlights.scm`](queries/highlights.scm) is included.

## Used by

- [`zed-quint`](https://github.com/zdavison/zed-quint) — Quint syntax
  highlighting for the [Zed](https://zed.dev) editor.

## Building

```sh
npm install
npx tree-sitter generate
npx tree-sitter test
npx tree-sitter parse path/to/example.qnt
```

## License

Apache-2.0.
