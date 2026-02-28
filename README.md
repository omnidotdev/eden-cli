<div align="center">
  <img src="/assets/logo.png" width="100" />

  <h1 align="center">Eden</h1>

[Website](https://eden.omni.dev) | [Docs](https://docs.omni.dev/armory/eden) | [Feedback](https://backfeed.omni.dev/workspaces/omni/projects/eden) | [Discord](https://discord.gg/omnidotdev) | [X](https://x.com/omnidotdev)

</div>

**Eden** is a Rust-powered CLI tool that verifies required dependencies and configurations are properly installed before development begins. Run one command to ensure your dev environment is ready.

## Installation

| Platform | Channel | Command / Link |
| --- | --- | --- |
| All | [GitHub Releases](https://github.com/omnidotdev/eden-cli/releases) | Download from releases page |
| All | [crates.io](https://crates.io/crates/eden-cli) | `cargo install eden-cli` |
| macOS / Linux | [Homebrew](https://github.com/omnidotdev/homebrew-tap/blob/master/Formula/eden.rb) | `brew install omnidotdev/tap/eden` |
| Arch Linux | [AUR](https://aur.archlinux.org/packages/omnidotdev-eden) / [AUR (bin)](https://aur.archlinux.org/packages/omnidotdev-eden-bin) | `paru -S omnidotdev-eden` or `paru -S omnidotdev-eden-bin` |

### Build from source

```bash
git clone https://github.com/omnidotdev/eden-cli
cd eden-cli
cargo build --release
# Binary will be at target/release/eden
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

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `test.yml` | Push/PR to `master` | Runs fmt, clippy, and tests |
| `sync.yml` | PR to `master` | Validates version sync, fmt, clippy, test, build |
| `release.yml` | Push to `master` | Creates releases via Changesets, builds multi-platform binaries |

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
5. **Manually** publish to crates.io: `cargo publish`

## Ecosystem

- **[Omni CLI](https://github.com/omnidotdev/cli)**: Agentic CLI for the Omni ecosystem
- **[Omni Terminal](https://github.com/omnidotdev/terminal)**: GPU-accelerated terminal emulator built to run everywhere

## License

The code in this repository is licensed under MIT, &copy; [Omni LLC](https://omni.dev). See [LICENSE.md](LICENSE.md) for more information.
