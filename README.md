# 🌿 Eden

Developer onboarding preflight checks.

Eden is a Rust-powered CLI tool that verifies required dependencies and configurations are properly installed before development begins. Run one command to ensure your dev environment is ready.

## Installation

```sh
cargo install eden-cli
```

## Quick Start

1. Initialize a config file in your project root:

```sh
eden init
```

Or create an `eden.toml` manually:

```toml
[checks]
binaries = ["docker", "node", "bun"]
environment = ["DATABASE_URL"]
```

2. Run the preflight check:

```sh
eden check
```

Example output:

```
🌱 Binary: docker - v29.1.3 (/usr/bin/docker)
🌱 Binary: node - v25.2.1 (/usr/bin/node)
🌱 Binary: bun - v1.3.5(/usr/bin/bun)
🥀 Env: DATABASE_URL - not set

🌱 3 sprouted, 🥀 1 needs water
```

## Configuration

Eden supports multiple config formats:

- TOML: `eden.toml`
- YAML: `eden.yaml` / `eden.yml`
- JSON: `eden.json` / `eden.jsonc`

Generate a starter config:

```sh
eden init
eden init --format yaml
```

If no `--format` is specified, Eden will default to TOML.

### Config Options

```toml
[checks]
# binaries that must be in PATH
binaries = [
    "git",
    "docker",
    "node",
]

# environment variables that must be set
environment = [
    "DATABASE_URL",
    "API_KEY",
]
```

## CLI Reference

```sh
eden check                 # run all checks (default)
eden check -c custom.toml  # use custom config file
eden init                  # create starter config
eden --help                # show help
eden --version             # show version
```

## Use Cases

- **New Developer Onboarding**: Include in your README as the first setup step
- **CI/CD Pipelines**: Verify environments before running tests
- **Pre-commit Hooks**: Ensure environment consistency
- **Documentation**: Self-documenting project requirements

## Roadmap

- [ ] Version constraints for binaries (`node >= 18`)
- [ ] Interactive fix mode (`eden fix`)
- [ ] Network connectivity checks
- [ ] Disk space checks
- [ ] Custom script validators
- [ ] Host JSON schema on the internet

## Development

### Version Syncing

Eden uses a dual-package setup (Rust crate + npm package) with automated version synchronization:

- **Source of truth**: `package.json` holds the canonical version, and is used for Changesets
- **Sync script**: `scripts/syncVersion.ts` propagates the version to `Cargo.toml`
- **Changesets**: Manages version bumps and changelog generation

The sync script runs automatically during the release process via the `version` npm script:

```sh
bun run version  # syncs `package.json` version → `Cargo.toml`
```

### CI/CD

Two GitHub workflows handle versioning:

| Workflow      | Trigger             | Purpose                                                         |
| ------------- | ------------------- | --------------------------------------------------------------- |
| `sync.yml`    | Push/PR to `master` | Validates versions match, runs tests, builds artifacts          |
| `release.yml` | Push to `master`    | Creates releases via Changesets, builds multi-platform binaries |

The sync workflow will fail if `package.json` and `Cargo.toml` versions diverge.

### Release Process

1. Create a changeset: `bun changeset`
2. Push to `master`
3. Changesets action creates a "Version Packages" PR
4. Merge the PR to trigger a release with binaries for:
   - `x86_64-unknown-linux-gnu`
   - `aarch64-unknown-linux-gnu`
   - `x86_64-apple-darwin`
   - `aarch64-apple-darwin`

## License

The code in this repository is licensed under MIT, &copy; Omni LLC. See [LICENSE.md](LICENSE.md) for more information.
