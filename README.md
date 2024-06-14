# Cypress Plugin

[![ci](https://github.com/fluentci-io/cypress-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/cypress-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with [Cypress](https://www.cypress.io/).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm cypress install
fluentci run --wasm cypress run
```

## Functions

| Name    | Description                                                     |
| ------- | --------------------------------------------------------------- |
| run     | Runs Cypress tests from the CLI without the GUI                 |
| install | Installs the Cypress executable matching this package's version |
| cache   | Manages the Cypress binary cache                                |
| info    | Prints Cypress and system information                           |
| verify  | Verifies that Cypress is installed correctly and executable     |
| help    | Shows CLI help and exits                                        |
| version | prints Cypress version                                          |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/cypress@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: cypress
    args: |
      help
- name: Show cypress version
  run: |
    fluentci run --wasm cypress version
```
