# Known issues

## Top-level `{{x:y}}` function node discards preceding sibling nodes

When a top-level **function** node (`{{name:arg}}`, parsed as `Node::Function`)
is completed, the parser discards the nodes accumulated *before* it on the
preceding lines instead of appending. Templates (`{{Foo|a}}`) and parameters
(`{{{x}}}`) are unaffected — only function nodes trigger it.

Observed on commit `37ea74a` (branch `cargo-update`).

### Minimal repro

```rust
let cfg = /* a normal fr/en Configuration */;
assert_eq!(cfg.parse("Hello '''world'''. End.").unwrap().nodes.len(), 5); // ok
assert_eq!(cfg.parse("Hello world.\n{{Foo|a}}").unwrap().nodes.len(), 2);  // ok (Template)
assert_eq!(cfg.parse("Hello world.\n{{lc:X}}").unwrap().nodes.len(), 2);   // FAILS: returns 1
assert_eq!(cfg.parse("a\n{{DEFAULTSORT:X}}\nb").unwrap().nodes.len(), 4);  // FAILS: returns 2, "a" dropped
```

`"a\n{{DEFAULTSORT:X}}\nb"` returns 2 nodes and the leading `a` is gone;
content *after* the function survives. On a real article that ends with
`{{DEFAULTSORT:…}}` + categories, the parse collapses to the ~3 tail nodes and
the whole body is dropped.

### Expected

A function node should be appended to the sibling list like a template node;
preceding nodes must be preserved.

### Impact / workaround

Feeding raw wikitext (with functions like `{{DEFAULTSORT:…}}`, `{{#if:…}}`,
etc.) straight to `parse()` loses page bodies. Consumers can work around it by
running a brace-matching preprocessor first, so `parse()` only ever sees
function-free text (this is what `pedia`'s `explode` engine does).
