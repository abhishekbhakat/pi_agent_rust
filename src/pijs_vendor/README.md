# PiJS vendored packages

Unmodified ESM builds embedded as PiJS virtual modules (see
`default_virtual_modules` in `src/extensions_js.rs`). PiJS never reads
`node_modules`, so bare package imports only resolve to modules registered
there.

| Module   | Version | Source file                      | License            |
|----------|---------|----------------------------------|--------------------|
| `marked` | 18.0.14 | `marked/lib/marked.esm.js`       | MIT (`marked.LICENSE`)    |
| `shlex`  | 3.0.0   | `shlex/shlex.js`                 | MIT (`shlex.LICENSE`) |

`marked.esm.js` has only its trailing `sourceMappingURL` comment removed.
